use crate::core::{Backend, BackendArgs, ExternalBackends, Ops};
use crate::errors::{BackendError, DelegateError};
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
	application: Option<String>,
	refresh: bool,
) -> Result<(), DelegateError> {
	match handler {
		HandleOps::Run => delegate(func.unwrap(), application.unwrap().as_ref(), refresh),
		HandleOps::Kill => terminate(None, func, application),
		HandleOps::Heartbeat => heartbeat(),
	}
}

pub fn delegate(func: Ops, application: &str, refresh: bool) -> Result<(), DelegateError> {
	match func {
		Ops::Trace => ExternalBackends::from_dir("lib/backends")?
			.backends
			.get(application)
			.ok_or_else(|| BackendError::NotFound {
				backend_str: application.to_string(),
			})?
			.trace(BackendArgs { refresh })
			.map_err(Into::into),
	}
}

pub fn terminate(
	thread_id: Option<u16>,
	operation: Option<Ops>,
	application: Option<String>,
) -> Result<(), DelegateError> {
	Ok(())
}

pub fn heartbeat() -> Result<(), DelegateError> {
	Ok(())
}
