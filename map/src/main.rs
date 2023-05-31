#![warn(rust_2018_idioms, unreachable_pub)]
#![deny(unsafe_code)]
#![forbid(unused_must_use)]

use clap::Parser;
use m3_map::Map;

#[derive(Debug, Parser)]
pub struct Opt {
	file: String
}

fn main() {
	let opt = Opt::parse();
	let result = Map::from_tmx(opt.file);
	match result {
		Err(err) => {
			eprintln!("ERROR: {err}");
			std::process::exit(1);
		},
		Ok(map) => println!("{map:#?}\nmap is valid")
	}
}
