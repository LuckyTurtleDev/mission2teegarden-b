# 🪐 Mission to Teegarden b

![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/LuckyTurtleDev/mission2teegarden-b?label=latest&color=orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/latest)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/mission2teegarden-b)
![Rust Version: ^1.70](https://img.shields.io/badge/rustc-%5E1.70-orange.svg)


## Installation (Pc):

Mission to Teegarden b is avaibale at the following repositories:

[![Packaging status][__link0]][__link1]

Prebuild binarys can also been downloaded from the [Github release][__link2].


#### Building from source:

Alternative you can easily build Mission to Teegarden b  by yourself:

 - on Linux install the following development dependencies:
	 - [`alsa-lib`][__link3]
	 - [`libudev`][__link4] At some distros (lika Alpine and Debian) seperate development packages exist, regular suffixed with `-dev`. It this the case make sure that you have also installed the `*-dev` packages.
	
	
 - [install rust][__link5]
 - [Download][__link6] and unpack the source code.
 - run `cargo install --path pc --locked` inside the unpacked folder, to build and install the mission2teegarden-b. See the [rust book][__link7] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable, otherwise the `mission2teegarden-b` executeable can not be found.


## Flash Pybadge:

 - Install an UF2 flasher. I recommand using [hf2-cli][__link8].
 - Download and unpack Pybadge binary from [Github release][__link9].
 - Press the reset button of the pybdage twice, to enter the bootloader.
 - After this execute `hf2 elf mission2teegarden-b-pybadge` (or the corresponding command of your flahing tool) to flash the binary to the pybadge.
 - Press the reset button again.


#### Building from source:

Alternative you can build m3 by yourself:

 - [install rustup][__link10]
 - run `cargo install hf2-cli --locked` to build and install the [hf2-cli][__link11] flasher. See the [rust book][__link12] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable otherwise the executeable can not be found..
 - install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
 - optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
 - [Download][__link13] and unpack the source code (if not already done).
 - press the reset button of the pybadge twice to enter bootloader
 - compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder. `+nightly` is optional and have to be left out if the “install nightly toolchain” step was skip. Please use `+nightly` for bug reports.
 - Press the reset button again.


 [__link0]: https://repology.org/badge/vertical-allrepos/mission2teegarden_b.svg
 [__link1]: https://repology.org/project/mission2teegarden-b/versions
 [__link10]: https://www.rust-lang.org/tools/install
 [__link11]: https://crates.io/crates/hf2-cli
 [__link12]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link13]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link2]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
 [__link3]: https://github.com/alsa-project/alsa-lib
 [__link4]: https://github.com/systemd/systemd
 [__link5]: https://www.rust-lang.org/tools/install
 [__link6]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link7]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link8]: https://crates.io/crates/hf2-cli
 [__link9]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
