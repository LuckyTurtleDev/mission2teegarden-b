#[cfg(target_os = "window")]
use attohttpc::get;
#[cfg(target_os = "window")]
use piz::read::ZipArchive;
#[cfg(target_os = "windows")]
use std::path::Path;
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
	let mut save_to = File::create(file).expect(&format!("failed to create file {file}"));
	io::copy(&mut reader, &mut save_to).expect(&format!("failed to write file {file}"));
}

fn main() {
	//download SDL2 to working directory
	#[cfg(target_os = "window")]
	if !Path::new("SDL2.dll").exists() {
		// Download zip archive
		let bytes = get("https://github.com/libsdl-org/SDL/releases/download/release-2.26.5/SDL2-2.26.5-win32-x64.zip")
    		.send()
    		.expect("failed to download SDL2")
    		.error_for_status()
    		.expect("failed to download SDL2").bytes().unwrap();
		let zip = ZipArchive::new(&bytes).expect("failed to read zip archive");
		extract_file(&zip, "README-SDL.txt");
		extract_file(&zip, "SDL2.dll");
	}
}
