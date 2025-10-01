import argparse
import sys
from pathlib import Path


import dissync_kalman_report


parser = argparse.ArgumentParser()
parser.add_argument("out_path")
args = parser.parse_args()
path = Path(args.out_path).resolve()
ret = 0 if dissync_kalman_report.generate_report(path) else 1
sys.exit(ret)
