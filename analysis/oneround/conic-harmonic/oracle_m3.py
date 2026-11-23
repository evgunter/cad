#!/usr/bin/env python3
"""M3 oracle. For each run of the ladder the harness logged the anchor,
the five coefficients of F*(1+t^2)^2 in t = tan((theta - anchor)/2) and
the depressed quartic (p, q, s). The truth is formed here in 60-digit
mpmath from the stored curve and the sphere: a0 = F(anchor),
a1 = 2F'(anchor), a2 = F(anchor)+F(pole)+F''(anchor)+F''(pole),
a3 = -2F'(pole), a4 = F(pole), then monic in tau = lever*t and depressed.
"""
import json, sys, collections
sys.argv_saved = sys.argv
exec(open(__file__.replace("oracle_m3.py", "oracle_m1.py")).read().split("def main")[0])

def jet(k, c, rad, t):
    q, q1, q2 = sub(k.at(t), c), k.d(t), k.dd(t)
    return ((dot(q, q) - rad * rad) / (2 * rad), 2 * dot(q, q1) / (2 * rad), 2 * (dot(q1, q1) + dot(q, q2)) / (2 * rad))

for eps in ("1e-9", "1e-12"):
    rows = []
    for line in open(f"{sys.argv_saved[1]}/m3-eps{eps}.jsonl"):
        r = json.loads(line)
        qs = [l for l in r["log"] if l.startswith("ladder-quartic")]
        ds = [l for l in r["log"] if l.startswith("ladder-depressed")]
        rec = {key: r[key] for key in ("variant", "fat", "target", "case", "radius", "pose", "arc")}
        rec["kind"] = r["kind"].split("{")[0].strip()
        if qs and ds:
            w = qs[-1].split(); anchor = F(w[2]); a = [F(x) for x in w[4:9]]
            k = Conic(r); c, rad = V(r["c"]), F(r["radius"])
            f0, f1, f2 = jet(k, c, rad, anchor); g0, g1, g2 = jet(k, c, rad, anchor + pi)
            true = [f0, 2 * f1, f0 + g0 + f2 + g2, -2 * g1, g0]
            rec["a_rel"] = [float(abs(x - y) / abs(y)) if y != 0 else 0.0 for x, y in zip(a, true)]
            rec["a0_abs"] = float(abs(a[0] - true[0])); rec["a0_true"] = float(true[0])
            d = ds[-1].split(); p, q, s, lever = F(d[2]), F(d[4]), F(d[6]), F(d[8])
            b3 = lever * true[3] / true[4]; c2 = lever ** 2 * true[2] / true[4]; d1 = lever ** 3 * true[1] / true[4]; e0 = lever ** 4 * true[0] / true[4]
            tp = c2 - 3 * b3 ** 2 / 8; tq = d1 - b3 * c2 / 2 + b3 ** 3 / 8
            ts = e0 - b3 * d1 / 4 + b3 ** 2 * c2 / 16 - 3 * b3 ** 4 / 256
            rec["dep_rel"] = [float(abs(x - y) / abs(y)) if y != 0 else 0.0 for x, y in ((p, tp), (q, tq), (s, ts))]
            # the same depression done in f64 from the TRUE coefficients rounded once: what the stage itself loses
            fa = [float(x) for x in true]; L = float(lever)
            B3 = L * fa[3] / fa[4]; C2 = L ** 2 * fa[2] / fa[4]; D1 = L ** 3 * fa[1] / fa[4]; E0 = L ** 4 * fa[0] / fa[4]
            fs = E0 - B3 * D1 / 4 + B3 ** 2 * C2 / 16 - 3 * B3 ** 4 / 256
            rec["s_stage_rel"] = float(abs(F(fs) - ts) / abs(ts)) if ts != 0 else 0.0
            rec["s_terms_over_s"] = float((abs(e0) + abs(b3 * d1 / 4) + abs(b3 ** 2 * c2 / 16) + abs(3 * b3 ** 4 / 256)) / abs(ts)) if ts != 0 else 0.0
        rows.append(rec)
    json.dump(rows, open(f"{sys.argv_saved[1]}/m3-eps{eps}.judged.json", "w"))
