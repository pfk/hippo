//
// Hippo
// (C) 2021 Brave Monday
//

use std::env;
use std::io::Result;
use std::path::PathBuf;
use std::process::{Command, Output};

use serde::Deserialize;

use super::OutputFormat;

/// An agent responsible for preprocessor orchestration.
#[derive(Deserialize)]
pub struct Preprocessor {
	/// The command associated with the preprocessor.
	pub command: String,

	/// The collection of program options that will influence the command behavior.
	#[serde(default)]
	pub flags: Vec<String>,

	/// The path to the prepended to any input argument.
	pub prefix: Option<String>,

	/// The output format of the preprocessor. Defaults to bytes.
	#[serde(default)]
	pub format: OutputFormat
}

impl Preprocessor {
	/// Construct a preprocessor.
	pub fn new(command: &str) -> Self {
		Preprocessor {
			command: command.to_owned(),
			flags:   vec![],

			prefix: None,

			format: OutputFormat::Bytes
		}
	}

	/// Rewrite input arguments into absolute paths.
	pub fn rewrite_arguments(&self, args: &Vec<String>) -> Vec<String> {
		args.iter().map(|p| {
			let mut buf = PathBuf::new();

			if let Some(root) = env::var("CARGO_MANIFEST_DIR").ok() {
				buf.push(root);
			}

			if let Some(prefix) = &self.prefix {
				buf.push(prefix);
			}

			buf.push(p);

			buf.to_string_lossy().to_string()
		})
		.collect::<Vec<String>>()
	}

	/// Execute the preprocessor with input arguments.
	pub fn execute(&self, args: &Vec<String>) -> Result<Output> {
		Command::new(&self.command)
			.args(&self.flags)
			.args(args)
			.output()
	}
}
