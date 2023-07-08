use crate::{Map, MAP_FILE_EXTENSION};
use anyhow::Context;
use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug, Parser)]
pub struct OptValidateMap {
	file: String
}

#[derive(Debug, Parser)]
pub struct OptExportMap {
	/// Map to be exported.
	file: String
}

pub fn validate(opt: OptValidateMap) -> anyhow::Result<()> {
	let map = Map::from_tmx(opt.file)?;
	println!("{map:#?}\nmap is valid");
	Ok(())
}

pub fn export(opt: OptExportMap) -> anyhow::Result<()> {
	let map = Map::from_tmx(&opt.file).context("Failed to load tmx map from file}")?;
	let path = PathBuf::from(opt.file);
	let path = path.file_name().context("no filename at path")?;
	let mut path = PathBuf::from(path);
	path.set_extension(MAP_FILE_EXTENSION);
	println!("export map to {path:?}");
	let mut file = File::create(&path)
		.with_context(|| format!("failed to create file {:?}", path))?;
	write!(file, "{}", map.to_string())
		.with_context(|| format!("failed to wirte file {:?}", path))
}
