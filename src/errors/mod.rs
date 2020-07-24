//! Errors
//!
//! Errors for Godwit Daemon.
use custom_error::custom_error;

custom_error! {pub PatchError
	IO {source: std::io::Error} = "IO error occured.",
	Glob {source: glob::GlobError} = "GlobError occured.",
	FromUtf8 {source: std::string::FromUtf8Error} = "FromUtf8Error occured.",
	Serde {source: serde_json::Error} = "Serde_Json error occured.",
}

custom_error! {pub PurgeBaseError
	IO {source: std::io::Error} = "IO error occured.",
	Glob {source: glob::GlobError} = "GlobError occured.",
	FromUtf8 {source: std::string::FromUtf8Error} = "FromUtf8Error occured.",
}

custom_error! {pub RevisionError
	BaseNotFound {base_path: String} = "Base file at {base_path} wasn't found. Can't traverse the revision.",
	IO {source: std::io::Error} = "IO error occured.",
	GlobPattern {source: glob::PatternError} = "PatternError occured.",
	SerdeError {source: serde_json::Error} = "Serde_Json error occured.",
	Patch {source: PatchError} = "PatchError occured.",
}

custom_error! {pub BackendError
	NotFound {backend_str: String} = "Backend {backend_str} not found.",
	Glob {source: glob::GlobError} = "GlobError occured.",
	VersionMismatch = "RUSTC_VERSION or CORE_VERSION mismatched.",
	LibLoading {source: libloading::Error} = "Dynamic libloading threw unexpected error.",
}

custom_error! {pub TraceError
	Backend {source: BackendError} = "Backend threw unexpected error.",
	Internal {err_str: String} = "Backend threw custom error: {err_str}",
	IO {source: std::io::Error} = "IO error occured.",
	Glob {source: glob::GlobError} = "GlobError occured.",
	GlobPattern {source: glob::PatternError} = "PatternError occured.",
	PurgeBase {source: PurgeBaseError} = "PurgeBaseError occured.",
	Patch {source: PatchError} = "PatchError occured.",
	Revision {source: RevisionError} = "RevisionError occured.",
}

custom_error! {pub DelegateError
	PurgeBase {source: PurgeBaseError} = "PurgeBaseError occured.",
	Backend {source: BackendError} = "BackendError occured.",
	Trace {source: TraceError} = "TraceError occured.",
}

custom_error! {pub NetworkError
	IO {source: std::io::Error} = "IO error occured.",
	ZMQ {source: zmq::Error} = "ZMQ error occured.",
	Serde {source: serde_json::Error} = "Serde_Json error occured."
}

custom_error! {pub RunError
	Network {source: NetworkError} = "NetworkError occured."
}
