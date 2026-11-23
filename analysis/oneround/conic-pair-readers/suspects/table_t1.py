#!/usr/bin/env python3
"""Task 1 tables from the judged runs: per door/family/variant and shape, by semi-axis:
r right, R refused, W WRONG, G definite in the gap, P a meeting reported past K eps from a curve."""

import json
import sys
import collections

code = {"right": "r", "refused": "R", "WRONG": "W", "GAPDEF": "G", "WRONG-POINT": "P"}
for path in sys.argv[1:]:
    rows = json.load(open(path))
    eps = rows[0]["eps"]
    sizes = sorted({r["a"] for r in rows})
    print(
        f"\n===== eps {eps}   (columns: a = "
        + " ".join(f"{a:.0e}" for a in sizes)
        + ";  each cell: s = 0, 0.3, -3 m of arc from the anchor)"
    )
    groups = collections.OrderedDict()
    for r in rows:
        groups.setdefault(
            (r["door"], r["fam"], r["var"], r["shape"]), collections.defaultdict(dict)
        )[r["a"]][r["s"]] = r
    for (door, fam, var, shape), by in groups.items():
        cells = []
        truths = set()
        for a in sizes:
            cell = ""
            for s in sorted(by[a], key=lambda s: (s != 0.0, s < 0)):
                r = by[a][s]
                cell += code[r["verdict"]]
                truths.add(r["truth"])
            cells.append(cell)
        flag = " <<<" if any(c in "".join(cells) for c in "WGP") else ""
        print(
            f"{door[:5]:5s} {fam} {var:16s} {shape:4s} truth {'/'.join(sorted(truths)):12s} | "
            + " ".join(f"{c:3s}" for c in cells)
            + flag
        )
