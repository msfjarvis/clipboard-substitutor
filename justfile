alias b := build
alias c := check
alias cl := clippy
alias f := fmt
alias r := run
alias t := test

set positional-arguments := true
set dotenv-load := true

_default:
    just --list

build *args:
    cargo build {{ args }}

check *args:
    cargo check {{ args }}

clippy *args:
    cargo clippy {{ args }}

fmt:
    cargo fmt

log:
    journalctl --user -xeu clipboard-substitutor.service

run type="":
    cargo run {{ type }}

start:
    systemctl --user start clipboard-substitutor.service

status:
    systemctl --user status clipboard-substitutor.service

stop:
    systemctl --user stop clipboard-substitutor.service

test *args:
    cargo nextest run {{ args }}
