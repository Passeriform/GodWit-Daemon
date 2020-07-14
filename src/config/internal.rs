use crate::errors::PatchError;
use serde::Serialize;
use std::convert::From;
use std::fmt::Display;
use std::io::Write;
use std::process::Command;
use std::string::String;
use tempfile::NamedTempFile;

pub fn diff<T: Display + From<String>, U: Display + From<String>>(
	original: T,
	changed: U,
) -> Result<String, PatchError>
where
	T: Serialize,
	U: Serialize,
{
	let tmp_original = NamedTempFile::new()?;
	serde_json::to_writer_pretty(&tmp_original, &original)?;

	let tmp_changed = NamedTempFile::new()?;
	serde_json::to_writer_pretty(&tmp_changed, &changed)?;

	let output = Command::new("diff")
		.args(&[
			"-u",
			&format!("{}", tmp_original.path().to_string_lossy()),
			&format!("{}", tmp_changed.path().to_string_lossy()),
		])
		.output()?;

	tmp_original.close()?;
	tmp_changed.close()?;

	Ok(String::from_utf8(output.stdout)?)
}

pub fn patch<T: Display + From<String>, U: Display + From<String>>(
	original: T,
	patch: U,
) -> Result<String, PatchError> {
	let mut tmp_original = NamedTempFile::new()?;
	tmp_original.write_all(original.to_string().as_bytes())?;

	let mut tmp_patch = NamedTempFile::new()?;
	tmp_patch.write_all(patch.to_string().as_bytes())?;

	let output = Command::new("patch")
		.args(&[
			&format!("{}", tmp_original.path().to_string_lossy()),
			&format!("{}", tmp_patch.path().to_string_lossy()),
		])
		.output()?;

	tmp_original.close()?;
	tmp_patch.close()?;

	Ok(String::from_utf8(output.stdout)?)
}
