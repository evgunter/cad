#!/usr/bin/env python3
"""Reviewer cost by model, paired within each dual review.

Both reviewers in a dual read the SAME code at the SAME time under
identical briefs, so the within-pair ratio is a clean cost comparison.
Cross-model pairs are R1 fable / R2 opus; same-model pairs (both fable)
are the control for any R1-vs-R2 slot asymmetry.
"""
import math
import os
import re

LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                   "..", "..", "docs", "MODEL-AB-LOG.md")
COLS = 14
CROSS = {"M8-14b", "U4A-DOOR", "TESS-SPAN", "RIM", "SSIFLAT", "ASM-DEMO",
         "TUBEWALL", "M9-3", "ONARC", "OFFB", "RESPELL-PR2", "M9-2"}


def scale(n, u):
    return float(n) * 1000 if u.lower() == "m" else float(n)


def load():
    out = []
    for line in open(LOG):
        s = line.strip()
        if not s.startswith("|"):
            continue
        c = [x.strip() for x in s.strip("|").split("|")]
        if len(c) != COLS or not re.search(r"sample #\d+|DUAL", c[5], re.I):
            continue
        tk = {r: scale(n, u)
              for r, n, u in re.findall(r"R([12])\s*~?([0-9.]+)([kKmM])", c[12])}
        wl = {}
        for r, n, u in re.findall(r"R([12])\s*~?([0-9.]+)\s*(h|min)", c[13]):
            wl[r] = float(n) / 60 if u == "min" else float(n)
        out.append((c[0], "cross" if c[0] in CROSS else "same", tk, wl))
    return out


def ratio(rows, kind, idx, label):
    sel = [r for r in rows if r[1] == kind and "1" in r[idx] and "2" in r[idx]]
    if not sel:
        return
    a = [r[idx]["1"] for r in sel]
    b = [r[idx]["2"] for r in sel]
    lr = [math.log(y / x) for x, y in zip(a, b)]
    m = sum(lr) / len(lr)
    sd = (sum((x - m) ** 2 for x in lr) / max(len(lr) - 1, 1)) ** 0.5
    se = sd / len(lr) ** 0.5
    print("  %-6s %-20s n=%-2d  R1 %7.1f  R2 %7.1f   R2/R1 %.2f [%.2f, %.2f]"
          % (kind, label, len(sel), sum(a) / len(a), sum(b) / len(b),
             math.exp(m), math.exp(m - 1.96 * se), math.exp(m + 1.96 * se)))


if __name__ == "__main__":
    rows = load()
    print("cross-model: R1 fable / R2 opus.  same-model: both fable (control)\n")
    for idx, lab in ((2, "tokens (k)"), (3, "wall-clock (h)")):
        ratio(rows, "cross", idx, lab)
        ratio(rows, "same", idx, lab)
        print()
