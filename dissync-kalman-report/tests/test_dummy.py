import dissync_kalman_report

def test_rust_module_is_callable() -> None:
    assert [0,0] == dissync_kalman_report.dissync_kalman_py.filter([0,0])
