use glob::glob;
use std::path::PathBuf;

use m3_map::Map;
use proc_macro::TokenStream;
use quote::quote;
use self_rust_tokenize::SelfRustTokenize;

#[proc_macro]
pub fn include_map(input: TokenStream) -> TokenStream {
	let mut path = input.to_string();
	path.remove(0);
	path.pop();
	let path = PathBuf::from(path).canonicalize().unwrap();
	let path = path.to_str().unwrap();
	let map = Map::from_tmx(path).expect("failed to load map");
	let map = SelfRustTokenize::to_tokens(&map);
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
		use m3_map::Map;
		use m3_map::tiles::MapBaseTile;
		use m3_map::Player;
		use m3_map::tiles::ObjectTile;
		use m3_map::Orientation;
		// include the bytes so that the compiler knows to recompile when the
		// map or tilesets changes
		const _: &[u8] = ::core::include_bytes!(#path);
		#tileset;
		#map
	}
	}
	.into()
}
