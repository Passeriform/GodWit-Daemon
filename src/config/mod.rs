mod internal;

use crate::errors::{PatchError, PurgeBaseError, RevisionError};
use chrono::prelude::*;
use getter_derive::Getter;
use glob::glob;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

/// Define godwit daemon config.
#[derive(Debug, Deserialize, Serialize, Getter)]
#[serde(rename_all = "snake_case")]
pub struct Config {
	pub daemon_directory: PathBuf,
	pub max_threads: usize,
	pub daemon_url: String,
	pub client_url: String,
}

impl Config {
	/// Returns new settings instance.
	pub fn init(
		daemon_directory: Option<PathBuf>,
		max_threads: Option<usize>,
		tcp_port: Option<String>,
	) -> Self {
		Config {
			daemon_directory: daemon_directory
				.unwrap_or_else(|| Config::default().daemon_directory),
			max_threads: max_threads.unwrap_or_else(|| Config::default().max_threads),
			daemon_url: tcp_port
				.clone()
				.unwrap_or_else(|| Config::default().daemon_url),
			client_url: tcp_port
				.clone()
				.unwrap_or_else(|| Config::default().client_url),
		}
	}

	pub fn get_base_path(&self, application: &str, file_name: &str) -> Option<PathBuf> {
		let base_path = self
			.daemon_directory
			.as_path()
			.join(application)
			.join(format!("{}.base", &file_name));

		return if base_path.exists() {
			Some(base_path)
		} else {
			None
		};
	}

	/// Get patched revision for file of an application
	pub fn get_patched_rev(
		&self,
		application: &str,
		file_name: &str,
	) -> Result<Value, RevisionError> {
		let base_path = self
			.daemon_directory
			.as_path()
			.join(application)
			.join(format!("{}.base", &file_name));

		if !base_path.exists() {
			return Err(RevisionError::BaseNotFound {
				base_path: file_name.to_string(),
			});
		}

		let base = fs::read_to_string(&base_path)?;

		let patched = glob(&format!(
			"{}/*.patch",
			&self
				.daemon_directory
				.as_path()
				.join(application.to_string())
				.join("patches")
				.to_string_lossy(),
		))?
		.into_iter()
		.try_fold(base, |acc, patch_path| {
			let patch_file = fs::read_to_string(patch_path?)?;

			internal::patch(acc, patch_file)
		})?;

		Ok(serde_json::from_str(&patched)?)
	}

	// Save new patch files
	pub fn save_patch_file<T>(
		&self,
		application: &str,
		file_name: &str,
		patch_content: T,
	) -> Result<(), PatchError>
	where
		T: Display + Serialize,
	{
		if self.get_base_path(application, file_name).is_none() {
			let patch_path = self
				.daemon_directory
				.as_path()
				.join(application.to_string())
				.join(format!("{}.base", &file_name));

			fs::create_dir_all(patch_path.parent().unwrap())?;
			let patch_file = File::create(patch_path)?;

			serde_json::to_writer_pretty(patch_file, &patch_content)?;
		} else {
			let patch_path = self
				.daemon_directory
				.as_path()
				.join(application.to_string())
				.join("patches")
				.join(&file_name)
				.join(format!("{}-{}.patch", &file_name, Utc::now()));

			fs::create_dir_all(patch_path.parent().unwrap())?;
			let mut patch_file = File::create(patch_path)?;

			patch_file.write_all(patch_content.to_string().as_bytes())?;
		};
		Ok(())
	}
}

impl Default for Config {
	fn default() -> Self {
		Config {
			daemon_directory: dirs::home_dir()
				.expect("Home couldn't be located in current $PATH variables.")
				.join(".godwit")
				.join("daemon"),
			max_threads: 23,
			daemon_url: "tcp://*:5555".to_string(),
			client_url: "tcp://127.0.0.1:5555".to_string(),
		}
	}
}

/// Get settings instance from settings source file.
pub fn get_config() -> Result<Config, io::Error> {
	let config_rc_path: PathBuf = dirs::home_dir()
		.expect("Home couldn't be located in current $PATH variables.")
		.join(".gdrc")
		.iter()
		.collect();

	let config_path: PathBuf = dirs::home_dir()
		.expect("Home couldn't be located in current $PATH variables.")
		.join(".godwit")
		.join("daemon.gwcore")
		.iter()
		.collect();

	let config = if config_rc_path.exists() {
		info!("Daemon config found at {}", config_rc_path.display());
		File::open(config_rc_path).and_then(|config_file| {
			let config: Config = serde_json::from_reader(config_file)?;
			Ok(config)
		})?
	} else if config_path.exists() {
		info!("Daemon config found at {}", config_path.display());
		File::open(config_path).and_then(|config_file| {
			let config: Config = serde_json::from_reader(config_file)?;
			Ok(config)
		})?
	} else {
		Config::default()
	};

	Ok(config)
}

pub fn update_patches(
	application: &str,
	file_path: &Path,
	curr_instance: Value,
) -> Result<(), RevisionError> {
	let config = get_config()?;

	let file_name = file_path
		.file_name()
		.unwrap()
		.to_string_lossy()
		.into_owned();

	if config.get_base_path(application, &file_name).is_none() {
		return config
			.save_patch_file(application, &file_name, curr_instance)
			.map_err(Into::into);
	}

	let prev_instance = config.get_patched_rev(application, &file_name)?;

	let new_patch = internal::diff(prev_instance, curr_instance)?;

	config.save_patch_file(application, &file_name, new_patch)?;
	Ok(())
}

pub fn purge_base_file(application: &str, file_path: &Path) -> Result<(), PurgeBaseError> {
	let file_name = file_path
		.file_name()
		.unwrap_or_default()
		.to_string_lossy()
		.into_owned();

	if let Some(base_path) = get_config()?.get_base_path(application, &file_name) {
		fs::remove_dir_all(base_path)?;
	}
	Ok(())
}
