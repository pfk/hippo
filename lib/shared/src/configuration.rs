//
// Hippo
// (C) 2021 Brave Monday
//

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::Preprocessor;

/// A collection of program options.
#[derive(Deserialize)]
pub struct Configuration {
	#[serde(flatten)]
	pub preprocessors: HashMap<String, Preprocessor>
}

impl Configuration {
	/// The default configuration file name.
	pub const DEFAULT_NAME: &'static str = "Hippo.toml";

	/// Attempt to locate the Hippo on-disk configuration.
	pub fn locate() -> Option<PathBuf> {
		env::var("CARGO_MANIFEST_DIR").ok().map(|p| {
			let mut buf = PathBuf::new();
			
			buf.push(p);
			buf.push(Self::DEFAULT_NAME);

			buf
		})
	}

	/// Load an on-disk configuration.
	pub fn load(path: &Path) -> Result<Self> {
		toml::from_str(&fs::read_to_string(path)?).map_err(
			|e| Error::new(ErrorKind::Other, e)
		)
	}
}
