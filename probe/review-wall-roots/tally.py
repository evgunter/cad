"""Tally the probe tree's [m11-disc] lines: per predicate, decisions,
undecided at each scalar, and the time inside them. Also [bf-tally] lines
and the hole finish lines if present. Usage: python3 -I tally.py err.txt"""
import re
import sys
from collections import defaultdict

n = defaultdict(int)
und = defaultdict(int)
ms = defaultdict(float)
pat = re.compile(r"\[m11-disc\] name=(\S+) scalar=(\S+) outcome=(\S+) ms=([0-9.]+)")
for line in open(sys.argv[1], errors="replace"):
    m = pat.match(line)
    if not m:
        continue
    name, scalar, outcome, t = m.group(1), m.group(2), m.group(3), float(m.group(4))
    key = (name, scalar)
    n[key] += 1
    ms[key] += t
    if outcome == "Indeterminate":
        und[key] += 1
for key in sorted(n):
    print(f"{key[0]:28s} {key[1]:22s} decisions {n[key]:8d} undecided {und[key]:7d} time {ms[key] / 1e3:8.1f} s")
