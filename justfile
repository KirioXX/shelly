# Default recipe list when running `just`
default:
    @just --list

# Build release binary
build:
    cargo build --release

# Build and install to cargo's bin directory (respects $CARGO_HOME)
install: build
    @CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin" && \
        cp target/release/shelly "$CARGO_BIN/shelly" && \
        echo "✓ Installed shelly to $CARGO_BIN/shelly"

# Run cargo check
check:
    cargo check

# Format all code
fmt:
    cargo fmt

# Run clippy
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Build and run (dev build)
run *ARGS:
    cargo run -- {{ARGS}}
