#!/usr/bin/env python3
"""How much would MORE dual pairs actually buy?

Answers the stopping question in the currency that matters: not
"significance", but whether the uncertainty on reviewer noise still
changes a decision.

Two questions are projected separately, because they need very different
amounts of data:

  Q1  How precisely do we know reviewer noise sigma_e?
      -> Needed to widen the arm interval for reviewer variance.
      -> But see the note below: arm-INDEPENDENT noise cannot bias the
         rate ratio at all, and the negative-binomial fit already
         absorbs overdispersion using all 76 rows rather than 9 pairs.

  Q2  Is reviewer noise ARM-DEPENDENT?
      -> This is the only thing that could BIAS the arm contrast rather
         than merely widen it, and it is what the dual design is
         uniquely able to test.

Method: for paired log-differences d_i ~ N(0, 2 sigma^2), the posterior
for sigma^2 under a Jeffreys prior is scaled-inverse-chi-square, so
sigma^2 = k*s^2 / X with X ~ chi2_k. Sampling X directly gives exact
projections with no scipy.
"""
import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

HERE = os.path.dirname(os.path.abspath(__file__))
NDRAW = 200000


def chi2_draws(k, rng, n=NDRAW):
    return [sum(rng.gauss(0, 1) ** 2 for _ in range(k)) for _ in range(n)]


def sigma_posterior(k, s2, rng):
    """Posterior draws of sigma given k pairs with sufficient stat s2."""
    return [math.sqrt(k * s2 / x) for x in chi2_draws(k, rng) if x > 0]


def q(v, p):
    v = sorted(v)
    return v[min(len(v) - 1, max(0, int(p * (len(v) - 1))))]


def se_inflation(sigma, m):
    """SE inflation on the log-rate arm coefficient from reviewer noise."""
    return math.sqrt(1.0 + m * (math.exp(sigma * sigma) - 1.0))


def main():
    rng = random.Random(11)
    C = json.load(open(os.path.join(HERE, "concordance.json")))
    block = C.get("all9") or C.get("all8")
    maj = block["maj"]

    # observed paired log-differences on MAJOR counts
    d = maj["noise"]["raw_diffs"] if maj.get("noise") else []
    k_obs = len(d)
    s2 = sum(x * x for x in d) / (2.0 * k_obs) if k_obs else 0.0
    sigma_hat = math.sqrt(s2)

    # mean MAJOR count per row, for the inflation conversion
    import analyze
    Q = analyze.v2_quality(analyze.load())
    ys = [analyze.y_maj(r) for r in Q if analyze.y_maj(r) is not None]
    m = sum(ys) / len(ys)

    print("observed: %d MAJOR pairs, sigma_hat = %.3f, mean MAJOR/row = %.2f"
          % (k_obs, sigma_hat, m))
    print("\nQ1 — precision on reviewer noise, holding the point estimate fixed")
    print("%6s %22s %26s" % ("pairs", "sigma_e 95% CrI", "implied SE inflation"))
    rows = []
    for k in (k_obs, 6, 10, 16, 25, 40, 60):
        s = sigma_posterior(k, s2, rng)
        lo, hi = q(s, 0.025), q(s, 0.975)
        infl_hi = se_inflation(hi, m)
        rows.append((k, lo, hi, infl_hi))
        print("%6d %10.3f – %-9.3f %20s"
              % (k, lo, hi, "up to %+.0f%%" % (100 * (infl_hi - 1))))

    print("\n  (SE inflation is what reviewer noise does to the arm interval.")
    print("   For reference the negative-binomial fit, which absorbs ALL")
    print("   overdispersion from all %d rows rather than %d pairs, widens"
          % (len(ys), k_obs))
    print("   the arm interval by 8%%.)")

    print("\nQ2 — detecting ARM-DEPENDENT reviewer noise (the only kind that biases)")
    print("  Split k pairs evenly by implementer arm; what noise RATIO between")
    print("  arms could be distinguished from 1.0 at 95%?")
    print("%6s %14s" % ("pairs", "detectable ratio"))
    for k in (6, 10, 16, 25, 40, 60):
        half = max(2, k // 2)
        a = sigma_posterior(half, s2, rng)
        b = sigma_posterior(half, s2, rng)
        ratios = sorted(x / y for x, y in zip(a, b))
        hi = ratios[int(0.975 * (len(ratios) - 1))]
        print("%6d %11.2f x" % (k, hi))
    print("\n  A ratio of 1.5x would mean one arm's code draws 50% more")
    print("  reviewer disagreement than the other's — the scale at which")
    print("  differential measurement starts to matter.")

    json.dump({"k_obs": k_obs, "sigma_hat": sigma_hat, "mean_maj": m,
               "q1": [{"k": r[0], "lo": r[1], "hi": r[2], "infl_hi": r[3]}
                      for r in rows]},
              open(os.path.join(HERE, "projection.json"), "w"))
    print("\nwrote projection.json")


if __name__ == "__main__":
    main()
