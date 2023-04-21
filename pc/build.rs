#[cfg(target_os = "windows")]
use attohttpc::get;
#[cfg(target_os = "windows")]
use piz::read::ZipArchive;
#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::{fs::File, io};

#[cfg(target_os = "windows")]
fn extract_file(zip: &ZipArchive, file: &str) {
	let entry = zip
		.entries()
		.iter()
		.find(|f| f.path.as_str() == file)
		.expect(&format!("can not found {file} in zip"));
	let mut reader = zip.read(entry).expect("failed to create zip reader");
	let output_path = PathBuf::from(file);
	let output_path: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap())
		.join(output_path.file_name().unwrap());
	let mut save_to =
		File::create(output_path).expect(&format!("failed to create file {file}"));
	io::copy(&mut reader, &mut save_to).expect(&format!("failed to write file {file}"));
}

fn main() {
	//download SDL2 to working directory
	#[cfg(target_os = "windows")]
	{
		let files = [
			"SDL2-2.26.5/README-SDL.txt",
			"SDL2-2.26.5/lib/x86/SDL2.dll",
			"SDL2-2.26.5/lib/x86/SDL2.lib"
		];
		if files.iter().any(|f| !Path::new(f).exists()) {
			// Download zip archive
			let bytes = get("https://github.com/libsdl-org/SDL/releases/download/release-2.26.5/SDL2-devel-2.26.5-VC.zip")
    		.send()
    		.expect("failed to download SDL2")
    		.error_for_status()
    		.expect("failed to download SDL2")
    		.bytes()
    		.unwrap();
			let zip = ZipArchive::new(&bytes).expect("failed to read zip archive");
			for file in files {
				extract_file(&zip, file)
			}
		}
	}
}
