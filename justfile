format:
    cargo +nightly fmt --all

lint:
    cargo clippy --all-targets

format-and-lint: format lint

update-deps:
    cargo upgrade -i
    cargo update

test:
    cargo test

run ROM:
    cargo run -p gb-eframe -- --cgb "{{ROM}}"
