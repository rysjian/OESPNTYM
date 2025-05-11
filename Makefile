# Makefile for OESPNTYM project

# Variables
CARGO = cargo
PROJECT_NAME = oespntym
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug
LOG_FILE = playlist_processor.log
MERGED_PLAYLIST = merged_playlist.m3u8

.PHONY: all build release debug test clean run fmt clippy check log

all: release

# Build in release mode
release:
	@echo "Building in release mode..."
	@$(CARGO) build --release
	@echo "Build complete. Binary available at $(RELEASE_DIR)/$(PROJECT_NAME)"

# Build in debug mode (default)
debug:
	@echo "Building in debug mode..."
	@$(CARGO) build
	@echo "Build complete. Binary available at $(DEBUG_DIR)/$(PROJECT_NAME)"

# Run tests
test:
	@echo "Running tests..."
	@$(CARGO) test
	@echo "Tests completed."

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@$(CARGO) clean
	@rm -f $(LOG_FILE) $(MERGED_PLAYLIST)
	@echo "Clean complete."

# Run the program
run: debug
	@echo "Running program..."
	@RUST_LOG=info $(DEBUG_DIR)/$(PROJECT_NAME)

# Run in release mode
run-release: release
	@echo "Running program in release mode..."
	@RUST_LOG=info $(RELEASE_DIR)/$(PROJECT_NAME)

# Format code
fmt:
	@echo "Formatting code..."
	@$(CARGO) fmt
	@echo "Formatting complete."

# Run clippy linter
clippy:
	@echo "Running clippy..."
	@$(CARGO) clippy --all-targets --all-features -- -D warnings
	@echo "Clippy check complete."

# Check code without compiling
check:
	@echo "Checking code..."
	@$(CARGO) check
	@echo "Check complete."

# View log file
log:
	@echo "Displaying log file..."
	@test -f $(LOG_FILE) && cat $(LOG_FILE) || echo "No log file found."
	@echo ""

# View merged playlist
playlist:
	@echo "Displaying merged playlist..."
	@test -f $(MERGED_PLAYLIST) && cat $(MERGED_PLAYLIST) || echo "No playlist file found."
	@echo ""

# Install dependencies (run once)
install:
	@echo "Installing required Rust toolchain..."
	@rustup update
	@rustup component add rustfmt
	@rustup component add clippy
	@echo "Toolchain setup complete."