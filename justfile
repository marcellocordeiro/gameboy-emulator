default:
  @just --list

format *ARGS:
  cargo +nightly fmt --all {{ARGS}}

[unix]
format-cpp:
  #!/usr/bin/env zsh
  clang-format -i ./apps/**/**.{cpp,hpp}
  clang-format -i ./core/**/**.h

lint *ARGS:
  cargo clippy --all-targets {{ARGS}}
  cargo clippy -p gb-eframe --target wasm32-unknown-unknown {{ARGS}}

fix *ARGS: (lint "--fix" ARGS) format

update:
  cargo upgrade -i
  cargo update
  vcpkg x-update-baseline

test *ARGS:
  cargo test {{ARGS}}

run ROM *ARGS:
  cargo run -p gb-eframe -- {{ARGS}} "{{ROM}}"

run-with-bootrom ROM *ARGS:
  cargo run -p gb-eframe --features bootrom -- {{ARGS}} "{{ROM}}"
