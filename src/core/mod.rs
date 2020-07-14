pub mod backends;

use self::backends::Backend;
use crate::errors::{SplitError, TraceError};
use clap::arg_enum;
use serde::{Deserialize, Serialize};

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

pub fn trace(application: Apps, refresh: bool) -> Result<(), TraceError> {
	application.trace(refresh)
}
pub fn split(application: Apps, refresh: bool) -> Result<(), SplitError> {
	// TODO: Process splitting to be used by runner.
	Ok(())
}
