#!/usr/bin/env python3
"""Refit the two models that missed the R-hat < 1.01 bar, with longer chains,
and patch results.json in place. Everything else is left untouched."""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import analyze
from models import fit_gauss, fit_ordered_logit, diagnose, contrast

HERE = os.path.dirname(os.path.abspath(__file__))
LONG = dict(n_iter=40000, n_warmup=12000, n_chains=4)


def patch(R, key, chains, npar, names, n, contrasts, keep):
    out = dict(keep)
    out["diagnostics"] = diagnose(chains, npar, names)
    out["contrasts"] = {}
    for cname, (j, tf) in contrasts.items():
        summ, draws = contrast(chains, j, tf)
        out["contrasts"][cname] = {
            "summary": summ,
            "draws": draws[:: max(1, len(draws) // 2000)]}
    R[key] = out
    worst_r = max(d["rhat"] for d in out["diagnostics"])
    worst_e = min(d["ess"] for d in out["diagnostics"])
    print("  %-14s worst R-hat %.4f  min ESS %.0f  -> %s"
          % (key, worst_r, worst_e,
             "OK" if worst_r < 1.01 and worst_e > 400 else "STILL HIGH"))


def main():
    R = json.load(open(os.path.join(HERE, "results.json")))
    rows = analyze.load()
    Q = analyze.v2_quality(rows)

    print("refitting with %d iters / %d warmup x %d chains"
          % (LONG["n_iter"], LONG["n_warmup"], LONG["n_chains"]))

    # ---- fix_size: ordered logit
    y, X, used = [], [], []
    for r in Q:
        v = analyze.y_fix_size(r)
        if v is None:
            continue
        y.append(int(v))
        X.append([analyze.opus(r), analyze.dS(r), analyze.dL(r)])
        used.append(r["row_id"])
    ch, npar = fit_ordered_logit(y, X, n_cat=5, **LONG)
    patch(R, "fix_size", ch, npar,
          ["opus", "diff_S", "diff_L", "cut1", "d2", "d3", "d4"], len(y),
          {"odds_ratio_bigger_fix_opus": (0, math.exp), "log_odds": (0, None)},
          {"title": R["fix_size"]["title"], "n": len(y),
           "arm_means": R["fix_size"].get("arm_means"), "rows_used": used,
           "note": "refit with longer chains"})

    # ---- char_rubric: gaussian with interaction
    y, X, used = [], [], []
    for r in Q:
        v = analyze.y_rubric_mean(r)
        if v is None:
            continue
        o = analyze.opus(r)
        b = 1.0 if r.get("ch_character") == "build_new" else 0.0
        y.append(v)
        X.append([1.0, o, b, o * b, analyze.dS(r), analyze.dL(r)])
        used.append(r["row_id"])
    ch, npar = fit_gauss(y, X, prior_sd=[10.0] + [2.0] * 5, **LONG)
    patch(R, "char_rubric", ch, npar,
          ["intercept", "opus", "build_new", "opus_x_build_new", "diff_S",
           "diff_L", "log_sigma"], len(y),
          {"opus_x_build_new": (3, None), "opus_main": (1, None)},
          {"title": R["char_rubric"]["title"], "n": len(y),
           "rows_used": used, "note": "refit with longer chains"})

    json.dump(R, open(os.path.join(HERE, "results.json"), "w"))
    print("patched results.json")


if __name__ == "__main__":
    main()
