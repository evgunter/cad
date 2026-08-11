#!/usr/bin/env python3
"""Reviewer-concordance analysis from the dual-review pairs.

The experiment sends every 3rd merged row to a SECOND independent blinded
reviewer on the same code head. This estimates how much of a finding
count is the reviewer rather than the code — the confound the first
readout named as its binding measurement problem.

Outputs concordance.json for the report.
"""
import csv
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from mcmc import (log_normal_pdf, log_half_normal, run_chains, flatten,
                  summarize, rhat, ess, quantile)

HERE = os.path.dirname(os.path.abspath(__file__))
NA = ("NA", "", "-", "None", None)


def na(v):
    return v is None or str(v).strip() in NA


def num(v):
    return None if na(v) else float(v)


def load_pairs(include_caveated=True):
    rows = []
    for r in csv.DictReader(open(os.path.join(HERE, "labels/dual-review.csv"))):
        if r["row_id"].startswith("#"):
            continue
        if r["status"] != "valid":
            continue
        if not include_caveated and r["row_id"] == "U7":
            continue
        rows.append(r)
    return rows


def pairs_for(rows, f1, f2):
    out = []
    for r in rows:
        a, b = num(r[f1]), num(r[f2])
        if a is None or b is None:
            continue
        out.append((r["row_id"], a, b))
    return out


# ---------------------------------------------------------------- models

def beta_posterior_summary(successes, trials, a=1.0, b=1.0, n=200000):
    """Exact Beta(a+s, b+f) summary by quadrature (no scipy)."""
    A, B = a + successes, b + (trials - successes)
    N = 20001
    xs = [i / float(N - 1) for i in range(N)]
    logs = []
    for x in xs:
        if x <= 0 or x >= 1:
            logs.append(-1e300)
        else:
            logs.append((A - 1) * math.log(x) + (B - 1) * math.log(1 - x))
    m = max(logs)
    w = [math.exp(v - m) for v in logs]
    tot = sum(w)
    w = [v / tot for v in w]
    mean = sum(x * wi for x, wi in zip(xs, w))
    cum, q = 0.0, {}
    for x, wi in zip(xs, w):
        cum += wi
        for p in (0.025, 0.5, 0.975):
            if p not in q and cum >= p:
                q[p] = x
    return {"mean": mean, "median": q.get(0.5), "q025": q.get(0.025),
            "q975": q.get(0.975), "successes": successes, "trials": trials}


def sigma_e_from_pairs(prs, offset=0.5, seed=3):
    """Reviewer noise SD on the log-count scale.

    d_i = log(y_i1+c) - log(y_i2+c) ~ Normal(delta, 2*sigma_e^2)
    delta is a systematic R1-vs-R2 order effect (0 = exchangeable).
    """
    d = [math.log(a + offset) - math.log(b + offset) for _, a, b in prs]
    n = len(d)

    def logpost(th):
        delta, log_s = th
        if log_s > 3 or log_s < -9:
            return -1e300
        s = math.exp(log_s)
        lp = log_normal_pdf(delta, 0.0, 1.0)
        lp += log_half_normal(s, 1.0) + log_s          # half-normal + jacobian
        sd = s * math.sqrt(2.0)
        for di in d:
            lp += log_normal_pdf(di, delta, sd)
        return lp

    chains = run_chains(logpost, lambda c: [0.0, math.log(0.3 + 0.1 * c)],
                        n_chains=4, n_iter=30000, n_warmup=8000, seed=seed)
    sig = [math.exp(x) for x in flatten(chains, 1)]
    dlt = flatten(chains, 0)
    return {
        "n_pairs": n,
        "sigma_e": summarize(sig),
        "order_effect": summarize(dlt),
        "rhat": max(rhat(chains, 0), rhat(chains, 1)),
        "ess": min(ess(chains, 0), ess(chains, 1)),
        "raw_diffs": d,
    }


def gauss_sigma_pairs(prs, seed=5):
    """Reviewer noise SD in raw rating points (for the 1-5 rubric)."""
    d = [a - b for _, a, b in prs]
    n = len(d)
    if all(abs(x) < 1e-12 for x in d):
        return {"n_pairs": n, "degenerate": True,
                "sigma_e": None, "raw_diffs": d}

    def logpost(th):
        delta, log_s = th
        if log_s > 3 or log_s < -9:
            return -1e300
        s = math.exp(log_s)
        lp = log_normal_pdf(delta, 0.0, 1.0) + log_half_normal(s, 1.0) + log_s
        sd = s * math.sqrt(2.0)
        for di in d:
            lp += log_normal_pdf(di, delta, sd)
        return lp

    chains = run_chains(logpost, lambda c: [0.0, math.log(0.4 + 0.1 * c)],
                        n_chains=4, n_iter=30000, n_warmup=8000, seed=seed)
    return {"n_pairs": n, "degenerate": False,
            "sigma_e": summarize([math.exp(x) for x in flatten(chains, 1)]),
            "order_effect": summarize(flatten(chains, 0)),
            "raw_diffs": d}


def exact_agreement(prs):
    agree = sum(1 for _, a, b in prs if abs(a - b) < 1e-9)
    return beta_posterior_summary(agree, len(prs))


def informative_subset(prs):
    """Pairs where at least one reviewer reported a nonzero count.

    A 0-0 pair is nearly free agreement; it is the prevalence trap.
    """
    return [p for p in prs if p[1] > 0 or p[2] > 0]


