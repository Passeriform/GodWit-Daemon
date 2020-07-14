use crate::core::{self, Apps, Ops};
use crate::errors::DelegateError;
use serde::{Deserialize, Serialize};
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
) -> Result<(), DelegateError> {
	match handler {
		HandleOps::Run => delegate(func.unwrap(), application.unwrap(), refresh),
		HandleOps::Kill => terminate(None, func, application),
		HandleOps::Heartbeat => heartbeat(),
	}
}

pub fn delegate(func: Ops, application: Apps, refresh: bool) -> Result<(), DelegateError> {
	match func {
		Ops::Trace => core::trace(application, refresh).map_err(Into::into),
		Ops::Split => core::split(application, refresh).map_err(Into::into),
	}
}

pub fn terminate(
	thread_id: Option<u16>,
	operation: Option<Ops>,
	application: Option<Apps>,
) -> Result<(), DelegateError> {
	Ok(())
}

pub fn heartbeat() -> Result<(), DelegateError> {
	Ok(())
}
