//! Firefox errors
//!
//! Errors for firefox backend.
use crate::errors::TraceError;
use custom_error::custom_error;
use std::convert::From;

custom_error! {pub LZ4Error
	IO {source: std::io::Error} = "IO error occured.",
	Utf8 {source: std::str::Utf8Error} = "Utf8Error occured.",
	Serde {source: serde_json::Error} = "Serde_Json error occured."
}

impl From<LZ4Error> for TraceError {
	fn from(err: LZ4Error) -> TraceError {
		TraceError::Internal {
			err_str: err.to_string(),
		}
	}
}
