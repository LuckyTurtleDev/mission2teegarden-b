# 🪐 Mission to Teegarden b

![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![v0.1.1](https://img.shields.io/badge/version-v0.1.1-orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.1)
[![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/LuckyTurtleDev/mission2teegarden-b?label=latest&color=orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/latest)
[![Packaging status](https://repology.org/badge/tiny-repos/mission2teegarden-b.svg)](https://repology.org/project/mission2teegarden-b/versions)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/mission2teegarden-b)
![Rust Version: ^1.70](https://img.shields.io/badge/rustc-%5E1.70-orange.svg)

Welcome to a journey to the unexplored planet Teegarden b, to find a new home for humanity. Robots were sent to the planet’s surface for exploration. Program these robots from the safety of your spaceship, by using your [pybadge][__link0].

<div align="center">
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/2a4af8f4-28a4-4589-99c3-2b18de4de267" width=60%>
</div>
What hazards await you on the planet?
Face them with up to 4 players.
Work together to solve all puzzles and challenges.
Will you be able to prepare everything, so humans can arrive on the planet?
<div align="center">
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/71af7810-5927-4d05-be75-9ca37617c411" width=49%>
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/779ec8f7-6e15-4e2c-b737-b1ad5477d9f2" width=49%>
</div>
Try out Mission to Teegarden b now for free and figure it out.

## Installation (Pc):

Mission to Teegarden b is available at the following repositories:

[![Packaging status][__link1]][__link2]

Prebuild binaries can also be downloaded from the [GitHub release][__link3].

Mission to Teegarden b is only tested on Arch Linux and Ubuntu. MacOS and Windows versions complie sucessfull but are untested. Supressing standby on MacOS is temporary disable, see [#157][__link4].


#### Building from source:

Alternative you can easily build Mission to Teegarden b  by yourself:

 - On Linux, install the following development dependencies. On some distros (like Alpine and Debian), separate development packages exist, regular suffixed with `-dev`. If this is the case, make sure that you have also installed the `*-dev` version.
	 - [`alsa-lib`][__link5]
	 - [`libudev`][__link6]
	
	
 - [Install rust][__link7]
 - [Download][__link8] and unpack the source code.
 - Run `cargo install --path pc --locked` inside the unpacked folder, to build and install mission2teegarden-b. See the [rust book][__link9] for more information about cargo install.
 - Make sure that `~/.cargo/bin` is listed in the `PATH` environment variable otherwise, the `mission2teegarden-b` executable can not be found.


## Flash Pybadge:

 - Install an UF2 flasher. I recommend using [hf2-cli][__link10].
 - Download and unpack Pybadge binary from [GitHub release][__link11].
 - Press the reset button of the pybdage twice, to enter the bootloader.
 - After this, execute `hf2 elf mission2teegarden-b-pybadge.elf` (or the corresponding command of your flashing tool) to flash the binary to the pybadge.
 - Press the reset button again.


#### Building from source:

Alternative you can build m3 by yourself:

 - [Install rustup][__link12]
 - Install [hf2-cli][__link13] flasher. See the [rust book][__link14] for more information about cargo install.
 - Make sure that `~/.cargo/bin` is listed at the `PATH` environment variable, otherwise the executeable can not be found..
 - Install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
 - Optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
 - [Download][__link15] and unpack the source code (if not already done).
 - Press the reset button of the pybadge twice to enter bootloader
 - Compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder. `+nightly` is optional and have to be left out if the “install nightly toolchain” step was skip. Please use `+nightly` for bug reports.
 - Press the reset button again.


## Map/Level Editor:

Mission to Teegarden b allow creating custom maps/levels, by using the powerfull [Tiled Map editor][__link16]. See [here][__link17] for more information about creating maps.

<div align="center">
		<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/68403ebd-ce64-4baa-bba2-b52962b89d5c" width=80%>
 </div>

 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyFeA8xDJNVnGxf23IOvNAysG80feHOCOwZiGzoQhbxoYmBeYXKEG8Sfv-I6j0i5G5iSOXEkkSjJG-uP4br6AkBGG5VrI0C4iF1fYWSBgndtaXNzaW9uMnRlZWdhcmRlbl9iX21hcGUwLjIuMA
 [__link0]: https://www.adafruit.com/product/4200
 [__link1]: https://repology.org/badge/vertical-allrepos/mission2teegarden-b.svg
 [__link10]: https://crates.io/crates/hf2-cli
 [__link11]: https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.1
 [__link12]: https://www.rust-lang.org/tools/install
 [__link13]: https://crates.io/crates/hf2-cli
 [__link14]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link15]: https://github.com/LuckyTurtleDev/mission2teegarden-b/archive/refs/tags/v0.1.1.zip
 [__link16]: https://www.mapeditor.org/
 [__link17]: https://docs.rs/mission2teegarden_b_map/0.2.0
 [__link2]: https://repology.org/project/mission2teegarden-b/versions
 [__link3]: https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.1
 [__link4]: https://github.com/LuckyTurtleDev/mission2teegarden-b/issues/157
 [__link5]: https://github.com/alsa-project/alsa-lib
 [__link6]: https://github.com/systemd/systemd
 [__link7]: https://www.rust-lang.org/tools/install
 [__link8]: https://github.com/LuckyTurtleDev/mission2teegarden-b/archive/refs/tags/v0.1.1.zip
 [__link9]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
