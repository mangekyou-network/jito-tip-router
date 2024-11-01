#! /bin/zsh

# cargo b && ./target/debug/jito-restaking-cli --markdown-help > ./docs/_tools/00_cli.md && ./target/debug/jito-shank-cli && yarn generate-clients && cargo b
cargo sort --workspace
cargo fmt --all
cargo nextest run --all-features
cargo clippy --all-features -- -D warnings -D clippy::all -D clippy::nursery -D clippy::integer_division -D clippy::arithmetic_side_effects -D clippy::style -D clippy::perf


cargo b && ./target/debug/jito-tip-router-shank-cli && yarn install && yarn generate-clients && cargo b
cargo-build-sbf

