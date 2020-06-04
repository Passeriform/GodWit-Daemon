pub mod backends;

use self::backends::Backend;
use clap::arg_enum;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Ops {
	Trace,
	Split,
}

arg_enum! {
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Apps {
	Firefox,
	Termite,
}
}

pub fn trace(application: Apps, refresh: bool) -> Result<(), Box<dyn Error>> {
	application.trace(refresh)
}
pub fn split(application: Apps, refresh: bool) -> Result<(), Box<dyn Error>> {
	// TODO: Process splitting to be used by runner.
	Ok(())
}
