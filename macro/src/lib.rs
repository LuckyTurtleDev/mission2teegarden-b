#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use glob::glob;
use m3_map::Map;
use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;

#[proc_macro]
pub fn include_map(input: TokenStream) -> TokenStream {
	let mut path = input.to_string();
	path.remove(0);
	path.pop();
	let path = PathBuf::from(path).canonicalize().unwrap();
	let path = path.to_str().unwrap();
	let map = Map::from_tmx(path).expect("failed to load map");
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
	quote! {
		{
		// include the bytes so that the compiler knows to recompile when the
		// map or tilesets changes
		const _: &[u8] = ::core::include_bytes!(#path);
		#tileset;
		#map
	}
	}
	.into()
}
