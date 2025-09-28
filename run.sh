#!/bin/bash
set -e

setup_python_env () {
    python3 -m venv .venv
    . .venv/bin/activate
    pip install -U setuptools
    pip install maturin
    pip install poetry
}

prepare_py_module() {
    . .venv/bin/activate
    maturin develop -m dissync-kalman-py/Cargo.toml
    poetry -C dissync-kalman-report install
}

test_rs() {
    cargo test --workspace
}

test_py() {
    prepare_py_module
    poetry -C dissync-kalman-report run mypy .
    poetry -C dissync-kalman-report run pytest
}

check_style() {
    cargo clippy --workspace -- -D warnings
    cargo fmt --all --check
}

format() {
    cargo fmt --all
}

case $1 in

  setup-py)
    setup_python_env
    ;;

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
