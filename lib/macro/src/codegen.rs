//
// Hippo
// (C) 2021 Brave Monday
//

use proc_macro2 as pm2;

use proc_macro_error::abort_call_site;
use quote::{format_ident, quote, ToTokens};

/// An output container that enables selective code generation based on type.
pub enum Container {
	Bytes(Vec<u8>),
	Utf8(String)
}

impl Container {
	/// Deduce the backing type of the data.
	pub fn deduce_type(&self) -> syn::Type {
		let text = match self {
			Container::Bytes(b) => format!("[u8; {}]", b.len()),
			Container::Utf8(_)  => "&'static str".to_string()
		};

		syn::parse_str(&text).unwrap_or_else(
			|e| abort_call_site!("cannot deduce output type: {}", e)
		)
	}
}

impl ToTokens for Container {
	/// Render a data container into a `TokenStream`.
	fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
		match self {
			Container::Bytes(b) => tokens.extend(quote! {
				[ #(#b),* ]
			}),
			Container::Utf8(s) => s.to_tokens(tokens)
		}
	}
}

/// Produce an AST for the implementation.
pub fn emit(ast: syn::DeriveInput, inputs: &Vec<String>, data: Container) -> pm2::TokenStream {
	let name = &ast.ident;
	let ty   = data.deduce_type();

	let (ig, tg, wc) = ast.generics.split_for_impl();

	let mut tokens = quote! {
		use hippo::Preprocessed;

		impl #ig #name #tg #wc {
			const HIPPO_DATA: #ty = #data;
		}

		impl #ig Preprocessed<#ty> for #name #tg #wc {
			fn preprocessed_data() -> &'static #ty {
				&Self::HIPPO_DATA
			}
		}
	};

	tokens.extend(match data {
		Container::Bytes(_) => quote! {
			impl #ig std::fmt::Display for #name #tg #wc {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					write!(f, "{:?}", Self::preprocessed_data())
				}
			}
		},
		Container::Utf8(_) => quote! {
			impl #ig std::fmt::Display for #name #tg #wc {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					write!(f, "{}", Self::preprocessed_data())
				}
			}
		}
	});

	// Output original data with built-in include macros to force the compiler to
	// consider the input files when tracking changes. Hopefully, the compiler will
	// aggressively remove these unless they're used directly.
	for (i, v) in inputs.iter().enumerate() {
		let ident = format_ident!("HIPPO_ORIGINAL_DATA_{}", i);

		tokens.extend(match data {
			Container::Bytes(_) => quote! {
				impl #ig #name #tg #wc {
					const #ident: #ty = &include_bytes!(#v);
				}
			},
			Container::Utf8(_) => quote! {
				impl #ig #name #tg #wc {
					const #ident: #ty = &include_str!(#v);
				}
			},
		});
	}

	tokens
}
