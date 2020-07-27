// Full fledged solution to this is in works here: https://github.com/rust-lang/cargo/issues/97
//
// Until then parsing Cargo.toml using serde_toml
use rustc_version;

#[cfg(feature = "symbols")]
fn main() {
	let rustc_v = rustc_version::version().unwrap();
	println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_v);

	println!("cargo:rerun-if-env-changed=RUSTC_VERSION");
	println!("cargo:rerun-if-env-changed=CORE_VERSION");
}

#[cfg(not(feature = "symbols"))]
mod dyn_compile;

#[cfg(not(feature = "symbols"))]
fn main() {
	let rustc_v = rustc_version::version().unwrap();
	println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_v);

	dyn_compile::compile("backends", "lib/backends");

	println!("cargo:rerun-if-env-changed=RUSTC_VERSION");
	println!("cargo:rerun-if-env-changed=CORE_VERSION");
}
