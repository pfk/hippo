//
// Hippo
// (C) 2021 Brave Monday
//

use proc_macro_error::abort_call_site;

/// A set of procedure macro attributes.
pub struct Meta {
	/// The name of the preprocessor to be used.
	pub preprocessor: String,

	/// The input arguments to be processed.
	pub inputs: Vec<String>
}

impl Meta {
	/// Construct a meta object.
	pub fn new(ast: &syn::DeriveInput) -> Self {
		let mut parts: Vec<String> = Vec::new();

		for attr in ast.attrs.iter() {
			let attr = attr.parse_meta().unwrap_or_else(
				|e| abort_call_site!("attribute could not be parsed: {}", e)
			);

			if !attr.path().is_ident("hippo") {
				continue;
			}

			let list = match attr {
				syn::Meta::List(x) => x,

				_ => abort_call_site!("attribute must be a list")
			};

			for entry in list.nested {
				if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = entry {
					parts.push(s.value());
				} else {
					abort_call_site!("attribute may only contain strings");
				}
			}
		}

		if parts.len() < 1 {
			abort_call_site!("attribute must contain at least one entry");
		}

		Meta {
			preprocessor: parts[0].clone(),
			inputs:       parts[1..].to_vec()
		}
	}
}
