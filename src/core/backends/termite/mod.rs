pub mod errors;

use crate::errors::TraceError;
use std::env;
use std::fs::File;
use std::path::PathBuf;

pub fn trace(refresh: bool) -> Result<(), TraceError> {
	let histpath = env::var("HISTFILE").or_else(|_| {
		Err(TraceError::Internal {
			err_str: "No HISTFILE variable set. Can't track shell state.".to_string(),
		})
	})?;
	let histfile = File::open(PathBuf::from(histpath))?;

	if refresh {
		// TODO: Purge prev_state
	}

	// TODO: If prev_state doesn't exist, create new state, save onto disk and early exit
	// else
	// TODO: Calculate diff between histfile and prev_state and and save onto disk

	Ok(())
}
