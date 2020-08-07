// Full fledged solution to this is in works here: https://github.com/rust-lang/cargo/issues/97
//
// Until then parsing Cargo.toml using serde_toml
use rustc_version;

#[cfg(any(feature = "symbols", feature = "docs-rs"))]
fn main() {
	let rustc_v = rustc_version::version().unwrap();
	println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_v);

	println!("cargo:rerun-if-env-changed=RUSTC_VERSION");
	println!("cargo:rerun-if-env-changed=CORE_VERSION");
}

#[cfg(all(not(feature = "symbols"), not(feature = "docs-rs")))]
mod dyn_compile;

#[cfg(all(not(feature = "symbols"), not(feature = "docs-rs")))]
fn main() {
	let rustc_v = rustc_version::version().unwrap();
	println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_v);

	dyn_compile::compile("backends", "lib/backends");

	println!("cargo:rerun-if-env-changed=RUSTC_VERSION");
	println!("cargo:rerun-if-env-changed=CORE_VERSION");
}
