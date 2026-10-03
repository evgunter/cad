#!/usr/bin/env python3
"""Reduce one eps dir to results/bodies-<tag>.tsv.gz: export.py <dir> <tag>"""
import gzip, json, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, load_declared, reduce, suite_of
from tables import fixture

d, tag = sys.argv[1], sys.argv[2]
rows = load(d)
by, origins = reduce(rows)
decl = load_declared(d)
out = __file__.rsplit("/", 1)[0] + f"/results/bodies-{tag}.tsv.gz"
with gzip.open(out, "wt") as f:
    f.write("fp\tfirst_origin_suite\tfirst_origin\tn_origins\tfixture\tdeclared_elsewhere\tsolids\tv\te\tf\tkinds\tt3_ok\tp_ok\tt3_s\tp_s\tc_s\tvariants\tfirst_error\n")
    for fp, r in by.items():
        o = sorted(origins[fp])
        f.write("\t".join(map(str, [fp, suite_of(o[0]), o[0], len(o), int(fixture(o)), int(fp in decl), r["solids"], r["v"], r["e"], r["f"], r["kinds"], int(r["t3_ok"]), int(r["p_ok"]), r["t3_s"], r["p_s"], r["c_s"], json.dumps(r["variants"]), (r["errs"][0][:300] if r["errs"] else "")])) + "\n")
print(out)
