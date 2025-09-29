from pathlib import Path

import dissync_kalman_report


def test_rust_module_is_callable() -> None:
    assert [0,0] == dissync_kalman_report.dissync_kalman_py.filter([0,0])

def test_generate_report(tmp_path: Path) -> None:
    ret = dissync_kalman_report.generate_report(tmp_path)
    assert ret
