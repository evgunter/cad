#!/usr/bin/env python3
"""Tables for M1 from the judged records."""
import json, sys, collections

def load(eps):
    return json.load(open(f"{sys.argv[1]}/m1-eps{eps}.judged.json"))

def door_a(J, eps):
    e = float(eps)
    agg = collections.defaultdict(lambda: dict(n=0, ans=0, ref=0, arc=0.0, h=0.0, mis=0, floor=0.0, past=0, past10=0))
    for r in J:
        if r["door"] != "A":
            continue
        a = agg[(r["fat"], r["target"], "near" if r["place"] != "far" else "far", r["variant"])]
        a["n"] += 1
        if r["kind"] == "roots":
            a["ans"] += 1
            if not (r["true_in_span"][0] <= len(r["roots"]) <= r["true_in_span"][1]):
                a["mis"] += 1
            for x in r["roots"]:
                a["arc"] = max(a["arc"], x["arc"]); a["h"] = max(a["h"], x["height"]); a["floor"] = max(a["floor"], x["ulp_floor"])
                a["past"] += x["arc"] > e; a["past10"] += x["arc"] > 10 * e
        else:
            a["ref"] += 1
    print(f"door A (plane_crossing_lane), eps {eps}")
    print("shape  a      where var |  n ans ref | max arc miss  max height  theta-ulp floor | roots>eps >10eps count-mismatch")
    for k in sorted(agg):
        a = agg[k]
        print(f"{'fat ' if k[0] else 'thin'} {k[1]:.0e} {k[2]:4} ({'i','ii','iii'}[{k[3]}]) | {a['n']:2} {a['ans']:3} {a['ref']:3} | {a['arc']:.1e}  {a['h']:.1e}  {a['floor']:.1e} | {a['past']:3} {a['past10']:3} {a['mis']:3}".replace("('i', 'ii', 'iii')[","").replace("]) |"," |"))

def door_bc(J, eps, door):
    e = float(eps)
    agg = collections.defaultdict(lambda: collections.Counter())
    arc = collections.defaultdict(float)
    for r in J:
        if r["door"] != door:
            continue
        key = (r["fat"], r["target"], "near" if r["place"] != "far" else "far", r["variant"])
        meets = r["true_gap"] <= e
        apart = r["true_gap"] > 10 * e
        truth = "meets" if meets else ("apart" if apart else "gap")
        if r["kind"].startswith("ok"):
            n = len(r["points"])
            if n == 0:
                verdict = "MISSED" if meets else ("apart-ok" if apart else "apart-in-gap")
            else:
                worst = max(p["arc"] for p in r["points"])
                arc[key] = max(arc[key], worst)
                good = [p for p in r["points"] if p["arc"] <= e]
                verdict = ("found" if len(good) >= 1 and n == 1 else ("found+extra" if good else "OFF>eps")) if not apart else "PHANTOM"
        else:
            verdict = r["kind"].replace("refused:", "ref:")
        agg[key][(truth, verdict)] += 1
    print(f"door {door}, eps {eps}")
    for k in sorted(agg):
        print(f"{'fat ' if k[0] else 'thin'} {k[1]:.0e} {k[2]:4} v{k[3]} | max arc {arc[k]:.1e} | " + ", ".join(f"{t}/{v}:{n}" for (t, v), n in sorted(agg[k].items())))

for eps in ("1e-9", "1e-12"):
    J = load(eps)
    which = sys.argv[2] if len(sys.argv) > 2 else "ABC"
    if "A" in which: door_a(J, eps)
    if "B" in which: door_bc(J, eps, "B")
    if "C" in which: door_bc(J, eps, "C")
