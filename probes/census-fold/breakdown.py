#!/usr/bin/env python3
"""Failure breakdown: breakdown.py <dir> — by solid count, variant, suite."""
import collections, sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, load_declared, reduce, suite_of

d = sys.argv[1]
rows = load(d)
by, origins = reduce(rows)
decl = load_declared(d)
pop = {fp: r for fp, r in by.items() if r["t3_ok"]}
def cls(r):
    return "1-solid" if r["solids"] == 1 else "multi-solid"
tab = collections.Counter()
for fp, r in pop.items():
    tab[(cls(r), "pass" if r["p_ok"] else "fail")] += 1
print("pass/fail by solids:", dict(tab))
suites = collections.defaultdict(lambda: [0, 0, collections.Counter()])
for fp, r in pop.items():
    s = suite_of(sorted(origins[fp])[0])
    suites[s][0] += 1
    if not r["p_ok"]:
        suites[s][1] += 1
        for k in r["variants"]:
            suites[s][2][k] += 1
print("suite\tbodies\tfail\tvariants")
for s, (n, f, v) in sorted(suites.items(), key=lambda x: -x[1][1]):
    if f or "-v" in sys.argv:
        print(f"{s}\t{n}\t{f}\t{dict(v)}")
print("\nsingle-solid failures:")
for fp, r in pop.items():
    if r["solids"] == 1 and not r["p_ok"]:
        print(f"- {fp} f={r['f']} {r['kinds']} {r['variants']} decl={'Y' if fp in decl else 'n'} :: {sorted(origins[fp])[:3]}")
        for e in r["errs"][:2]:
            print("     ", e[:300])
