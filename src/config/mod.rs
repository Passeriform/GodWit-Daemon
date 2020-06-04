use getter_derive_rs::Getter;
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{fs::File, path::PathBuf};

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
}

impl Default for Config {
	fn default() -> Self {
		Config {
			daemon_directory: dirs::home_dir()
				.expect("Home couldn't be located in current $PATH variables.")
				.join(".godwit")
				.join("daemon.gwcore"),
			max_threads: 23,
			daemon_url: "tcp://*:5555".to_string(),
			client_url: "tcp://127.0.0.1:5555".to_string(),
		}
	}
}

/// Get settings instance from settings source file.
pub fn get_config() -> Result<Config, Box<dyn Error>> {
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

	if config_rc_path.exists() {
		info!("Daemon config found at {}", config_rc_path.display());
		let config = File::open(config_rc_path).and_then(|config_file| {
			let config: Config = serde_json::from_reader(config_file)?;
			Ok(config)
		})?;
		Ok(config)
	} else if config_path.exists() {
		info!("Daemon config found at {}", config_path.display());
		let config = File::open(config_path).and_then(|config_file| {
			let config: Config = serde_json::from_reader(config_file)?;
			Ok(config)
		})?;
		Ok(config)
	} else {
		Ok(Config::default())
	}
}
