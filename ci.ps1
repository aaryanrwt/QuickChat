$ErrorActionPreference = "Stop"

Write-Host "Running cargo clean..."
cargo clean

Write-Host "Running cargo fmt..."
cargo fmt --all --check

Write-Host "Running cargo clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

Write-Host "Running cargo check..."
cargo check --workspace

Write-Host "Running cargo build release..."
cargo build --workspace --release

Write-Host "Running cargo test..."
cargo test --workspace

Write-Host "Running cargo doc..."
cargo doc --workspace

Write-Host "CI Replication Complete!"
