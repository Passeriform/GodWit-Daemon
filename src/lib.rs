pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub mod config;
pub mod core;
pub mod dispatcher;
pub mod errors;
pub mod prochandler;
pub mod runner;
