# Documentation: https://github.com/casey/just
# Cheat sheet: https://cheatography.com/linux-china/cheat-sheets/justfile/

# List all available scripts
[private]
default:
  @just --list --unsorted

# Init submodules. Warning: may discard changes
[group("configuration")]
init-submodules:
  git submodule update --init --recursive

# Format all crates in the workspace
[group("maintenance")]
format *ARGS:
  cargo +nightly fmt --all {{ARGS}}

# Format C/C++ source files and headers
[unix]
[group("maintenance")]
format-cpp:
  #!/usr/bin/env zsh
  clang-format -i ./apps/**/**.{cpp,hpp}
  clang-format -i ./core/**/**.h

# Lint all crates in the workspace
[group("maintenance")]
lint *ARGS:
  cargo clippy --all-targets {{ARGS}}
  cargo clippy -p gb-eframe --target wasm32-unknown-unknown {{ARGS}}

# Lint and fix all crates in the workspace, then format
[group("maintenance")]
fix *ARGS: (lint "--fix" ARGS) format

# Update all project dependencies (cargo and vcpkg)
[group("maintenance")]
update:
  cargo upgrade -i
  cargo update
  vcpkg x-update-baseline

# Run the eframe app
[group("development")]
run *ARGS:
  cargo run -p gb-eframe -- {{ARGS}}

# Run the eframe app with the bundled bootrom
[group("development")]
run-with-bootrom *ARGS:
  cargo run -p gb-eframe --features bundled-bootrom -- {{ARGS}}

# Run tests for all crates in the workspace
[group("development")]
test *ARGS:
  cargo test {{ARGS}}
