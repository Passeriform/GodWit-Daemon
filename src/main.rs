use godwit_daemon::core::ExternalBackends;
use godwit_daemon::core::Ops;
use godwit_daemon::runner::{self, Regress};
use simplelog::*;
use structopt::{clap::Shell, StructOpt};

use cargo_clone;
use std::error::Error;
use std::fs;
use std::process::Command;
use toml;
use toml::map::Map;
use toml::Value;

#[macro_use]
extern crate lazy_static;

fn get_dylib_deps() -> Result<Map<String, Value>, Box<dyn Error>> {
	let parsed_cargo_toml: Value = toml::from_str(&fs::read_to_string("Cargo.toml")?)?;

	parsed_cargo_toml["runtime-cdylib-dependencies"]
		.as_table()
		.map_or_else(
			|| Err("No array could be parsed.".into()),
			|val| Ok(val.clone()),
		)
}

lazy_static! {
	static ref EXT_BACKENDS: Vec<String> = {
		let mut end_vec = Vec::new();

		for (dependency, version) in get_dylib_deps()
			.expect("Dependencies couldn't be fetched.")
			.into_iter()
		{
			cargo_clone::clone("crate", &dependency, version.as_str(), &[]);
		}

		for key in ExternalBackends::from_dir("lib/backends")
			.unwrap()
			.backends
			.keys()
		{
			end_vec.push(key.to_string());
		}

		end_vec
	};
}

// Define Config
#[derive(Debug, StructOpt)]
#[structopt(name = "Godwit-Daemon", about = "Runner daemon for GodWit.")]
struct DaemonArgs {
	/// Silence all output
	#[structopt(short, long, global = true, conflicts_with = "verbose")]
	quiet: bool,

	/// Debug mode
	#[structopt(
		short,
		long,
		global = true,
		conflicts_with = "quiet",
		parse(from_occurrences)
	)]
	verbose: u64,

	/// Operation Variant
	#[structopt(subcommand)]
	ops_variant: Option<OpsVariantEnum>,
}

/// Daemon operation variants
#[derive(Debug, StructOpt)]
#[structopt(about = "Operation variant selection")]
enum OpsVariantEnum {
	/// Start new task and schedule
	New {
		/// Operation
		#[structopt(subcommand)]
		operation: OpsEnum,

		/// Discard all previous diffs
		#[structopt(short, long, global = true)]
		refresh: bool,
	},
	/// Regress operation until killsignal sent
	Regress {
		/// Operation
		#[structopt(subcommand)]
		operation: OpsEnum,

		/// Discard all previous diffs
		#[structopt(short, long, global = true)]
		refresh: bool,
	},
	/// Send killsignal
	Die {
		/// Operation
		#[structopt(subcommand)]
		operation: OpsEnum,
	},
}

///Daemon operations
#[derive(Debug, StructOpt)]
#[structopt(about = "Operation selection")]
enum OpsEnum {
	/// Trace an application state
	Trace {
		/// Application name
		#[structopt(possible_values = &EXT_BACKENDS
			.iter()
			.map(|b| b.as_ref())
			.collect::<Vec<&str>>()
			.as_slice(), case_insensitive = true)]
		application: String,
	},
}

fn get_log_level(quiet: bool, verbosity: u64) -> LevelFilter {
	if quiet {
		return LevelFilter::Off;
	} else if verbosity == 0 || verbosity == 1 {
		return LevelFilter::Warn;
	} else if verbosity == 2 {
		return LevelFilter::Info;
	} else if verbosity == 3 {
		return LevelFilter::Debug;
	} else {
		return LevelFilter::Trace;
	}
}

fn main() {
	DaemonArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "completions");
	DaemonArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, "completions");
	DaemonArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::PowerShell, "completions");
	DaemonArgs::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, "completions");

	let config = DaemonArgs::from_args();

	// Logging globals
	let (verbosity, quiet) = (config.verbose, config.quiet);

	// Logger setup
	CombinedLogger::init(vec![TermLogger::new(
		get_log_level(quiet, verbosity),
		Config::default(),
		TerminalMode::Mixed,
	)
	.unwrap()])
	.unwrap();

	// TODO: Improve this match hell.
	// TODO: Add listing threads.
	let result = match config.ops_variant {
		Some(OpsVariantEnum::New { operation, refresh }) => match operation {
			OpsEnum::Trace { application } => {
				runner::run(Ops::Trace, &application, refresh, Regress::Once)
			}
		},
		Some(OpsVariantEnum::Regress { operation, refresh }) => match operation {
			OpsEnum::Trace { application } => {
				runner::run(Ops::Trace, &application, refresh, Regress::Infinite)
			}
		},
		Some(OpsVariantEnum::Die { operation }) => match operation {
			OpsEnum::Trace { application } => runner::kill(Ops::Trace, &application),
		},
		_ => runner::init_daemon().map_err(Into::into),
	};
}
