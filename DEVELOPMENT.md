# Development notes

## TODO

- Take a look at the `log` and `env_logger` crates to improve the logging.
- Study Rust's macros to improve the code.
- Study Rust's explicit lifetime to avoid nesting the components.
- Improve the FFI code.
- Lock access to some registers after the bootrom is finished (CGB).
- Add more linter rules

## Formatting and linting

TODO: Add more rules from both `clippy` and `rustc`.

```sh
cargo +nightly fmt --all
cargo clippy --all-targets

cargo +nightly fmt --all && cargo clippy --all-targets
```

## Profiling

https://github.com/flamegraph-rs/flamegraph

```sh
# cargo
cargo install flamegraph

# Arch
sudo pacman -S cargo-flamegraph

cargo flamegraph --dev -- roms/rom.gb
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- roms/rom.gb
```

## Upgrading packages

```sh
# cargo
cargo install cargo-edit

# brew
brew install cargo-edit

# Arch
sudo pacman -S cargo-edit

cargo upgrade -i && cargo update
```

## Useful commands

```sh
cargo build --all-targets && cargo build --release --all-targets
```
