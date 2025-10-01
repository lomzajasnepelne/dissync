from matplotlib.figure import Figure
from pathlib import Path

import dissync_kalman_py as dissync_kalman_py


def plot(raw: list[float], filtered: list[float], path: Path) -> None:
    fig = Figure()
    ax = fig.subplots()
    ax.plot(raw, label="input")
    ax.plot(filtered, label="filtered")
    ax.set_xlabel("step")
    ax.set_ylabel("value")
    ax.set_title("Kalman filter")
    fig.savefig(path)


def generate_report(out_path: Path) -> bool:
    a = [1.0, 2.0, 3.0]
    b = dissync_kalman_py.filter(a)
    plot(a, b, out_path)
    return True
