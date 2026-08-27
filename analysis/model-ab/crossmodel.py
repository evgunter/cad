#!/usr/bin/env python3
"""Cross-model reviewer analysis: does an opus reviewer find what a fable one misses?

Design note. Within a dual pair both reviewers read the SAME code at the
SAME time, so unit difficulty, era, and the implementer arm cannot
produce a within-pair asymmetry. The only competing explanation for a
cross-model gap is an R1-vs-R2 ROLE effect, and the same-model pairs
(both reviewers fable) are the control that rules it in or out.

Inference is conditional and exact: given a pair's total findings, the
share attributable to one reviewer is Binomial, so a Beta posterior on
that share tests the asymmetry without modelling the unit's difficulty.
"""
import csv, math, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
NA = ("NA", "", "-", "None", "unknown", None)


def na(v): return v is None or str(v).strip() in NA
def num(v): return None if na(v) else float(v)


def rd(p):
    f = os.path.join(HERE, p)
    if not os.path.exists(f):
        return []
    return [r for r in csv.DictReader(open(f))
            if r.get(list(r.keys())[0], "") and not list(r.values())[0].startswith("#")]


def beta_summary(s, f, a=1.0, b=1.0, N=40001):
    """Posterior for a proportion, by quadrature."""
    A, B = a + s, b + f
    xs = [i / float(N - 1) for i in range(N)]
    lg = []
    for x in xs:
        lg.append(-1e300 if x <= 0 or x >= 1 else
                  (A - 1) * math.log(x) + (B - 1) * math.log(1 - x))
    m = max(lg)
    w = [math.exp(v - m) for v in lg]
    t = sum(w); w = [v / t for v in w]
    mean = sum(x * wi for x, wi in zip(xs, w))
    cum, q = 0.0, {}
    for x, wi in zip(xs, w):
        cum += wi
        for p in (0.025, 0.5, 0.975):
            if p not in q and cum >= p:
                q[p] = x
    p_gt_half = sum(wi for x, wi in zip(xs, w) if x > 0.5)
    return {"mean": mean, "med": q.get(0.5), "lo": q.get(0.025),
            "hi": q.get(0.975), "p_gt_half": p_gt_half, "s": s, "f": f}


def show(tag, r):
    print("  %-42s %d vs %d -> share %.2f (95%% CrI %.2f-%.2f), P(>0.5)=%.3f"
          % (tag, r["s"], r["f"], r["med"], r["lo"], r["hi"], r["p_gt_half"]))


def main():
    xf = rd("labels/cross-model-findings.csv")
    xp = rd("labels/cross-model-pairs.csv")
    sf = rd("labels/same-model-findings.csv")

    print("=" * 74)
    print("CROSS-MODEL PAIRS: paired MAJOR counts")
    print("=" * 74)
    tot_op = tot_fa = 0
    npair = 0
    for r in xp:
        r1m, r2m = r.get("r1_model", "?"), r.get("r2_model", "?")
        a, b = num(r.get("r1_maj")), num(r.get("r2_maj"))
        if a is None or b is None:
            print("  #%-3s %-12s R1(%s)=%s R2(%s)=%s   [incomplete]"
                  % (r["sample_no"], r["row_id"], r1m, r.get("r1_maj"),
                     r2m, r.get("r2_maj")))
            continue
        npair += 1
        op = b if r2m == "opus" else a
        fa = a if r2m == "opus" else b
        tot_op += op; tot_fa += fa
        print("  #%-3s %-12s  fable %g MAJ  |  opus %g MAJ   (%s / %s)"
              % (r["sample_no"], r["row_id"], fa, op,
                 r.get("difficulty", "?"), r.get("task_class", "?")))
    print("\n  totals over %d complete pairs:  fable %d MAJOR, opus %d MAJOR"
          % (npair, tot_fa, tot_op))
    if tot_op + tot_fa:
        show("opus share of all MAJORs raised", beta_summary(tot_op, tot_fa))

    print("\n" + "=" * 74)
    print("THE HEADLINE: unilateral MAJORs (other reviewer never mentioned it)")
    print("=" * 74)
    cats = {}
    for r in xf:
        cats.setdefault(r["correspondence"], []).append(r)
    for k in sorted(cats):
        print("  %-16s %d" % (k, len(cats[k])))
    uni = cats.get("unilateral", [])
    u_op = sum(1 for r in uni if r["raiser_model"] == "opus")
    u_fa = sum(1 for r in uni if r["raiser_model"] == "fable")
    print("\n  unilateral raised by OPUS  : %d" % u_op)
    print("  unilateral raised by FABLE : %d" % u_fa)
    if u_op + u_fa:
        show("opus share of unilateral MAJORs", beta_summary(u_op, u_fa))
    sp = cats.get("severity_split", [])
    s_op = sum(1 for r in sp if r["raiser_model"] == "opus")
    s_fa = sum(1 for r in sp if r["raiser_model"] == "fable")
    print("\n  severity_split raised as MAJOR by opus : %d" % s_op)
    print("  severity_split raised as MAJOR by fable: %d" % s_fa)
    if sp:
        from collections import Counter
        print("  other reviewer's severity:",
              dict(Counter(r["other_severity"] for r in sp)))

    print("\n" + "=" * 74)
    print("CONTROL: same-model pairs (both reviewers fable)")
    print("=" * 74)
    if not sf:
        print("  [same-model findings file not present]")
    else:
        c2 = {}
        for r in sf:
            c2.setdefault(r["correspondence"], []).append(r)
        for k in sorted(c2):
            print("  %-16s %d" % (k, len(c2[k])))
        u2 = c2.get("unilateral", [])
        a = sum(1 for r in u2 if r["raised_by"] == "R2")
        b = sum(1 for r in u2 if r["raised_by"] == "R1")
        print("\n  unilateral by R2: %d   by R1: %d" % (a, b))
        if a + b:
            show("R2 share of unilateral MAJORs (expect ~0.5)",
                 beta_summary(a, b))
        print("\n  Read: this is the ROLE-effect control. If the R2 share here")
        print("  sits near 0.5 while the opus share above is high, the")
        print("  cross-model gap is the reviewer MODEL, not the R1/R2 slot.")


if __name__ == "__main__":
    main()
