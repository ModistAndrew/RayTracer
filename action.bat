REM test the github action
@echo off
cargo fmt && (
    echo cargo fmt succeeded
) || (
    echo cargo fmt failed
    exit /b 1
)

cargo test --all-features && (
    echo cargo test succeeded
) || (
    echo cargo test failed
    exit /b 1
)

cargo fmt -- --check && (
    echo cargo fmt check succeeded
) || (
    echo cargo fmt check failed
    exit /b 1
)

cargo clippy --all-targets --all-features -- -D warnings && (
    echo cargo clippy succeeded
) || (
    echo cargo clippy failed
    exit /b 1
)

cargo build --release --all-features && (
    echo cargo build succeeded
) || (
    echo cargo build failed
    exit /b 1
)

cargo run --release && (
    echo cargo run succeeded
) || (
    echo cargo run failed
    exit /b 1
)