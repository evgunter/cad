#!/usr/bin/env python3
"""Reduce the census-fold hook's JSONL logs.

    analyze.py <dir-eps-1e-9> [<dir-eps-1e-6> <dir-eps-1e-12> ...]

Each dir holds `<pid>.jsonl` files written by the scratch topo hook
(`kernel-hook.patch`). A body is identified by its fingerprint `fp`
(Euler counts + vertex coordinates at 1e-6 + surface-kind set); the
first record of an fp at a given scalar is its row, and every origin
that produced it is kept.
"""

import collections
import glob
import json
import math
import os
import sys


def load(d):
    load_crate_map(d)
    rows = []
    for p in glob.glob(os.path.join(d, "*.jsonl")):
        if p.endswith(".declared.jsonl"):
            continue
        with open(p) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    rows.append(json.loads(line))
                except json.JSONDecodeError as e:
                    print(f"skip bad line in {p}: {e}", file=sys.stderr)
    return rows


def load_declared(d):
    out = collections.defaultdict(set)
    for p in glob.glob(os.path.join(d, "*.declared.jsonl")):
        with open(p) as f:
            for line in f:
                if line.strip():
                    r = json.loads(line)
                    out[r["declared"]].add(r["origin"])
    return out


_MOD_CRATE = {}


def load_crate_map(d):
    """Test module -> crate, read off the run's nextest log."""
    import re
    p = os.path.join(d, "nextest.log")
    if not os.path.exists(p):
        return
    with open(p) as f:
        for line in f:
            m = re.search(r"\) ([\w-]+)::(\w+) (\w+)::", line)
            if m:
                _MOD_CRATE[m.group(3)] = m.group(1)


def suite_of(origin):
    if origin.startswith("tour"):
        return "tour"
    head, _, rest = origin.partition("::")
    crate = head.split("-")[0]
    mod = rest.split("::")[0] if rest else "?"
    if crate == "all":
        crate = _MOD_CRATE.get(mod, "all")
    return f"{crate}::{mod}"


def crate_of(origin):
    if origin.startswith("tour::"):
        return "tour"
    return origin.split("::")[0].split("-")[0]


def q(v, p):
    v = sorted(v)
    if not v:
        return float("nan")
    k = min(len(v) - 1, max(0, math.ceil(p * len(v)) - 1))
    return v[k]


def reduce(rows, scalar="f64"):
    by = collections.OrderedDict()
    origins = collections.defaultdict(set)
    for r in rows:
        if r["scalar"] != scalar:
            continue
        origins[r["fp"]].add(r["origin"])
        if r["fp"] not in by:
            by[r["fp"]] = r
    return by, origins


def main():
    dirs = sys.argv[1:]
    for d in dirs:
        rows = load(d)
        scalars = collections.Counter(r["scalar"] for r in rows)
        by, origins = reduce(rows)
        decl = load_declared(d)
        pop = {fp: r for fp, r in by.items() if r["t3_ok"]}
        eps = {r["eps"] for r in rows}
        print(f"=== {d}: eps={sorted(eps)} records={len(rows)} scalars={dict(scalars)}")
        print(f"unique f64 bodies={len(by)} passing tier 3={len(pop)}")
        crates = collections.Counter()
        for fp in pop:
            crates[min(crate_of(o) for o in origins[fp])] += 1
        print("population by first crate:", dict(sorted(crates.items())))
        kinds = collections.Counter(
            "curved" if r["kinds"] != "Plane" else "planar" for r in pop.values()
        )
        multi = sum(1 for r in pop.values() if r["solids"] > 1)
        print(f"planar/curved: {dict(kinds)}; multi-solid: {multi}")
        fails = {fp: r for fp, r in pop.items() if not r["p_ok"]}
        nd = sum(1 for fp in pop if fp in decl)
        print(f"population bodies also gated WITH non-empty contacts somewhere: {nd}")
        fd = sum(1 for fp in fails if fp in decl)
        print(f"failures whose body is gated WITH declarations elsewhere: {fd}; "
              f"failures with no declaration seen: {len(fails) - fd}")
        cfail = sum(1 for r in pop.values() if not r["c_ok"])
        print(f"fail empty-contact 3': {len(fails)}  (census-alone fails: {cfail})")
        var = collections.Counter()
        for r in fails.values():
            for k, n in r["variants"].items():
                var[k] += 1
        print("bodies per error variant:", dict(var))
        # timing
        ratio_c = [r["c_s"] / r["t3_s"] for r in pop.values() if r["t3_s"] > 0 and not math.isnan(r["c_s"])]
        ratio_p = [r["p_s"] / r["t3_s"] for r in pop.values() if r["t3_s"] > 0]
        t3 = [r["t3_s"] for r in pop.values()]
        cs = [r["c_s"] for r in pop.values() if not math.isnan(r["c_s"])]
        for name, v in (("census/t3", ratio_c), ("3'/t3", ratio_p)):
            print(f"{name}: median={q(v,.5):.3f} p90={q(v,.9):.3f} max={max(v):.3f} n={len(v)}")
        print(f"t3 seconds: median={q(t3,.5):.2e} p90={q(t3,.9):.2e} max={max(t3):.2e} sum={sum(t3):.2f}")
        print(f"census seconds: median={q(cs,.5):.2e} p90={q(cs,.9):.2e} max={max(cs):.2e} sum={sum(cs):.2f}")
        # split by planar/curved, solids
        for label, pred in (
            ("planar", lambda r: r["kinds"] == "Plane"),
            ("curved", lambda r: r["kinds"] != "Plane"),
            ("multi-solid", lambda r: r["solids"] > 1),
        ):
            v = [r["c_s"] / r["t3_s"] for r in pop.values() if pred(r) and r["t3_s"] > 0 and not math.isnan(r["c_s"])]
            if v:
                print(f"  census/t3 {label}: median={q(v,.5):.3f} p90={q(v,.9):.3f} max={max(v):.3f} n={len(v)}")
        top = sorted(pop.values(), key=lambda r: -r["c_s"] / max(r["t3_s"], 1e-12))[:8]
        print("largest census/t3:")
        for r in top:
            print(f"  {r['c_s']/r['t3_s']:.2f}  c={r['c_s']:.2e} t3={r['t3_s']:.2e} f={r['f']} s={r['solids']} {r['kinds']}  {sorted(origins[r['fp']])[0]}")
        print("failures:")
        for fp, r in fails.items():
            os_ = sorted(origins[fp])
            tag = "DECLARED-ELSEWHERE" if fp in decl else "no-decl-seen"
            print(f"- [{tag}] {fp} f={r['f']} solids={r['solids']} kinds={r['kinds']} variants={r['variants']} n_err={r['n_err']}")
            print(f"    origins ({len(os_)}): {os_[:4]}")
            for e in r["errs"][:2]:
                print(f"    err: {e[:400]}")
        print()


if __name__ == "__main__":
    main()
