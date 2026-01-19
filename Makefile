.PHONY: test build run clean fmt clippy check help
# Default target
help:
	@echo "Available commands:"
	@echo "  make test    - Run all tests"
	@echo "  make build   - Build the project"
	@echo "  make run     - Run the project"
	@echo "  make fmt     - Format code with rustfmt"
	@echo "  make clippy  - Run clippy linter"
	@echo "  make check   - Run fmt + clippy + test"
	@echo "  make clean   - Remove build artifacts"
# Run tests
test:
	cargo test
# Build the project
build:
	cargo build
# Build release version
build-release:
	cargo build --release
# Run the project
run:
	cargo run
# Format code
fmt:
	cargo fmt
# Check formatting without modifying
fmt-check:
	cargo fmt -- --check
# Run clippy (Rust linter)
clippy:
	cargo clippy -- -D warnings
# Full check: format, clippy, and test
check: fmt clippy test
	@echo "✅ All checks passed!"
# Clean build artifacts
clean:
	cargo clean