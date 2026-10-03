#!/usr/bin/env python3
"""Which census arm produced each stored error (first 6 per body): arms.py <dir>..."""
import collections, re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, reduce
from tables import fixture

def arm(e):
    m = re.match(r"UndeclaredContact \{ contact: (\w+)", e)
    if m:
        return "UndeclaredContact/" + m.group(1)
    m = re.search(r'predicate: Some\("(\w+)"\)', e)
    v = re.match(r"(\w+)", e).group(1)
    if v == "CensusUnsupported":
        c = re.search(r"cause: (\w+)", e)
        return f"CensusUnsupported/{c.group(1) if c else '?'}"
    if v == "CensusUndecidable":
        return "CensusUndecidable/cross-solid backstop" if "within reach" in e else "CensusUndecidable/" + e[:60]
    if m:
        return f"{v}/{m.group(1)}"
    return v

for d in sys.argv[1:]:
    rows = load(d)
    by, origins = reduce(rows)
    tab = {True: collections.Counter(), False: collections.Counter()}
    for fp, r in by.items():
        if not r["t3_ok"] or r["p_ok"]:
            continue
        fx = fixture(origins[fp])
        for a in {arm(e) for e in r["errs"]}:
            tab[fx][a] += 1
    print(f"== {d}")
    keys = sorted(set(tab[True]) | set(tab[False]))
    print("| arm (variant/kind or predicate) | non-fixture bodies | fixture bodies |\n|---|---|---|")
    for k in keys:
        print(f"| {k} | {tab[False][k]} | {tab[True][k]} |")
