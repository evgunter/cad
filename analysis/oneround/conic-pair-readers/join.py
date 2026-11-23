#!/usr/bin/env python3
"""Join the arc-held ladder's judged readings, base to head, pose by pose.

Usage: join.py <base-dir> <head-dir> [name]   (name: `ladder-eps`, the default)

Each directory holds `<name><eps>.judged.json` for eps 1e-9 and 1e-12, written
by `oracle.py`. A pose is (door, lane, fat, target, case, shift); a reading's
state is `right`, `WRONG` or `refused:<row>` (the row the refusal names).

The ACCEPTANCE table is the poses whose spans lie in the session box (the
harness's predicate on the spans' points, `in_box`, not the kernel's
cover-based one) on an ellipse whose
parameter is zero at its anchor (`seam_at_anchor`): it must show 0 wrong on
the head and 0 right-to-wrong. The past-box poses are printed beside it,
joined the same way, with no claim.
"""

import collections
import json
import sys

base_dir, head_dir = sys.argv[1], sys.argv[2]
name = sys.argv[3] if len(sys.argv) > 3 else "ladder-eps"


def key(j):
    return (j["door"], j["lane"], j["fat"], j["target"], j["case"], j["shift"])


def state(j):
    if j["said"] == "refused":
        return "refused:" + j["row"]
    return "WRONG" if j["wrong"] else "right"


def coarse(s):
    return "refused" if s.startswith("refused") else s


for eps in ("1e-9", "1e-12"):
    B = {key(j): j for j in json.load(open(f"{base_dir}/{name}{eps}.judged.json"))}
    H = {key(j): j for j in json.load(open(f"{head_dir}/{name}{eps}.judged.json"))}
    assert set(B) == set(H), "the two ladders read different poses"
    for j in H.values():
        assert j["in_box"] == B[key(j)]["in_box"], (
            "the box flag moved between base and head"
        )
    print(f"===== eps {eps}: {len(H)} readings")
    for title, sel in (
        (
            "ACCEPTANCE: in the box, seam at the anchor",
            lambda j: j["in_box"] and j["seam_at_anchor"],
        ),
        ("past the box (no claim)", lambda j: not j["in_box"]),
        (
            "in the box, seam off the anchor",
            lambda j: j["in_box"] and not j["seam_at_anchor"],
        ),
    ):
        ks = [k for k in H if sel(H[k])]
        print(f"--- {title}: {len(ks)} readings")
        if not ks:
            continue
        mv = collections.Counter(
            (k[1], coarse(state(B[k])), coarse(state(H[k]))) for k in ks
        )
        for lane in ("f64", "interval"):
            row = {m: v for m, v in mv.items() if m[0] == lane}
            if not row:
                continue
            wrong = sum(v for m, v in row.items() if m[2] == "WRONG")
            r2w = row.get((lane, "right", "WRONG"), 0)
            print(
                f"  {lane}: head wrong {wrong}, right->wrong {r2w}, base wrong "
                f"{sum(v for m, v in row.items() if m[1] == 'WRONG')}"
            )
            for m in sorted(row):
                print(f"    base {m[1]:8} -> head {m[2]:8} {row[m]}")
        lost = collections.Counter(
            (k[1], k[0], state(H[k]))
            for k in ks
            if state(B[k]) == "right" and state(H[k]) != "right"
        )
        if lost:
            print("  right on base, refused on head, by (lane, door, refusing row):")
            for m in sorted(lost):
                print("    ", *m, lost[m])
        refusals = collections.Counter(
            (k[1], state(H[k])) for k in ks if state(H[k]).startswith("refused")
        )
        print("  every head refusal, by (lane, row):")
        for m in sorted(refusals):
            print("    ", *m, refusals[m])
        for k in ks:
            if H[k]["wrong"]:
                h = H[k]
                print(
                    "  HEAD WRONG",
                    *k,
                    f"theta={h['theta']:.4g} gap={h['gap']:.3g} said={h['said']} "
                    f"truth={h['truth']} off={['%.3g' % o for o in h['off']]} base={state(B[k])}",
                )
