#!/usr/bin/env python3
"""Readout 6: load every dual-review row (front matter + prose cells).

Stdlib only. Usage: readout-6-rows.py [repo-root]. Imported by
readout-6-numbers.py; not part of the log's bookkeeping."""
import os, re, sys

COLS = ["date", "protocol", "unit", "klass", "head", "r1", "r2", "corr", "tally", "fair", "fix"]

def split_cells(line):
    # split on | not escaped as \|
    s = line.strip()
    assert s.startswith("|"), s[:80]
    inner = s[1:-1] if s.endswith("|") else s[1:]   # a truncated row has no closing cell
    parts = re.split(r"(?<!\\)\|", inner)
    return [p.strip() for p in parts]

def load(root):
    d = os.path.join(root, "docs/dual-review-log/rows")
    rows = []
    for name in sorted(os.listdir(d)):
        raw = open(os.path.join(d, name), encoding="utf-8").read()
        _, fm, body = raw.split("---\n", 2)
        f = {}
        for ln in fm.splitlines():
            k, _, v = ln.partition(":")
            f[k.strip()] = v.strip()
        lines = [l for l in body.split("\n") if l.strip()]
        cells = split_cells(lines[0])
        r = dict(name=name, pr=int(name[3:-3]), dr=int(f["dr"]) if f.get("dr") else None,
                 arm=f["arm"], letter=f["letter"], tallied=int(f["tallied"]),
                 major=f["major"] == "true", excluded=f.get("excluded", ""), miss=f["miss"],
                 notes=lines[1:], ncells=len(cells), closed=lines[0].rstrip().endswith("|"))
        for i, c in enumerate(COLS):
            r[c] = cells[i] if i < len(cells) else ""
        r["extra"] = cells[len(COLS):]
        rows.append(r)
    rows.sort(key=lambda r: (r["dr"] or 10**9, r["pr"]))
    return rows

if __name__ == "__main__":
    rows = load(sys.argv[1] if len(sys.argv) > 1 else ".")
    bad = [r for r in rows if r["ncells"] != len(COLS) or not r["closed"]]
    print(len(rows), "rows;", len(bad), "with a cell count other than", len(COLS))
    for r in bad:
        print("  ", r["dr"], r["name"], r["ncells"], "closed" if r["closed"] else "UNCLOSED")
