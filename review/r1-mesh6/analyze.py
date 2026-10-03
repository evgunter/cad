#!/usr/bin/env python3
"""Reduce the in-binary A/B rounds under `ab/` to one min-over-rounds table.

Adopted from the R1 review of PR 1545. Two changes made on adoption, both
disclosed in the PR: the data directory is derived from this file's own
location (it was hard-coded to the reviewer's lane, so the script could not
run anywhere else), and the body is spelled to pass this repo's ruff gate.
The arithmetic is unchanged.

Usage: analyze.py [tag]   # tag is one of def, e6, e12
"""

import collections
import glob
import re
import sys
from pathlib import Path

DATA = Path(__file__).resolve().parent / "ab"
ROW = re.compile(r"S65-COST (\S+) (\S+) (\d+) ([\d.]+) ([\d.]+) ([\d.]+)")
MODES = ["none", "seam", "chord", "both", "check"]


def main() -> None:
    tag = sys.argv[1] if len(sys.argv) > 1 else "def"
    rows = collections.defaultdict(lambda: collections.defaultdict(list))
    chk = collections.defaultdict(list)
    for path in sorted(glob.glob(f"{DATA}/{tag}_r*_*.txt")):
        mode = path.rsplit("_", 1)[1][:-4]
        with open(path) as handle:
            for line in handle:
                m = ROW.match(line)
                if not m:
                    continue
                body, delta, tris, tess, _check, pct = m.groups()
                rows[(body, delta, tris)][mode].append(float(tess))
                if mode == "none":
                    chk[(body, delta, tris)].append(float(pct))
    if not rows:
        print(f"no rounds found for tag={tag} under {DATA}")
        return
    first = rows[next(iter(rows))]
    counts = ", ".join(f"{m}={len(first[m])}" for m in MODES)
    print(f"tag={tag}  rounds per mode: {counts}")
    header = (
        f"{'body':13}{'d':7}{'tris':>7} {'today_us':>9} {'+seam%':>7} "
        f"{'+chord%':>8} {'+both%':>7} {'+check%':>8} {'chk/tess%':>9} "
        f"{'noise(none)%':>12}"
    )
    print(header)
    for (body, delta, tris), modes in rows.items():
        base = min(modes["none"])
        seam = min(modes["seam"])
        chord = min(modes["chord"])
        both = min(modes["both"])
        check = min(modes["check"])
        noise = (max(modes["none"]) - base) / base * 100
        print(
            f"{body:13}{delta:7}{tris:>7} {base:9.1f} "
            f"{(base - seam) / seam * 100:7.1f} "
            f"{(base - chord) / chord * 100:8.1f} "
            f"{(base - both) / both * 100:7.1f} "
            f"{(check - base) / base * 100:8.1f} "
            f"{min(chk[(body, delta, tris)]):9.1f} {noise:12.1f}"
        )


if __name__ == "__main__":
    main()
