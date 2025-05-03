# gameboy-emulator

Experimental Game Boy emulator written in Rust.

There are several frontends available for no reason but to experiment with different languages and frameworks consuming the main core written in Rust. The [`eframe`](platform/eframe/) frontend provides some debugging tools.

**_Disclaimer_**: this emulator is an experimental project for educational purposes. The development and use of emulators is legal, as long as no copyrighted content is illegally obtained. This means you are responsible for dumping your own boot ROMs and games. However, there exists free and open-source content in case you'd like to test this project and/or contribute :blush:

## Features

- [x] CPU
- [x] PPU
- [ ] APU
- [x] Input (native only)
- [x] Cartridge
  - [x] No MBC
  - [x] MBC1
  - [x] MBC2
  - [x] MBC3
    - [x] MBC30
    - [ ] RTC
  - [x] MBC5
- [ ] Saving
- [x] Debugging UI
- [ ] More debugging UI
- [x] Automated ROM tests (failing tests are disabled, typically with a reason)
  - [x] blargg
  - [x] mooneye-test-suite
  - [x] dmg-acid2 (DMG and CGB)
  - [x] cgb-acid2

## Screenshots

boop

## Repository structure

- **[`.`](/)**: Package/configuration files for all modules.
- **[`external`](external/)**: External dependencies.
- **[`core`](core/)**: Core modules.
  - **[`gb-core`](core/gb-core/)**: Main core written in Rust.
  - **[`gb-core-c`](core/gb-core-c/)**: `gb-core` shims for use in other languages. Contains a C/C++ header file with the function declarations.
  - **[`gb-core-swift`](core/gb-core-swift/)**: Contains a Swift package that wraps `gb-core-c` in a Swift class.
  - **[`gb-core-wasm`](core/gb-core-wasm/)**: `gb-core` wrapper targeting WASM.
  - **[`gb-opcode-info`](core/gb-opcode-info/)**: Contains opcode info for use in other modules.
- **[`platform`](platform/)**: Language/framework specific frontends.
  - **[`eframe`](platform/eframe/)**: Native app written in Rust using eframe.
  - **[`sdl3`](platform/sdl3/)**: Native app written in C++ using SDL3 and Dear ImGui.
  - **[`swiftui`](platform/swiftui/)**: Native app written in Swift using SwiftUI.
  - **[`web`](platform/web/)**: Web app written in TypeScript using Vite and React.

## Setup

### Rust

```sh
# Install rustup: https://www.rust-lang.org/
# Can also install rustup from the package manager

rustup default stable
rustup toolchain install nightly # For rustfmt
```

### wasm-pack

Required to build the web app.

```sh
# Install wasm-pack from source
cargo install wasm-pack

# Arch
sudo pacman -S wasm-pack

# macOS
brew install wasm-pack

# npm
npm install -g wasm-pack
```

### SDL2/SDL3

Required to build the C and Swift apps.

```sh
# Arch
sudo pacman -S sdl3

# Fedora
sudo dnf install SDL3-devel

# macOS
brew install sdl3
```

## Building

```sh
# Native
cargo build

# Web app
pnpm build # Implicitly builds the Rust dependencies
```

## Running

```sh
# Native
# Add --release after `run` if debug mode is too slow.
cargo run -- roms/rom.gb

# Native with info logs
RUST_LOG=info cargo run -- roms/rom.gb

# Web app
pnpm install
pnpm dev
```

## Tests

Check [`gb-core/tests`](core/gb-core/tests) for all the supported integration tests. Any failing test is commented out, typically with a reason. Unit tests are also included for some modules. Both the ALU and the instructions are being tested using `sm83-test-data`, so it may take a while for the tests to finish.

```sh
cargo test
```

## todo

- Improve code and repo quality
- APU
- Input (implemented for some frontends. Needs to be improved)
- Saving
- Improve accuracy
  - Implement FIFO fetcher instead of a scanline renderer
- Better debugging UI
- More automated ROM tests
- Pass more tests
- Game Boy Color support (done? ish.)

## References

- [Pan Docs](https://gbdev.io/pandocs/)
- [Game Boy: Complete Technical Reference](https://github.com/Gekkio/gb-ctr)
- [mooneye-gb](https://github.com/Gekkio/mooneye-gb)
- [SameBoy](https://github.com/LIJI32/SameBoy)
- [The Cycle-Accurate Game Boy Docs](https://github.com/geaz/emu-gameboy/blob/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf)
- [CPU opcode reference](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7)
- [A journey into GameBoy emulation](https://robertovaccari.com/blog/2020_09_26_gameboy/)
