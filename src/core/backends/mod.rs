pub mod firefox;
pub mod termite;

use super::Apps;
use std::error::Error;

pub trait Backend {
	fn trace(&self, refresh: bool) -> Result<(), Box<dyn Error>>;
}

impl Backend for Apps {
	fn trace(&self, refresh: bool) -> Result<(), Box<dyn Error>> {
		match self {
			Apps::Firefox => firefox::trace(refresh),
			Apps::Termite => termite::trace(refresh),
		}
	}
}
