# Build Cleaner Makefile
# Complete workflow for development, testing, and installation

BINARY_NAME := build-cleaner
CARGO := cargo
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

# Installation directories
LOCAL_BIN := $(HOME)/bin
SYSTEM_BIN := /usr/local/bin

# Build targets
.PHONY: all build release check test clean install install-local uninstall help dev docs serve

# Default target
all: build

# Development build (debug)
build:
	$(CARGO) build

# Release build (optimized)
release:
	$(CARGO) build --release

# Check code without building
check:
	$(CARGO) check

# Run all tests
test:
	$(CARGO) test

# Run clippy linter
lint:
	$(CARGO) clippy -- -D warnings

# Format code
fmt:
	$(CARGO) fmt

# Check formatting
fmt-check:
	$(CARGO) fmt -- --check

# Clean build artifacts
clean:
	$(CARGO) clean

# Full clean including Cargo.lock
distclean: clean
	rm -f Cargo.lock

# Install to system bin (requires sudo)
install: release
	@echo "Installing $(BINARY_NAME) v$(VERSION) to $(SYSTEM_BIN)..."
	sudo cp target/release/$(BINARY_NAME) $(SYSTEM_BIN)/$(BINARY_NAME)
	@echo "Installed successfully!"
	@echo "Run '$(BINARY_NAME) --version' to verify."

# Install to user's ~/bin (no sudo required)
install-local: release
	@echo "Installing $(BINARY_NAME) v$(VERSION) to $(LOCAL_BIN)..."
	@mkdir -p $(LOCAL_BIN)
	cp target/release/$(BINARY_NAME) $(LOCAL_BIN)/$(BINARY_NAME)
	@echo ""
	@echo "Installed successfully to $(LOCAL_BIN)/$(BINARY_NAME)"
	@echo ""
	@if echo "$$PATH" | grep -q "$(LOCAL_BIN)"; then \
		echo "Run '$(BINARY_NAME) --version' to verify."; \
	else \
		echo "NOTE: $(LOCAL_BIN) is not in your PATH."; \
		echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"; \
		echo ""; \
		echo "  export PATH=\"\$$HOME/bin:\$$PATH\""; \
		echo ""; \
		echo "Then restart your shell or run: source ~/.zshrc"; \
	fi

# Uninstall from system bin
uninstall:
	@echo "Removing $(BINARY_NAME) from $(SYSTEM_BIN)..."
	sudo rm -f $(SYSTEM_BIN)/$(BINARY_NAME)
	@echo "Uninstalled."

# Uninstall from local bin
uninstall-local:
	@echo "Removing $(BINARY_NAME) from $(LOCAL_BIN)..."
	rm -f $(LOCAL_BIN)/$(BINARY_NAME)
	@echo "Uninstalled."

# Development workflow: check, lint, test
dev: check lint test
	@echo "All checks passed!"

# Run the web UI in development mode
serve: build
	$(CARGO) run -- --serve

# Run with a specific directory
run: build
	$(CARGO) run -- $(ARGS)

# Build documentation (requires Jekyll)
docs:
	@echo "Building documentation..."
	cd docs && bundle exec jekyll build

# Serve documentation locally
docs-serve:
	@echo "Serving documentation at http://localhost:4000..."
	cd docs && bundle exec jekyll serve

# Show version
version:
	@echo "$(BINARY_NAME) v$(VERSION)"

# CI workflow: full validation
ci: fmt-check check lint test
	@echo "CI checks passed!"

# Prepare for release
release-prep: ci release
	@echo ""
	@echo "Release build ready!"
	@echo "Binary: target/release/$(BINARY_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Size: $$(du -h target/release/$(BINARY_NAME) | cut -f1)"

# Help
help:
	@echo "Build Cleaner Makefile"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Build targets:"
	@echo "  build         Build debug binary"
	@echo "  release       Build optimized release binary"
	@echo "  check         Check code without building"
	@echo "  clean         Remove build artifacts"
	@echo "  distclean     Remove build artifacts and Cargo.lock"
	@echo ""
	@echo "Quality targets:"
	@echo "  test          Run all tests"
	@echo "  lint          Run clippy linter"
	@echo "  fmt           Format code"
	@echo "  fmt-check     Check code formatting"
	@echo "  dev           Run check, lint, and test"
	@echo "  ci            Full CI validation"
	@echo ""
	@echo "Installation targets:"
	@echo "  install       Install to $(SYSTEM_BIN) (requires sudo)"
	@echo "  install-local Install to $(LOCAL_BIN) (no sudo)"
	@echo "  uninstall     Remove from $(SYSTEM_BIN)"
	@echo "  uninstall-local Remove from $(LOCAL_BIN)"
	@echo ""
	@echo "Run targets:"
	@echo "  serve         Run web UI dashboard"
	@echo "  run ARGS=...  Run with arguments (e.g., make run ARGS='--dry-run')"
	@echo ""
	@echo "Documentation targets:"
	@echo "  docs          Build Jekyll documentation"
	@echo "  docs-serve    Serve documentation locally"
	@echo ""
	@echo "Other targets:"
	@echo "  version       Show current version"
	@echo "  release-prep  Prepare release (ci + release build)"
	@echo "  help          Show this help"
