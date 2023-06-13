# 🚗 m3
![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![m3-pc on crates.io](https://img.shields.io/crates/v/m3-pc)](https://crates.io/crates/m3-pc)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/m3)
![Rust Version: ^1.64](https://img.shields.io/badge/rustc-%5E1.64-orange.svg)

# Installation (Pc): 
m3 is avaibale at the following repositories:

[![Packaging status](https://repology.org/badge/vertical-allrepos/m3.svg)](https://repology.org/project/m3/versions)

Prebuild binarys can also been downloaded from the [Github release](https://github.com/LuckyTurtleDev/m3/releases/latest).

### Building from source: 
Alternative you can easily build m3 by yourself:
* on Linux install the following dev dependencies:
  * [`alsa-lib`](https://github.com/alsa-project/alsa-lib)
  * [`libudev`](https://github.com/systemd/systemd)
* [install rust](https://www.rust-lang.org/tools/install)
* run `cargo install m3 --locked` to build and install the m3.
See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
* make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable

# Flash Pybadge:
Install an UF2 flasher. I recommand using [hf2-cli](https://crates.io/crates/hf2-cli).
Download and unpack Pybadge binary from [Github release](https://github.com/LuckyTurtleDev/m3/releases/latest).
Press the reset button of the pybdage twice, to enter the bootloader.
After this execute `hf2 elf m3-pybadge` (or the corresponding command of your flahing tool) to flash the binary to the pybadge.
### Building from source: 
Alternative you can build m3 by yourself:
* [install rust(up)](https://www.rust-lang.org/tools/install)
* run `cargo install hf2-cli --locked` to build and install the [hf2-cli](https://crates.io/crates/hf2-cli) flasher.
See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
* make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable
* install the rust `thumbv7em-none-eabihf` target. (the architecture of the pybadge)
```bash
rustup target install thumbv7em-none-eabihf
```
* optional: install nightly toolchain for better error messages at the pybadge.
```bash
rustup toolchain install nightly --target thumbv7em-none-eabihf
```
* download and unpack source code

On Linux or Macos please execute the following command, to download and unpack the source code.
```bash
wget -qO- https://crates.io/api/v1/crates/m3/$(wget -qO- https://crates.io/api/v1/crates/m3 | jq -r '.versions[0].num')/download | tar -xz
```
If you use windows or do not have `wget`,`jq` or `tar` installed,
please download the source manual by using this link `https://crates.io/api/v1/crates/m3/VERSION/download`.
Replace `VERSION` with the latest latest version of the m3-pybadge crate, whitch can be found on [crates.io](https://crates.io/crates/m3).
After this unpack the downloaded tar archive.
* switch working directory to the new downloaded directory
```bash
cd m3-*
```
* press the reset button of the pybadge twice to enter bootloader
* compile and flash program
```bash
cargo +nightly run --release -locked
```
`+nightly` is optional and have to be left out if the "install nightly toolchain" step was skip.
Please use `+nightly` for bug reports.
