//
// Hippo
// (C) 2021 Brave Monday
//

use serde::Deserialize;

/// A set of supported output formats.
#[derive(Deserialize)]
pub enum OutputFormat {
	#[serde(rename = "bytes")]
	Bytes,

	#[serde(rename = "utf-8")]
	Utf8
}

impl Default for OutputFormat {
	/// Construct an output format set to bytes.
	fn default() -> Self {
		OutputFormat::Bytes
	}
}
