use m3_map::Map;
use proc_macro::TokenStream;
use quote::quote;
use self_rust_tokenize::SelfRustTokenize;

#[proc_macro]
pub fn include_map(input: TokenStream) -> TokenStream {
	let mut path = input.to_string();
	path.remove(0);
	path.pop();
	let map = Map::from_tmx(&path).expect("failed to load map");
	let map = SelfRustTokenize::to_tokens(&map);
	quote! {
		{
		use m3_map::Map;
		use m3_map::tiles::MapBaseTile;
		#map
	}
	}
	.into()
}
