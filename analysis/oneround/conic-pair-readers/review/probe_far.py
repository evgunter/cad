#!/usr/bin/env python3
"""Review probe (C5): for each f64 line-door pose of the ladder, the height
`Conic::height_of` read for the line's f64 point q against the height the
stored numbers state, dot(q - S, axis), at 60 digits; and the true gap of the
pose (the unit's oracle). Usage: probe_far.py far-eps<eps>.jsonl"""
import json, sys, collections
sys.path.insert(0, __file__.rsplit('/', 2)[0])
import oracle
from oracle import F, V, dot, sub
out = []
for l in open(sys.argv[1]):
    r = json.loads(l)
    if r["door"] != "far": continue
    S, n, q = V(r["S"]), V(r["axis"]), V(r["q"])
    htrue = dot(sub(q, S), n)
    r2 = dict(r, door="line", lane="f64")
    j = oracle.judge(r2)
    j.update(hq=r["hq"], htrue=float(htrue), herr=float(F(r["hq"]) - htrue), zero=r["zero"], esc=r["esc"],
             dist=float(oracle.norm(sub(q, S))), log=r.get("log", ""))
    out.append(j)
json.dump(out, open(sys.argv[1].replace(".jsonl", ".judged.json"), "w"))
zero, esc = out[0]["zero"], out[0]["esc"]
tab = collections.defaultdict(lambda: [0, 0.0, 0, 0])
for j in out:
    k = (j["place"], "fat" if j["fat"] else "thin", j["target"])
    t = tab[k]; t[0] += 1; t[1] = max(t[1], abs(j["herr"])); t[2] += abs(j["herr"]) > zero; t[3] += j["wrong"]
print("zero", zero, "esc", esc)
print("place shape a | n | max |height_of - true| (m) | n with error > zero | meet WRONG")
for k in sorted(tab): print(*k, "|", tab[k][0], "| %.2e |" % tab[k][1], tab[k][2], "|", tab[k][3])
for j in out:
    if j["wrong"]:
        print("WRONG", j["place"], "fat" if j["fat"] else "thin", j["target"], "theta=%g gamma=%.3g shift=%g" % (j["theta"], j["gamma"], j["shift"]),
              "true gap %.3g (%s), said %s, off %s | |q-S| %.3g, height_of(q) %.3e, true %.3e, err %.2e" % (j["gap"], j["truth"], j["said"], ["%.2g" % o for o in j["off"]], j["dist"], j["hq"], j["htrue"], j["herr"]))
        print("      decided:", j["log"])
