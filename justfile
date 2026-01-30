# light_clone development tasks

# List available recipes
default:
    @just --list

# ============================================================================
# Linting & CI
# ============================================================================

# Run all CI checks (compile, fmt, lint, test)
lint-ci: compile fmt-check lint test

# Run clippy lints (matches CI)
lint:
    cargo clippy --workspace --all-features --all-targets -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying (matches CI)
fmt-check:
    cargo fmt --all --check

# Check compilation (fast feedback, matches CI)
compile:
    cargo check --workspace --all-features --all-targets

# ============================================================================
# Testing
# ============================================================================

# Run all tests (quiet output, shows summary + failures)
test:
    cargo test --workspace --all-features --quiet

# Run tests without feature flags
test-minimal:
    cargo test --workspace --quiet

# ============================================================================
# Building
# ============================================================================

# Build all targets
build:
    cargo build --workspace --all-features

# Clean build artifacts
clean:
    cargo clean

# ============================================================================
# Benchmarks
# ============================================================================

# Install benchmark tools (cargo-criterion and criterion-table)
install-bench-tools:
    cargo install cargo-criterion criterion-table

# Generate benchmark table with all features and save to file
bench-full-table-save: check-criterion-table
    cargo criterion -p light_clone --all-features --message-format=json 2>/dev/null | criterion-table > BENCHMARKS.md
    @echo "Saved to BENCHMARKS.md (with all features)"

# Run benchmarks with all features
bench-full:
    cargo bench -p light_clone --all-features

# Run benchmarks (full output)
bench:
    cargo bench -p light_clone

# Generate benchmark comparison table as markdown
bench-table: check-criterion-table
    cargo criterion -p light_clone --message-format=json 2>/dev/null | criterion-table

# Check if cargo-criterion is installed
[private]
check-cargo-criterion:
    @command -v cargo-criterion >/dev/null 2>&1 || (echo "Error: cargo-criterion not found. Install with: cargo install cargo-criterion" && exit 1)

# Check if criterion-table is installed
[private]
check-criterion-table: check-cargo-criterion
    @command -v criterion-table >/dev/null 2>&1 || (echo "Error: criterion-table not found. Install with: cargo install criterion-table" && exit 1)

# ============================================================================
# Documentation
# ============================================================================

# Build documentation
doc:
    cargo doc --workspace --all-features --no-deps

# Open documentation in browser
doc-open:
    cargo doc --workspace --all-features --no-deps --open

