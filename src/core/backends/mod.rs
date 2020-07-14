pub mod firefox;
pub mod termite;

use super::Apps;
use crate::errors::TraceError;

pub trait Backend {
	fn trace(&self, refresh: bool) -> Result<(), TraceError>;
}

impl Backend for Apps {
	fn trace(&self, refresh: bool) -> Result<(), TraceError> {
		match self {
			Apps::Firefox => firefox::trace(refresh),
			Apps::Termite => termite::trace(refresh),
		}
	}
}
