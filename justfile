default:
  @just --list

format:
  cargo +nightly fmt --all

[unix]
format-cpp:
  #!/usr/bin/env zsh
  clang-format -i ./apps/**/**.{cpp,hpp}
  clang-format -i ./core/**/**.h

lint:
  cargo clippy --all-targets

format-and-lint: format lint

update:
  cargo upgrade -i
  cargo update
  vcpkg x-update-baseline
  just ./apps/web/ update

test:
  cargo test

run ROM *ARGS:
  cargo run -p gb-eframe -- {{ARGS}} "{{ROM}}"
