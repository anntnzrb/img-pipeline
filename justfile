# prints this menu
default:
    @just --list

# format source tree
fmt:
    treefmt

# clean project via 'cargo clean'
clean:
    cargo clean
    cargo clean --release

# run project via 'cargo run'
run *ARGS:
    cargo run --quiet -- {{ARGS}}

# watch project
watch:
    bacon --job run

# run clippy lints
clippy:
    bacon --job clippy

# release build
release:
    cargo build --quiet --release
