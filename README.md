# 🪐 Mission to Teegarden b

![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![v0.1.0](https://img.shields.io/badge/version-v0.1.0-orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.0)
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


#### Building from source:

Alternative you can easily build Mission to Teegarden b  by yourself:

 - On Linux, install the following development dependencies. On some distros (like Alpine and Debian), separate development packages exist, regular suffixed with `-dev`. If this is the case, make sure that you have also installed the `*-dev` version.
	 - [`alsa-lib`][__link4]
	 - [`libudev`][__link5]
	
	
 - [Install rust][__link6]
 - [Download][__link7] and unpack the source code.
 - Run `cargo install --path pc --locked` inside the unpacked folder, to build and install mission2teegarden-b. See the [rust book][__link8] for more information about cargo install.
 - Make sure that `~/.cargo/bin` is listed in the `PATH` environment variable otherwise, the `mission2teegarden-b` executable can not be found.


## Flash Pybadge:

 - Install an UF2 flasher. I recommend using [hf2-cli][__link9].
 - Download and unpack Pybadge binary from [GitHub release][__link10].
 - Press the reset button of the pybdage twice, to enter the bootloader.
 - After this, execute `hf2 elf mission2teegarden-b-pybadge` (or the corresponding command of your flashing tool) to flash the binary to the pybadge.
 - Press the reset button again.


#### Building from source:

Alternative you can build m3 by yourself:

 - [Install rustup][__link11]
 - Run `cargo install hf2-cli --locked` to build and install the [hf2-cli][__link12] flasher. See the [rust book][__link13] for more information about cargo install.
 - Make sure that `~/.cargo/bin` is listed at the `PATH` environment variable, otherwise the executeable can not be found..
 - Install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
 - Optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
 - [Download][__link14] and unpack the source code (if not already done).
 - Press the reset button of the pybadge twice to enter bootloader
 - Compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder. `+nightly` is optional and have to be left out if the “install nightly toolchain” step was skip. Please use `+nightly` for bug reports.
 - Press the reset button again.


## Map/Level Editor:

Mission to Teegarden b allow creating custom maps/levels, by using the powerfull [Tiled Map editor][__link15]. See [here][__link16] for more information about creating maps.

<div align="center">
		<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/68403ebd-ce64-4baa-bba2-b52962b89d5c" width=80%>
 </div>
 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGxZ8633wCs_9GzKKlc-jeF26G4eLyZuq8IdiG7yPhHI4iD8_YXKEGxvALrpccUwnG3LtwJOv_IT2G96ptTVep6W9G7qKrdEKLREhYWSBgndtaXNzaW9uMnRlZWdhcmRlbl9iX21hcGUwLjEuMA
 [__link0]: https://www.adafruit.com/product/4200
 [__link1]: https://repology.org/badge/vertical-allrepos/mission2teegarden_b.svg
 [__link10]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
 [__link11]: https://www.rust-lang.org/tools/install
 [__link12]: https://crates.io/crates/hf2-cli
 [__link13]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link14]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link15]: https://www.mapeditor.org/
 [__link16]: https://docs.rs/mission2teegarden_b_map/0.1.0
 [__link2]: https://repology.org/project/mission2teegarden-b/versions
 [__link3]: https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v0.1.0
 [__link4]: https://github.com/alsa-project/alsa-lib
 [__link5]: https://github.com/systemd/systemd
 [__link6]: https://www.rust-lang.org/tools/install
 [__link7]: https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v0.1.0.zip
 [__link8]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link9]: https://crates.io/crates/hf2-cli