def projection(prs, extra_list=(4, 8, 16, 32)):
    """How much would more pairs tighten the agreement bound?

    Assumes future pairs agree at the currently-observed rate; reports the
    resulting Beta upper/lower bound. This is a precision projection, not
    a prediction that agreement continues.
    """
    agree = sum(1 for _, a, b in prs if abs(a - b) < 1e-9)
    n = len(prs)
    rate = agree / float(n) if n else 0.0
    out = []
    for extra in (0,) + tuple(extra_list):
        tot = n + extra
        succ = agree + rate * extra
        s = beta_posterior_summary(succ, tot)
        out.append({"total_pairs": tot, "assumed_agree": round(succ, 1),
                    "lower95": s["q025"], "median": s["median"]})
    return out


def main():
    res = {}
    for tag, incl in (("all8", True), ("drop_u7", False)):
        rows = load_pairs(include_caveated=incl)
        block = {"n_pairs": len(rows), "rows": [r["row_id"] for r in rows]}

        for name, f1, f2 in (("maj", "r1_maj", "r2_maj"),
                             ("min", "r1_min", "r2_min"),
                             ("note", "r1_note", "r2_note")):
            prs = pairs_for(rows, f1, f2)
            info = informative_subset(prs)
            block[name] = {
                "pairs": [(rid, a, b) for rid, a, b in prs],
                "exact_agreement": exact_agreement(prs),
                "n_informative": len(info),
                "exact_agreement_informative": (exact_agreement(info)
                                                if info else None),
                "noise": sigma_e_from_pairs(prs) if len(prs) > 1 else None,
                "projection": projection(prs),
            }

        for name, f1, f2 in (("idiom", "r1_idiom", "r2_idiom"),
                             ("tests", "r1_tests", "r2_tests"),
                             ("docs", "r1_docs", "r2_docs")):
            prs = pairs_for(rows, f1, f2)
            block["rubric_" + name] = {
                "pairs": [(rid, a, b) for rid, a, b in prs],
                "exact_agreement": exact_agreement(prs) if prs else None,
                "noise": gauss_sigma_pairs(prs) if len(prs) > 1 else None,
            }

        # verdict labels
        vd = [(r["row_id"], r["r1_verdict"], r["r2_verdict"]) for r in rows
              if not na(r["r1_verdict"]) and not na(r["r2_verdict"])]
        dis = [v for v in vd if v[1] != v[2]]
        block["verdict"] = {
            "pairs": vd,
            "n": len(vd),
            "n_disagree": len(dis),
            "disagreeing_rows": [v[0] for v in dis],
            "disagreement_rate": beta_posterior_summary(len(dis), len(vd)),
        }
        res[tag] = block

    with open(os.path.join(HERE, "concordance.json"), "w") as f:
        json.dump(res, f)

    # ------------------------------------------------ console summary
    b = res["all8"]
    print("dual-review pairs: %d  (%s)" % (b["n_pairs"], ", ".join(b["rows"])))
    for name in ("maj", "min", "note"):
        d = b[name]
        ea = d["exact_agreement"]
        print("\n%s counts" % name.upper())
        print("  pairs: %s" % ", ".join("%s %g/%g" % (r, a, bb)
                                        for r, a, bb in d["pairs"]))
        print("  exact agreement %d/%d -> p=%.2f (95%% CrI %.2f-%.2f)"
              % (ea["successes"], ea["trials"], ea["median"], ea["q025"],
                 ea["q975"]))
        print("  informative (non 0-0) pairs: %d" % d["n_informative"])
        if d["exact_agreement_informative"]:
            ei = d["exact_agreement_informative"]
            print("    agreement among those %d/%d -> p=%.2f (95%% CrI %.2f-%.2f)"
                  % (ei["successes"], ei["trials"], ei["median"], ei["q025"],
                     ei["q975"]))
        if d["noise"]:
            s = d["noise"]["sigma_e"]
            o = d["noise"]["order_effect"]
            print("  reviewer noise sigma_e (log scale) = %.3f (95%% CrI %.3f-%.3f)"
                  % (s["median"], s["q025"], s["q975"]))
            print("  R1-vs-R2 order effect = %+.3f (95%% CrI %+.3f-%+.3f)"
                  % (o["median"], o["q025"], o["q975"]))
    for name in ("idiom", "tests", "docs"):
        d = b["rubric_" + name]
        if not d["noise"]:
            continue
        print("\nRUBRIC %s: diffs %s" % (name, d["noise"]["raw_diffs"]))
        if d["noise"].get("degenerate"):
            print("  every pair identical - zero observed variance "
                  "(dimension is saturated, carries no information)")
        else:
            s = d["noise"]["sigma_e"]
            print("  sigma_e = %.2f rating points (95%% CrI %.2f-%.2f)"
                  % (s["median"], s["q025"], s["q975"]))
    v = b["verdict"]
    print("\nVERDICT LABELS: %d/%d pairs DISAGREE (%s)"
          % (v["n_disagree"], v["n"], ", ".join(v["disagreeing_rows"])))
    dr = v["disagreement_rate"]
    print("  disagreement rate %.2f (95%% CrI %.2f-%.2f)"
          % (dr["median"], dr["q025"], dr["q975"]))
    print("\nwrote concordance.json")


if __name__ == "__main__":
    main()
