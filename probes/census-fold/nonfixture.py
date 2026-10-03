#!/usr/bin/env python3
"""List the non-fixture failures of one eps dir, grouped by first origin's suite."""
import collections, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, load_declared, reduce, suite_of
from tables import fixture

d = sys.argv[1]
rows = load(d)
by, origins = reduce(rows)
decl = load_declared(d)
groups = collections.defaultdict(list)
for fp, r in by.items():
    if r["t3_ok"] and not r["p_ok"] and not fixture(origins[fp]):
        groups[suite_of(sorted(origins[fp])[0])].append(fp)
for s, fps in sorted(groups.items()):
    print(f"## {s} ({len(fps)})")
    for fp in fps:
        r = by[fp]
        whats = sorted({w for w in decl.get(fp, [])})
        print(f"- {fp} solids={r['solids']} f={r['f']} {r['kinds']} {r['variants']} decl={'Y' if fp in decl else 'n'}")
        print(f"    origins: {sorted(origins[fp])[:3]}")
        for e in r["errs"][:2]:
            print(f"    {e[:260]}")
