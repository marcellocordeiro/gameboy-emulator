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

fix *ARGS: (lint "--fix" ARGS) format

update:
  cargo upgrade -i
  cargo update
  vcpkg x-update-baseline
  just ./apps/web/ update

test:
  cargo test

run ROM *ARGS:
  cargo run -p gb-eframe -- {{ARGS}} "{{ROM}}"
