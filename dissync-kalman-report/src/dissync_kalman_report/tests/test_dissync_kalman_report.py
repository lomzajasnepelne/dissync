from pathlib import Path
import os

import dissync_kalman_report


def test_rust_module_is_callable() -> None:
    assert [0, 0] == dissync_kalman_report.dissync_kalman_py.filter([0, 0])


def test_generate_report(tmp_path: Path) -> None:
    report_path = tmp_path / "report.png"
    ret = dissync_kalman_report.generate_report(report_path)
    assert ret
    assert os.path.isfile(report_path)
