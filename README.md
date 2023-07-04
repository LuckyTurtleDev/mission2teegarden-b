# 🪐 Mission to Teegarden b

![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![v0.1.0](https://img.shields.io/badge/version-v0.1.0-orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.0)
[![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/LuckyTurtleDev/mission2teegarden-b?label=latest&color=orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/latest)
[![Packaging status](https://repology.org/badge/tiny-repos/mission2teegarden-b.svg)](https://repology.org/project/mission2teegarden-b/versions)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/mission2teegarden-b)
![Rust Version: ^1.70](https://img.shields.io/badge/rustc-%5E1.70-orange.svg)


## Map/Level Editor:

Mission to Teegarden b allow creating custom maps/levels, by using the powerfull [Tiled Map editor][__link0]. See [here][__link1] for more infos.


## Installation (Pc):

Mission to Teegarden b is avaibale at the following repositories:

[![Packaging status][__link2]][__link3]

Prebuild binarys can also been downloaded from the [Github release][__link4].


#### Building from source:

Alternative you can easily build Mission to Teegarden b  by yourself:

 - on Linux install the following development dependencies. At some distros (like Alpine and Debian) seperate development packages exist, regular suffixed with `-dev`. If this the case make sure that you have also installed the `*-dev` version.
	 - [`alsa-lib`][__link5]
	 - [`libudev`][__link6]
	
	
 - [install rust][__link7]
 - [Download][__link8] and unpack the source code.
 - run `cargo install --path pc --locked` inside the unpacked folder, to build and install the mission2teegarden-b. See the [rust book][__link9] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable, otherwise the `mission2teegarden-b` executeable can not be found.


## Flash Pybadge:

 - Install an UF2 flasher. I recommand using [hf2-cli][__link10].
 - Download and unpack Pybadge binary from [Github release][__link11].
 - Press the reset button of the pybdage twice, to enter the bootloader.
 - After this execute `hf2 elf mission2teegarden-b-pybadge` (or the corresponding command of your flahing tool) to flash the binary to the pybadge.
 - Press the reset button again.


#### Building from source:

Alternative you can build m3 by yourself:

 - [install rustup][__link12]
 - run `cargo install hf2-cli --locked` to build and install the [hf2-cli][__link13] flasher. See the [rust book][__link14] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable otherwise the executeable can not be found..
 - install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
 - optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
 - [Download][__link15] and unpack the source code (if not already done).
 - press the reset button of the pybadge twice to enter bootloader
 - compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder. `+nightly` is optional and have to be left out if the “install nightly toolchain” step was skip. Please use `+nightly` for bug reports.
 - Press the reset button again.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGxZ8633wCs_9GzKKlc-jeF26G4eLyZuq8IdiG7yPhHI4iD8_YXKEG2oEjWAiQVzSGwbQgP9v-iKFGxPC4vLgNSLbG_HLsRNq7H0jYWSBgndtaXNzaW9uMnRlZWdhcmRlbl9iX21hcGUwLjEuMA
 [__link0]: https://www.mapeditor.org/
 [__link1]: https://docs.rs/mission2teegarden_b_map/0.1.0
 [__link10]: https://crates.io/crates/hf2-cli
 [__link11]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
 [__link12]: https://www.rust-lang.org/tools/install
 [__link13]: https://crates.io/crates/hf2-cli
 [__link14]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link15]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link2]: https://repology.org/badge/vertical-allrepos/mission2teegarden_b.svg
 [__link3]: https://repology.org/project/mission2teegarden-b/versions
 [__link4]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
 [__link5]: https://github.com/alsa-project/alsa-lib
 [__link6]: https://github.com/systemd/systemd
 [__link7]: https://www.rust-lang.org/tools/install
 [__link8]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link9]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
