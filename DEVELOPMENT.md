# Assorted development notes

## TODO

- Take a look at the `log` and `env_logger` crates to improve the logging.
- Study Rust's macros to improve the code.
- Study Rust's explicit lifetime to avoid nesting the components.
- Improve the FFI code.
- Lock access to some registers after the bootrom is finished (CGB).
- Add more linter rules

## Linting

TODO: Add more rules from both `clippy` and `rustc`.

```sh
cargo clippy
```

## Profiling

https://github.com/flamegraph-rs/flamegraph

```sh
# Arch
sudo pacman -S cargo-flamegraph

cargo flamegraph --dev -- roms/rom.gb
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- roms/rom.gb
```
