# gameboy-emulator

Experimental Game Boy emulator written in Rust.

**_Disclaimer_**: this emulator is an experimental and educational project. The development and use of emulators is legal, as long as no copyrighted content is illegally obtained. This means you are responsible for dumping your own bootroms and games. Please note that there are free open-source and homebrew content available.

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
- [x] Automated ROM tests
  - [ ] blargg
  - [x] mooneye-test-suite (failing tests are commented out)
  - [x] dmg-acid2 (DMG only)

## Screenshots

boop

## Repository structure

- **[`.`](/)**: Package/configuration files for all modules.
- **[`external`](external/)**: External dependencies.
- **[`core`](core/)**: Core modules.
  - **[`gb-core`](core/gb-core/)**: Main core written in Rust.
  - **[`gb-core-c`](core/gb-core-c/)**: `gb-core` shims for use in other languages (interoperability). Contains a C/C++ header file with the function declarations.
  - **[`gb-core-wasm`](core/gb-core-wasm/)**: `gb-core` wrapper targeting WASM.
  - **[`gb-opcode-info`](core/gb-opcode-info/)**: Contains opcode info for use in other modules.
- **[`platform`](platform/)**: Language/framework specific frontends.
  - **[`browser`](platform/browser/)**: Browser app written in TypeScript using Vite and React.
  - **[`eframe-rust`](platform/eframe-rust/)**: Native app written in Rust using eframe.
  - **[`sdl2-c`](platform/sdl2-c/)**: Native app written in C using `gb-core-c` and SDL2.
  - **[`sdl2-rust`](platform/sdl2-rust/)**: Native app written in Rust using SDL2.
  - **[`sdl2-swift`](platform/sdl2-swift/)**: Native app written in Swift using `gb-core-c` and SDL2. Likely unsupported on Windows and Linux, but was briefly tested on the latter.

## Setup

### Rust

```sh
# Install rustup: https://www.rust-lang.org/

rustup default stable

# Arch
sudo pacman -S rust rust-src rust-wasm
```

### wasm-pack

Required to build the browser app.

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

### SDL2

Required to build the C and Swift apps.

```sh
# Arch
sudo pacman -S sdl2

# macOS
brew install sdl2
```

## Building

```sh
# Native
cargo build

# Browser app
pnpm build # Will build the Rust dependencies as well

# Swift
cargo build --release
cd gb-ui-sdl2-swift
swift build
```

## Running

```sh
# Native
# Add --release after `run` if debug mode is too slow.
cargo run -- roms/rom.gb

# Native with info logs
RUST_LOG=info cargo run -- roms/rom.gb

# Browser app
pnpm i
pnpm dev

# Swift
cd gb-ui-sdl2-swift
swift run GameBoy ../roms/rom.gb
```

## Tests

Check `gb-core/tests` for some of the supported tests. Most of blargg's tests are supported but not included (yet) in the test suite. `dmg-acid2` passes.

```sh
cargo test
```

## todo

- Improve code and repo quality
- APU
- Input
- Saving
- Improve accuracy
  - Implement FIFO fetcher instead of a scanline renderer
- Better debugging UI
- More automated ROM tests
- Pass more tests
- Game Boy Color support

## References

- [Pan Docs](https://gbdev.io/pandocs/)
- [Game Boy: Complete Technical Reference](https://github.com/Gekkio/gb-ctr)
- [mooneye-gb](https://github.com/Gekkio/mooneye-gb)
- [The Cycle-Accurate Game Boy Docs](https://github.com/geaz/emu-gameboy/blob/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf)
- [CPU opcode reference](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7)
- [A journey into GameBoy emulation](https://robertovaccari.com/blog/2020_09_26_gameboy/)
