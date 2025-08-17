# gameboy-emulator

Experimental Game Boy emulator written in Rust.

There are several frontends available, based on the same core written in Rust. Why? Because why not!
The main frontend uses [eframe/egui](https://github.com/emilk/egui) and [cpal](https://github.com/RustAudio/cpal),
and provides some simple debugging tools.

**Disclaimer**: this emulator is an experimental project for educational purposes,
and no copyrighted ROMs or boot ROMs are being included in this repository.
You're responsible for dumping your own binaries.

- [gameboy-emulator](#gameboy-emulator)
  - [Features](#features)
  - [Screenshots](#screenshots)
  - [Repository structure](#repository-structure)
  - [Setup](#setup)
    - [TL;DR](#tldr)
    - [Rust](#rust)
    - [wasm-pack](#wasm-pack)
    - [SDL3](#sdl3)
  - [Building](#building)
  - [Running](#running)
  - [Tests](#tests)
  - [todo](#todo)
  - [References](#references)

## Features

- [x] CPU
- [x] PPU
- [x] APU
- [x] Input
- [x] Cartridge
  - [x] No MBC
  - [x] MBC1
  - [x] MBC2
  - [x] MBC3
    - [x] MBC30
    - [ ] RTC
  - [x] MBC5
- [x] Saving
- [ ] Save states
- [x] Debugging UI
- [ ] More debugging UI
- [x] Automated ROM tests (failing tests are disabled)
  - [x] blargg
  - [x] mooneye-test-suite
  - [x] dmg-acid2 (DMG and CGB)
  - [x] cgb-acid2

## Screenshots

boop

## Repository structure

- [`apps`](apps): Frontends in different languages and frameworks
  - [`eframe`](apps/eframe-web): App written in Rust using eframe. Targets native and web
  - [`libretro`](apps/libretro): libretro core written in Rust
  - [`sdl3`](apps/sdl3): App written in C++ using SDL3 and Dear ImGui
  - [`swift`](apps/swift)
    - [`GameBoy`](apps/swift/GameBoy): App written in Swift using SwiftUI
    - [`GameBoyCore`](apps/swift/GameBoyCore): Swift package wrapping `gb-core-c`
- [`core`](core): Core modules
  - [`gb-core`](core/gb-core): Main core written in Rust
  - [`gb-core-c`](core/gb-core-c): Generates a C static library. Contains a C/C++ header file with
    the function declarations
  - [`gb-opcode-info`](core/gb-opcode-info): Contains opcode info for use in other modules
- [`external`](external): External dependencies

## Setup

### TL;DR

```shell
cargo run -- ./path/to/rom
```

Or, with [just](https://github.com/casey/just)

```shell
just run ./path/to/rom
```

### Rust

```shell
# Install rustup: https://www.rust-lang.org/
# Can also install rustup from the package manager

rustup default stable
rustup toolchain install nightly # For rustfmt
```

### Trunk

Required to build the web app.
For more information, check the [Trunk docs](https://trunkrs.dev/guide/installation).

### SDL3

Required to build the C++ app.

```shell
# Arch
sudo pacman -S sdl3

# Fedora
sudo dnf install SDL3-devel

# macOS
brew install sdl3
```

## Building

```shell
# Native
cargo build

# Web app
cd apps/eframe
trunk build
```

## Running

```shell
# Native
# Add --release after `run` if debug mode is too slow.
cargo run -- roms/rom.gb

# Native with info logs
RUST_LOG=info cargo run -- roms/rom.gb

# Web app
cd apps/eframe
trunk serve
```

## Tests

Check [`gb-core/tests`](core/gb-core/tests) for all the supported integration tests.
Failing tests are disabled. Unit tests are also included for some modules.
Both the ALU and the instructions are being tested using `sm83-test-data`, so it may take a while for the tests to
finish.

```shell
cargo test
```

## todo

- Improve code and repo quality
- Input (implemented for some frontends. Needs to be improved)
- Save states
- Improve accuracy
  - Implement FIFO fetcher instead of a scanline renderer
- Better debugging UI
- More automated ROM tests
- Pass more tests

## References

- [gbdev.io's Pan Docs](https://gbdev.io/pandocs/)
- [Game Boy: Complete Technical Reference](https://github.com/Gekkio/gb-ctr)
- [mooneye-gb](https://github.com/Gekkio/mooneye-gb)
- [SameBoy](https://github.com/LIJI32/SameBoy)
- [The Cycle-Accurate Game Boy Docs](https://github.com/geaz/emu-gameboy/blob/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf)
- [CPU opcode reference](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7)
- [A journey into GameBoy emulation](https://robertovaccari.com/blog/2020_09_26_gameboy/)
