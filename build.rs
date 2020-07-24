// Full fledged solution to this is in works here: https://github.com/rust-lang/cargo/issues/97
//
// Until then parsing Cargo.toml using serde_toml
use rustc_version;

fn main() {
	let rustc_v = rustc_version::version().unwrap();
	println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_v);

	println!("cargo:rerun-if-env-changed=RUSTC_VERSION");
	println!("cargo:rerun-if-env-changed=CORE_VERSION");
}

// cargo build -q --lib --all-targets --all-features --release --out-dir
