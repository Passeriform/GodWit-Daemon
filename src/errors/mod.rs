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

custom_error! {pub TraceError
	Internal {err_str: String} = "Backend threw custom error: {err_str}",
	IO {source: std::io::Error} = "IO error occured.",
	Glob {source: glob::GlobError} = "GlobError occured.",
	GlobPattern {source: glob::PatternError} = "PatternError occured.",
	PurgeBase {source: PurgeBaseError} = "PurgeBaseError occured.",
	Patch {source: PatchError} = "PatchError occured.",
	Revision {source: RevisionError} = "RevisionError occured.",
}

custom_error! {pub SplitError
	Internal {err_str: String} = "Backend threw custom error: {err_str}",
	Patch {source: PatchError} = "PatchError occured.",
	Revision {source: RevisionError} = "RevisionError occured.",
}

custom_error! {pub DelegateError
	PurgeBase {source: PurgeBaseError} = "PurgeBaseError occured.",
	Trace {source: TraceError} = "TraceError occured.",
	Split {source: SplitError} = "SplitError occured.",
}

custom_error! {pub NetworkError
	IO {source: std::io::Error} = "IO error occured.",
	ZMQ {source: zmq::Error} = "ZMQ error occured.",
	Serde {source: serde_json::Error} = "Serde_Json error occured."
}

custom_error! {pub RunError
	Network {source: NetworkError} = "NetworkError occured."
}
