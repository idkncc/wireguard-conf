CFLAGS := ""

default:
    just --list

test:
    cargo test --all-features {{CFLAGS}}
test-no-capture:
    cargo test --all-features {{CFLAGS}} -- --nocapture

build:
    cargo build {{CFLAGS}}

alias doc := docs
docs:
    cargo doc {{CFLAGS}}
docs-open:
    cargo doc {{CFLAGS}} --open

fmt:
    cargo fmt {{CFLAGS}}
lint:
    cargo clippy --all-features {{CFLAGS}}
lint-fix:
    cargo clippy --all-features --fix --allow-dirty {{CFLAGS}}
