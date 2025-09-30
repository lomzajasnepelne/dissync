from matplotlib.figure import Figure
from pathlib import Path

import dissync_kalman_py as dissync_kalman_py


def plot(a: list[float], b: list[float], path: Path) -> None:
    fig = Figure()
    ax = fig.subplots()
    ax.plot(a)
    ax.plot(b)
    fig.savefig(path)


def generate_report(out_dir: Path) -> bool:
    a = [1.0, 2.0, 3.0]
    b = dissync_kalman_py.filter(a)
    plot(a, b, out_dir)
    return True
