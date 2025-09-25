#!/bin/bash
set -e

prepare_py_module() {
    python3 -m venv .venv
    . .venv/bin/activate
    pip install maturin
    maturin develop -m dissync-kalman-py/Cargo.toml
}

test_rs() {
    cargo test --workspace
}

test_py() {
    prepare_py_module
    python3 -c "import dissync_kalman_py; print(dissync_kalman_py)"
}

check_style() {
    cargo clippy --workspace -- -D warnings
    cargo fmt --all --check
}

format() {
    cargo fmt --all
}

case $1 in

  test-rs)
    test_rs
    ;;

  test-py)
    test_py
    ;;

  test)
    test_rs
    test_py
    ;;

  style)
    check_style
    ;;

  fmt)
    format
    ;;

  all)
    check_style
    test_rs
    test_py
    ;;

  *)
    echo "Unknown command: $1"
    exit 1
    
esac
