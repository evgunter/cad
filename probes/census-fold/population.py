#!/usr/bin/env python3
"""Population by crate and by op family (module-name keywords): population.py <dir>"""
import collections, re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, reduce, suite_of
from tables import fixture

FAMILIES = [("boolean", r"bool|union|subtract|intersect|join|pinch|kiss|cut"),
            ("split", r"split|cleave|section"),
            ("loft", r"loft|skin"),
            ("blend", r"blend|fillet|chamfer|round"),
            ("shell", r"shell|hollow|offset"),
            ("sweep/extrude/revolve", r"sweep|extrude|revolve|helical|tube|prism|ring|wall|cap|band|germ"),
            ("step import", r"^step-import::")]

d = sys.argv[1]
rows = load(d)
by, origins = reduce(rows)
pop = {fp: r for fp, r in by.items() if r["t3_ok"]}
crate = collections.Counter()
fam = collections.Counter()
famfail = collections.Counter()
for fp, r in pop.items():
    suites = {suite_of(o) for o in origins[fp]}
    crate[sorted(s.split("::")[0] for s in suites)[0]] += 1
    hit = set()
    for name, pat in FAMILIES:
        if any(re.search(pat, s if name == "step import" else s.split("::", 1)[-1]) for s in suites):
            hit.add(name)
    for h in hit or {"other"}:
        fam[h] += 1
        if not r["p_ok"] and not fixture(origins[fp]):
            famfail[h] += 1
print("by crate:", dict(crate))
print("| family (module keyword; a body can be in several) | bodies | non-fixture fails |\n|---|---|---|")
for k, v in fam.most_common():
    print(f"| {k} | {v} | {famfail[k]} |")
