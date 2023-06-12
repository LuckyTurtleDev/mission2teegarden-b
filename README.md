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
* run `cargo install m3 --locked`.
See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
* make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable

# Flash Pybadge:
Download Pybadge binary from [Github release](https://github.com/LuckyTurtleDev/m3/releases/latest)
and flash it with an UF2 flash. I recommand using [hf2-cli](https://crates.io/crates/hf2-cli).
Press the reset button of the pybdage twice, to enter the bootloader, before flashing.
### Building from source: 
Alternative you can easily build m3 by yourself:
* [install rust(up)](https://www.rust-lang.org/tools/install)
* install the rust `thumbv7em-none-eabihf` target. (the architecture of the pybadge)
```bash
rustup target install thumbv7em-none-eabihf
```
* optional: install nightly toolchain for better error messages on pybadge (please use this version for bug reports).
```bash
rustup toolchain install nightly --target thumbv7em-none-eabihf
```
