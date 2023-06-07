#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use glob::glob;
use m3_map::Map;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use std::{fmt::Display, path::PathBuf};

struct Error {
	span: Span,
	msg: String
}

impl Error {
	fn new<T: Display>(span: Span, msg: T) -> Self {
		Self {
			span,
			msg: msg.to_string()
		}
	}

	fn into_compile_error(self) -> TokenStream2 {
		let Self { span, msg } = self;
		quote_spanned! { span => ::core::compile_error!(#msg) }
	}
}

fn expand_include_map(path: &str) -> Result<TokenStream2, Error> {
	let map = Map::from_tmx(path).map_err(|err| Error::new(Span::call_site(), err))?;
	let map = map.to_string();
	let mut tileset = quote!();
	// use glob as workaround until
	// https://github.com/mapeditor/rs-tiled/issues/263#issuecomment-1532822341 is impl and released
	for path in glob("./**/*.tsx").unwrap() {
		let path = path.unwrap().canonicalize().unwrap();
		let path = path.to_str().unwrap();
		tileset = quote! {
			#tileset;
			const _: &[u8] = ::core::include_bytes!(#path)
		}
	}
	Ok(quote! {
		{
			// include the bytes so that the compiler knows to recompile when the
			// map or tilesets changes
			const _: &[u8] = ::core::include_bytes!(#path);
			#tileset;
			#map
		}
	})
}

#[proc_macro]
pub fn include_map(input: TokenStream) -> TokenStream {
	let mut path = input.to_string();
	path.remove(0);
	path.pop();
	let path = PathBuf::from(path).canonicalize().unwrap();
	let path = path.to_str().unwrap();

	expand_include_map(path)
		.unwrap_or_else(Error::into_compile_error)
		.into()
}
