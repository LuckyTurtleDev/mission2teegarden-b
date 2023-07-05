# 🪐 Mission to Teegarden b

![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)
[![v0.1.0](https://img.shields.io/badge/version-v0.1.0-orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v0.1.0)
[![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/LuckyTurtleDev/mission2teegarden-b?label=latest&color=orange)](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/latest)
[![Packaging status](https://repology.org/badge/tiny-repos/mission2teegarden-b.svg)](https://repology.org/project/mission2teegarden-b/versions)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/mission2teegarden-b)
![Rust Version: ^1.70](https://img.shields.io/badge/rustc-%5E1.70-orange.svg)

Welcome to a journey to the unexplored planet Teegarden b, to find a new home for humanity. Robots where send to planets surface for explorations. Program the robots from the safety of your space ship, by using your [pybadge][__link0].

<div align="center">
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/2a4af8f4-28a4-4589-99c3-2b18de4de267" width=60%>
</div>
What hazards await you on the planet?
Face them with up to 4 players with up to 4 players.
Work together to solve all puzzles and challenges.
Will you be able to prepare everthing for the arrival of humans?
<div align="center">
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/42d76a9f-3320-427f-bcf0-07887f0fcedf" width=49%>
	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/779ec8f7-6e15-4e2c-b737-b1ad5477d9f2" width=49%>
</div>
Try out Mission to Teegarden b now for free and figure it out.

## Installation (Pc):

Mission to Teegarden b is avaibale at the following repositories:

[![Packaging status][__link1]][__link2]

Prebuild binarys can also been downloaded from the


#### Building from source:

Alternative you can easily build Mission to Teegarden b  by yourself:

 - on Linux install the following development dependencies. At some distros (like Alpine and Debian) seperate development packages exist, regular suffixed with `-dev`. If this the case make sure that you have also installed the `*-dev` version.
	 - [`alsa-lib`][__link3]
	 - [`libudev`][__link4]
	
	
 - [install rust][__link5] and unpack the source code.
 - run `cargo install --path pc --locked` inside the unpacked folder, to build and install the mission2teegarden-b. See the [rust book][__link6] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable, otherwise the `mission2teegarden-b` executeable can not be found.


## Flash Pybadge:

 - Install an UF2 flasher. I recommand using [hf2-cli][__link7].
 - Download and unpack Pybadge binary from
 - Press the reset button of the pybdage twice, to enter the bootloader.
 - After this execute `hf2 elf mission2teegarden-b-pybadge` (or the corresponding command of your flahing tool) to flash the binary to the pybadge.
 - Press the reset button again.


#### Building from source:

Alternative you can build m3 by yourself:

 - [install rustup][__link8]
 - run `cargo install hf2-cli --locked` to build and install the [hf2-cli][__link9] flasher. See the [rust book][__link10] for more information about cargo install.
 - make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable otherwise the executeable can not be found..
 - install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
 - optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf` and unpack the source code (if not already done).
 - press the reset button of the pybadge twice to enter bootloader
 - compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder. `+nightly` is optional and have to be left out if the “install nightly toolchain” step was skip. Please use `+nightly` for bug reports.
 - Press the reset button again.


## Map/Level Editor:

Mission to Teegarden b allow creating custom maps/levels, by using the powerfull [Tiled Map editor][__link11]. See [here][__link12] for more information about creating maps.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGxZ8633wCs_9GzKKlc-jeF26G4eLyZuq8IdiG7yPhHI4iD8_YXKEGwggEmd-11mdG3cjmb6MqXK9G9qi-Jo0hmRHG7e7arcQkqSPYWSBgndtaXNzaW9uMnRlZWdhcmRlbl9iX21hcGUwLjEuMA
 [__link0]: https://www.adafruit.com/product/4200
 [__link1]: https://repology.org/badge/vertical-allrepos/mission2teegarden_b.svg
 [__link10]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link11]: https://www.mapeditor.org/
 [__link12]: https://docs.rs/mission2teegarden_b_map/0.1.0
 [__link2]: https://repology.org/project/mission2teegarden-b/versions
 [__link3]: https://github.com/alsa-project/alsa-lib
 [__link4]: https://github.com/systemd/systemd
 [__link5]: https://www.rust-lang.org/tools/install
 [__link6]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
 [__link7]: https://crates.io/crates/hf2-cli
 [__link8]: https://www.rust-lang.org/tools/install
 [__link9]: https://crates.io/crates/hf2-cli
