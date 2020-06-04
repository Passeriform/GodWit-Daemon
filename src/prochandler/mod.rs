use crate::core::{self, Apps, Ops};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum HandleOps {
	Run,
	Kill,
	Heartbeat,
}

impl Default for HandleOps {
	fn default() -> Self {
		HandleOps::Heartbeat
	}
}

impl fmt::Display for HandleOps {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub fn handle(
	handler: HandleOps,
	func: Option<Ops>,
	application: Option<Apps>,
	refresh: bool,
) -> Result<(), Box<dyn Error>> {
	match handler {
		HandleOps::Run => dispatch(func.unwrap(), application.unwrap(), refresh),
		HandleOps::Kill => terminate(None, func, application),
		HandleOps::Heartbeat => heartbeat(),
	}
}

pub fn dispatch(func: Ops, application: Apps, refresh: bool) -> Result<(), Box<dyn Error>> {
	match func {
		Ops::Trace => core::trace(application, refresh),
		Ops::Split => core::split(application, refresh),
	}
}

pub fn terminate(
	thread_id: Option<u16>,
	operation: Option<Ops>,
	application: Option<Apps>,
) -> Result<(), Box<dyn Error>> {
	Ok(())
}

pub fn heartbeat() -> Result<(), Box<dyn Error>> {
	Ok(())
}
