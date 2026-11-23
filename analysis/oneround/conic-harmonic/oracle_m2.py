#!/usr/bin/env python3
"""M2 oracle: an arc-held ellipse against a sphere beside its anchor.

Truth: the curve the stored numbers state, in 60-digit mpmath (see
oracle_m1.py). The residual is |P(t) - c| - r; its roots near the span
are found by a sign scan over the span widened threefold and polished by
bisection in mpmath. The scratch jet's values and bounds are checked
against F = (|P - c|^2 - r^2)/2r and its derivatives formed here.
"""
import json, sys, collections
sys.argv_saved = sys.argv
exec(open(__file__.replace("oracle_m2.py", "oracle_m1.py")).read().split("def main")[0])


def judge(path, eps):
    out = []
    for line in open(path):
        r = json.loads(line)
        k = Conic(r)
        c, rad = V(r["c"]), F(r["radius"])
        res = lambda t: norm(sub(k.at(t), c)) - rad
        t0, t1 = F(r["span"][0]), F(r["span"][1])
        lo, hi = t0 - (t1 - t0), t1 + (t1 - t0)
        N = 240
        xs = [lo + (hi - lo) * i / N for i in range(N + 1)]
        vals = [res(x) for x in xs]
        truths = []
        for i in range(N):
            if vals[i] == 0 or vals[i] * vals[i + 1] < 0:
                a, b = xs[i], xs[i + 1]
                for _ in range(170):
                    m = (a + b) / 2
                    if res(a) * res(m) <= 0: b = m
                    else: a = m
                truths.append((a + b) / 2)
        rec = {key: r[key] for key in ("variant", "fat", "target", "case", "arc", "radius", "depth")}
        rec["kind"] = r["kind"]
        rec["true_roots"] = len(truths)
        # how deep the arc dips into the sphere (the least residual), for the graze's scale
        rec["dip"] = float(-min(vals))
        rec["roots"] = []
        for t in r["roots"]:
            tk = F(t)
            if truths:
                best = min(truths, key=lambda x: abs(wrap(tk - x)))
                rec["roots"].append({"arc": float(norm(sub(k.at(tk), k.at(best)))), "res": float(abs(res(tk))),
                                     "floor": float(abs(tk) * mpf(2) ** -53 * norm(k.d(best)))})
            else:
                rec["roots"].append({"arc": float("inf"), "res": float(abs(res(tk))), "floor": 0.0})
        # the jet's bounds
        worst = 0.0
        for j in r["jet"]:
            t = F(j[0])
            q, q1, q2 = sub(k.at(t), c), k.d(t), k.dd(t)
            true = [(dot(q, q) - rad * rad) / (2 * rad), 2 * dot(q, q1) / (2 * rad),
                    2 * (dot(q1, q1) + dot(q, q2)) / (2 * rad), 2 * (3 * dot(q1, q2) - dot(q, q1)) / (2 * rad)]
            for i in range(4):
                err, bound = abs(F(j[1 + 2 * i]) - true[i]), F(j[2 + 2 * i])
                if bound > 0:
                    worst = max(worst, float(err / bound))
                elif err > 0:
                    worst = float("inf")
        rec["jet_err_over_bound"] = worst
        rec["log"] = r["log"]
        out.append(rec)
    json.dump(out, open(path.replace(".jsonl", ".judged.json"), "w"))


if __name__ == "__main__":
    for eps in ("1e-9", "1e-12"):
        judge(f"{sys.argv_saved[1]}/m2-eps{eps}.jsonl", float(eps))
