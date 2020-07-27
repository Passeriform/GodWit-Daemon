// This utility is not-so-light more-or-less verbatim https://github.com/auxoncorp/cargo-5730

use flate2::read::GzDecoder;
use glob::glob;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::{env, fs, iter, path, process};
use tar::Archive;
use toml::Value;

struct BuildDir {
	pub path: path::PathBuf,
}

impl BuildDir {
	fn new() -> Self {
		let mut rng = thread_rng();

		let hex_str: String = iter::repeat(())
			.map(|()| rng.sample(Alphanumeric))
			.take(6)
			.collect();

		let mut dir = env::temp_dir();
		dir.push(format!("dyn-build-{}", hex_str));

		BuildDir { path: dir }
	}
}

impl Drop for BuildDir {
	fn drop(&mut self) {
		// some paranoia before running 'rm -rf'
		assert!(self.path.starts_with(env::temp_dir()));

		println!("Removing build crate staging dir: {}", self.path.display());
		fs::remove_dir_all(&self.path).expect(&format!(
			"Couldn't clean up build dir: {}",
			self.path.display()
		));
	}
}

struct Dependency {
	name: String,
	version: String,
}

fn extract_section_deps(toml_path: &path::Path, section_name: &str) -> Vec<Dependency> {
	let content = fs::read_to_string(toml_path).expect("Couldn't read contents of Cargo.toml");

	let parsed_toml: Value = toml::from_str(&content).expect("Couldn't parse Cargo.toml");

	let mut dependencies: Vec<Dependency> = Vec::new();

	if let Some(section) = parsed_toml.get(section_name) {
		if section.is_table() {
			dependencies = section
				.as_table()
				.unwrap()
				.iter()
				.map(|(key, value)| Dependency {
					name: key.to_string(),
					version: value.as_str().unwrap().to_string(),
				})
				.collect();
		}
	}

	dependencies
}

fn build_fetched_crate(
	crate_src: &path::Path, // /tmp/...
	path: &str,             // ENV PATH
) -> Result<(), Box<dyn Error>> {
	let res = process::Command::new(env::var("CARGO")?)
		.args(&["build", "--release"])
		.env_clear()
		.env("PATH", path)
		.current_dir(&crate_src)
		.stdout(process::Stdio::inherit())
		.stderr(process::Stdio::inherit())
		.output()?;

	assert!(
		res.status.success(),
		"Failed to run compile build crate at {} with {:#?}",
		crate_src.display(),
		res
	);

	Ok(())
}

pub fn compile(custom_head_name: &str, lib_path: &str) {
	// Bootstrap
	let path = env::var("PATH").expect("Can't get PATH from env");

	let base_dir = env::var("PWD").expect("Can't get PWD from env");
	let base_dir = path::Path::new(&base_dir);

	let lib_path = &base_dir.join(lib_path);
	let lib_path = path::Path::new(&lib_path);

	fs::create_dir_all(&lib_path).expect(&format!(
		"Couldn't create dynamic lib directory at {}",
		lib_path.display()
	));

	let cargo_toml = base_dir.join("Cargo.toml");

	for dependency in extract_section_deps(&cargo_toml, &custom_head_name) {
		// Build in /tmp to avoid the influence of .cargo/config settings in the
		// build crate's parent, which cargo gives us no way to ignore.

		if lib_path.join(&format!("{}.so", dependency.name)).exists() {
			continue;
		}

		let external_lib_dir = BuildDir::new();
		let lib_so_search_path = external_lib_dir
			.path
			.join(format!("{}-{}", &dependency.name, &dependency.version));

		fs::create_dir(&external_lib_dir.path).expect(&format!(
			"Couldn't create temp build dir at {}",
			external_lib_dir.path.display()
		));

		// Fetch backend from crates.io
		let crate_url = format!(
			"https://crates.io/api/v1/crates/{}/{}/download",
			&dependency.name, &dependency.version
		);

		let mut response = reqwest::blocking::get(&crate_url)
			.expect(&format!("Failed to download `{}`", &dependency.name));

		let mut body = Vec::new();
		response.copy_to(&mut body).unwrap();

		// Unpack the .tar.gz archive
		let gz = GzDecoder::new(body.as_slice());
		let mut tar = Archive::new(gz);
		tar.unpack(&external_lib_dir.path)
			.expect("Unpacking tar threw error.");

		build_fetched_crate(&lib_so_search_path, &path)
			.expect(&format!("Build for backend {:?} failed", &dependency.name));

		for entry in glob(&format!("{}/target/*/*.so", &lib_so_search_path.display()))
			.expect("Failed to parse glob pattern")
		{
			if let Ok(cdylib) = entry {
				fs::copy(&cdylib, &lib_path.join(&format!("{}.so", dependency.name))).expect(
					&format!(
						"Copying dynamic library was unsuccessful. src: {}, dst: {}",
						&cdylib.display(),
						&lib_path.join(&dependency.name).display()
					),
				);
			}
		}
	}
}
