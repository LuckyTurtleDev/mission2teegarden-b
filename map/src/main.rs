#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use std::process;

use clap::Parser;
use mission2teegarden_b_map::commands::*;

#[derive(Debug, Parser)]
enum Opt {
	/// Validate a Tiled map
	Validate(OptValidateMap),
	/// Export a tiled map to an mission2teegarden-b level
	Export(OptExportMap)
}

fn main() {
	let opt = Opt::parse();
	let result = match opt {
		Opt::Validate(opt) => validate(opt),
		Opt::Export(opt) => export(opt)
	};
	if let Err(err) = result {
		eprintln!("{err:?}");
		process::exit(1);
	}
}
