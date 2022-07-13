alias b := build
alias c := check
alias cl := clippy
alias f := fmt
alias r := run
alias t := test

set positional-arguments := true
set dotenv-load := true

build type="":
    cargo build {{ type }}

check type="":
    cargo check {{ type }}

clippy flags="":
    cargo clippy -- {{ flags }}

fmt:
    cargo fmt

run type="":
    cargo run {{ type }}

test:
    cargo nextest run
