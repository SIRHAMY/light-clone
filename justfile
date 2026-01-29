# lc_clone development tasks

# List available recipes
default:
    @just --list

# Run all tests
test:
    cargo test --workspace --all-features

# Run tests without feature flags
test-minimal:
    cargo test --workspace

# Run clippy lints
lint:
    cargo clippy --workspace --all-features -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Build all targets
build:
    cargo build --workspace --all-features

# Run benchmarks (full output)
bench:
    cargo bench -p lc_clone

# Check if cargo-criterion is installed
[private]
check-cargo-criterion:
    @command -v cargo-criterion >/dev/null 2>&1 || (echo "Error: cargo-criterion not found. Install with: cargo install cargo-criterion" && exit 1)

# Check if criterion-table is installed
[private]
check-criterion-table: check-cargo-criterion
    @command -v criterion-table >/dev/null 2>&1 || (echo "Error: criterion-table not found. Install with: cargo install criterion-table" && exit 1)

# Generate benchmark comparison table as markdown
bench-table: check-criterion-table
    cargo criterion -p lc_clone --message-format=json 2>/dev/null | criterion-table

# Generate benchmark table and save to file
bench-table-save: check-criterion-table
    cargo criterion -p lc_clone --message-format=json 2>/dev/null | criterion-table > BENCHMARKS.md
    @echo "Saved to BENCHMARKS.md"

# Run benchmarks with im feature
bench-im: check-criterion-table
    cargo criterion -p lc_clone --features im --message-format=json 2>/dev/null | criterion-table

# Build documentation
doc:
    cargo doc --workspace --all-features --no-deps

# Open documentation in browser
doc-open:
    cargo doc --workspace --all-features --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Install optional benchmark tools (cargo-criterion and criterion-table)
install-bench-tools:
    cargo install cargo-criterion criterion-table

# Run all checks (test, lint, fmt-check)
check: fmt-check lint test
