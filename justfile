format:
    cargo +nightly fmt --all

lint:
    cargo clippy --all-targets

format-and-lint: lint && format
