use crate::errors::{BackendError, TraceError};
use glob;
use libloading::Library;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

#[macro_export]
macro_rules! export_backend {
	($register:expr) => {
		#[doc(hidden)]
		#[no_mangle]
		pub static backend_declaration: $crate::core::BackendDeclaration =
			$crate::core::BackendDeclaration {
				rustc_version: $crate::RUSTC_VERSION,
				core_version: $crate::CORE_VERSION,
				register: $register,
			};
	};
}

#[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Clone)]
pub enum Ops {
	Trace,
}

pub trait Backend {
	fn trace(&self, args: BackendArgs) -> Result<(), TraceError>;
	fn help(&self) -> Option<&str> {
		None
	}
}

pub trait Registrar {
	fn register_backend(&mut self, name: &str, backend: Box<dyn Backend>);
}

pub struct BackendDeclaration {
	pub rustc_version: &'static str,
	pub core_version: &'static str,
	pub register: unsafe extern "C" fn(&mut dyn Registrar),
}

pub struct BackendArgs {
	pub refresh: bool,
}

pub struct BackendProxy {
	backend: Box<dyn Backend>,
	_lib: Rc<Library>,
}

impl Backend for BackendProxy {
	fn trace(&self, args: BackendArgs) -> Result<(), TraceError> {
		self.backend.trace(args)
	}

	fn help(&self) -> Option<&str> {
		self.backend.help()
	}
}

struct BackendRegistrar {
	backends: HashMap<String, BackendProxy>,
	lib: Rc<Library>,
}

impl BackendRegistrar {
	fn new(lib: Rc<Library>) -> BackendRegistrar {
		BackendRegistrar {
			backends: HashMap::default(),
			lib,
		}
	}
}

impl Registrar for BackendRegistrar {
	fn register_backend(&mut self, name: &str, backend: Box<dyn Backend>) {
		let proxy = BackendProxy {
			backend,
			_lib: Rc::clone(&self.lib),
		};
		self.backends.insert(name.to_string().to_lowercase(), proxy);
	}
}

pub struct ExternalBackends {
	pub backends: HashMap<String, BackendProxy>,
	libraries: Vec<Rc<Library>>,
}

impl ExternalBackends {
	pub fn new() -> ExternalBackends {
		ExternalBackends {
			backends: Default::default(),
			libraries: Default::default(),
		}
	}

	pub fn from_dir<P>(dir_path: P) -> Result<ExternalBackends, BackendError>
	where
		P: AsRef<Path>,
	{
		let mut ext_backends = ExternalBackends::new();

		for lib_so in glob::glob(&format!("{}/*.so", dir_path.as_ref().display())).unwrap() {
			unsafe {
				ext_backends.load(lib_so?)?;
			}
		}
		Ok(ext_backends)
	}

	pub unsafe fn load<P: AsRef<OsStr>>(&mut self, lib_path: P) -> Result<(), BackendError> {
		let library = Rc::new(Library::new(lib_path)?);

		let decl = library
			.get::<*mut BackendDeclaration>(b"backend_declaration\0")?
			.read();

		if decl.rustc_version != crate::RUSTC_VERSION || decl.core_version != crate::CORE_VERSION {
			return Err(BackendError::VersionMismatch);
		}

		let mut registrar = BackendRegistrar::new(Rc::clone(&library));

		(decl.register)(&mut registrar);

		self.backends.extend(registrar.backends);
		self.libraries.push(library);

		Ok(())
	}

	pub fn trace(&self, backend: &str, args: BackendArgs) -> Result<(), TraceError> {
		self.backends
			.get(backend)
			.ok_or_else(|| BackendError::NotFound {
				backend_str: backend.to_string(),
			})?
			.trace(args)
			.into()
	}
}
