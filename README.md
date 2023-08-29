# gameboy-emulator

Experimental Game Boy emulator written in Rust.

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
    - [ ] RTC
  - [x] MBC5
- [ ] Saving
- [x] Debugging UI
- [ ] More debugging UI
- [x] Automated ROM tests
  - [ ] blargg
  - [x] mooneye-test-suite (failing tests are commented out)
  - [ ] dmg-acid2

## Screenshots

boop

## Setup

### Rust

```sh
# Install rustup: https://www.rust-lang.org/

rustup default stable

# Arch
sudo pacman -S rust rust-src rust-wasm
```

### wasm-pack

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

## Building

```sh
cargo build
```

## Running

```sh
# Native
# Add --release after `run` if debug mode is too slow.
cargo run -- roms/rom.gb

# Native with info logs
RUST_LOG=info cargo run -- roms/rom.gb

# Browser
pnpm i
pnpm dev
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
