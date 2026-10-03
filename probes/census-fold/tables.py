#!/usr/bin/env python3
"""Per-eps result tables, stratified: tables.py <dir>... (one per eps)."""
import collections, math, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, load_declared, reduce, suite_of, q

# Test modules whose SUBJECT is touching / contacts / assemblies: their
# bodies are built to touch (or to be placed together and declared at
# the assembly layer) and are gated at tier 3 as the setup of a census
# test. Matched on the module name (the test path's first segment).
FIXTURE = ("contact", "census", "tier3prime", "m3_pr6", "mate", "asm",
           "msolve", "docm", "gather", "dsc_", "s49_", "s16_", "graft",
           "p2_gauge", "perf12", "m9_2", "review_s6", "m10_di", "r2_m10_di",
           "wire_product", "instance", "bool4", "interfer")

def fixture(origins):
    for o in origins:
        mod = suite_of(o).split("::")[-1]
        if not any(k in mod for k in FIXTURE):
            return False
    return True

def stratum(r, os_):
    if fixture(os_):
        return "F"
    return "1" if r["solids"] == 1 else "M"

NAMES = {"1": "op/other, 1 solid", "M": "op/other, multi-solid", "F": "contact/assembly-subject fixtures"}

def main():
    results = {}
    for d in sys.argv[1:]:
        rows = load(d)
        by, origins = reduce(rows)
        decl = load_declared(d)
        pop = {fp: r for fp, r in by.items() if r["t3_ok"]}
        eps = rows[0]["eps"]
        results[eps] = (pop, origins, decl)
        print(f"\n## eps = {eps:g}  ({len(pop)} distinct f64 bodies passing tier 3)\n")
        print("| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |")
        print("|---|---|---|---|---|---|---|---|---|---|---|")
        for s in ("1", "M", "F"):
            sub = {fp: r for fp, r in pop.items() if stratum(r, origins[fp]) == s}
            fails = {fp: r for fp, r in sub.items() if not r["p_ok"]}
            v = collections.Counter(k for r in fails.values() for k in r["variants"])
            nd = sum(1 for fp in fails if fp in decl)
            print(f"| {NAMES[s]} | {len(sub)} | {len(sub)-len(fails)} | {len(fails)} | {nd} | "
                  + " | ".join(str(v.get(k, 0)) for k in ("UndeclaredContact", "CensusUndecidable", "InstanceInterference", "CensusEscalated", "CensusUnsupported", "CensusLaneUnsupported")) + " |")
        print("\nTiming (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:\n")
        print("| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |")
        print("|---|---|---|---|---|---|---|---|---|---|---|")
        for label, pred in (("all", lambda r, o: True),
                            ("1 solid, planar", lambda r, o: r["solids"] == 1 and r["kinds"] == "Plane"),
                            ("1 solid, curved", lambda r, o: r["solids"] == 1 and r["kinds"] != "Plane"),
                            ("multi-solid", lambda r, o: r["solids"] > 1),
                            ("non-fixture", lambda r, o: not fixture(o))):
            sub = [r for fp, r in pop.items() if pred(r, origins[fp])]
            c = [r["c_s"] / r["t3_s"] for r in sub if r["t3_s"] > 0 and not math.isnan(r["c_s"])]
            p = [r["p_s"] / r["t3_s"] for r in sub if r["t3_s"] > 0]
            t3 = [r["t3_s"] for r in sub]
            cs = [r["c_s"] for r in sub if not math.isnan(r["c_s"])]
            print(f"| {label} | {len(sub)} | {q(c,.5):.2f} | {q(c,.9):.2f} | {max(c):.1f} | {q(p,.5):.2f} | {q(p,.9):.2f} | {max(p):.1f} | {q(t3,.5):.1e} | {q(cs,.5):.1e} | {max(cs):.1e} |")

    # cross-eps verdict flips on the same fingerprint
    eps_list = sorted(results)
    if len(eps_list) > 1:
        print("\n## Verdict changes across eps (same fingerprint)\n")
        common = set.intersection(*(set(results[e][0]) for e in eps_list))
        print(f"bodies present at every eps: {len(common)}")
        flips = [fp for fp in common if len({results[e][0][fp]['p_ok'] for e in eps_list}) > 1]
        vflips = [fp for fp in common if len({tuple(sorted(results[e][0][fp]['variants'])) for e in eps_list}) > 1]
        print(f"pass/fail flips: {len(flips)}; variant-set changes: {len(vflips)}")
        for fp in sorted(set(flips) | set(vflips))[:40]:
            r0 = results[eps_list[0]][0][fp]
            print(f"- {fp} s={r0['solids']} {r0['kinds']} :: " + "; ".join(
                f"{e:g}: {'ok' if results[e][0][fp]['p_ok'] else results[e][0][fp]['variants']}" for e in eps_list)
                + f" :: {sorted(results[eps_list[0]][1][fp])[0]}")


if __name__ == "__main__":
    main()
