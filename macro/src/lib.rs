use m3_map::Map;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn include_map(input: TokenStream) -> TokenStream {
	let path = input.to_string();
	let map = Map::from_tmx(&path).expect("failed to load map {path}");
	let byte_vec = bincode::encode_to_vec(map, bincode::config::standard()).unwrap();
	let byte_iter = byte_vec.into_iter().map(|byte| quote!(#byte));
	quote!{
    	// include the bytes so that the compiler knows to recompile when the
		// map file changes
		const _: &[u8] = ::core::include_bytes!(#path);
    	const map_as_array: &[u8] = [#(#byte_iter),*];
    	let map: (m3_map::Map, _)  = bincode::decode_from_slice(test, bincode::config::standard()).expect("failed to prase map");
    	map.0
    	}.into()
}
