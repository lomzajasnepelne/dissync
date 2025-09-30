#!/bin/bash
set -e

setup_python_env () {
    python3 -m venv .venv
    . .venv/bin/activate
    pip install -U setuptools
    pip install maturin
    pip install poetry
    maturin develop -m dissync-kalman-py/Cargo.toml
    poetry -C dissync-kalman-report install
}

test_rs() {
    cargo test --workspace
}

test_py() {
    . .venv/bin/activate
    poetry -C dissync-kalman-report run mypy .
    poetry -C dissync-kalman-report run pytest
}

check_style() {
    . .venv/bin/activate
    cargo clippy --workspace -- -D warnings
    cargo fmt --all --check
    poetry -C dissync-kalman-report run black . --diff --check
}

format() {
    . .venv/bin/activate
    cargo fmt --all
    poetry -C dissync-kalman-report run black .
}

report() {
  . .venv/bin/activate
  mkdir -p report
  poetry -C dissync-kalman-report run python3 -m dissync_kalman_report ../report/plot.png
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

  report)
    report
    ;;

  all)
    check_style
    test_rs
    test_py
    report
    ;;

  *)
    echo "Unknown command: $1"
    exit 1
    
esac
