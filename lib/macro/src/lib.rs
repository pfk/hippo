//
// Hippo
// (C) 2021 Brave Monday
//

extern crate proc_macro;

mod codegen;
mod meta;

use proc_macro  as pm1;
use proc_macro2 as pm2;

use proc_macro_error::{abort_call_site, proc_macro_error};

use hippo_shared::{Configuration, OutputFormat};

use codegen::{emit, Container};
use meta::Meta;

/// Preprocess and embed an asset in conformance to the [`Preprocessed`](hippo::Preprocessed)
/// trait.
#[proc_macro_derive(Preprocess, attributes(hippo))]
#[proc_macro_error]
pub fn preprocess(ast: pm1::TokenStream) -> pm1::TokenStream {
	let ast  = syn::parse_macro_input!(ast as syn::DeriveInput);
	let meta = Meta::new(&ast);

	let pp_name   = &meta.preprocessor;
	let pp_inputs = &meta.inputs;

	let cfg_path = Configuration::locate().unwrap_or_else(
		|| abort_call_site!("Hippo configuration file could not be located")
	);

	let cfg = Configuration::load(&cfg_path).unwrap_or_else(
		|e| abort_call_site!("Hippo configuration contains one or more errors: {}", e)
	);

	let pp = cfg.preprocessors.get(pp_name).unwrap_or_else(
		|| abort_call_site!("preprocessor `{}` not found in configuration", pp_name)
	);

	let out = pp.execute(pp_inputs.to_vec()).unwrap_or_else(
		|e| abort_call_site!("preprocessor command resulted in an error: {}", e)	
	);

	let output: pm2::TokenStream = {
		match pp.format {
			OutputFormat::Bytes => emit(
				ast,
				Container::Bytes(out.stdout.into())
			),
			OutputFormat::Utf8 => emit(
				ast,
				Container::Utf8(String::from_utf8_lossy(&out.stdout).to_string())
			)
		}
	};

	pm1::TokenStream::from(output)
}
