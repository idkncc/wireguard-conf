CFLAGS := ""

default:
    just --list

test:
    cargo test {{CFLAGS}}
test-no-capture:
    cargo test {{CFLAGS}} -- --nocapture

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
    cargo clippy {{CFLAGS}}
lint-fix:
    cargo clippy --fix --allow-dirty {{CFLAGS}}
