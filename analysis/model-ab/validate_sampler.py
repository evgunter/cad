#!/usr/bin/env python3
"""Independent check that the hand-rolled Metropolis sampler is correct.

Fits a 2-parameter Poisson log-link model (intercept + arm) to the real
MAJOR-count data two ways:
  1. the MCMC sampler used for the report
  2. deterministic 2-D grid quadrature of the same posterior
and compares the marginal posterior of the arm coefficient. If the
sampler is right these agree to ~2 decimal places.
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import analyze
from mcmc import log_normal_pdf, log_poisson_pmf, flatten, summarize, quantile
from models import fit_count


def main():
    rows = analyze.v2_quality(analyze.load())
    data = [(analyze.y_maj(r), analyze.opus(r)) for r in rows
            if analyze.y_maj(r) is not None]
    print("n = %d, total MAJORs = %d" % (len(data), int(sum(d[0] for d in data))))

    def loglik(a, b):
        s = 0.0
        for y, x in data:
            mu = math.exp(a + b * x)
            s += log_poisson_pmf(y, mu)
        return s

    def logprior(a, b):
        return log_normal_pdf(a, 0.0, 1.5) + log_normal_pdf(b, 0.0, 0.8)

    # ---- grid quadrature
    NA, NB = 601, 601
    a_lo, a_hi, b_lo, b_hi = -3.0, 1.5, -3.0, 3.0
    da = (a_hi - a_lo) / (NA - 1)
    db = (b_hi - b_lo) / (NB - 1)
    grid = []
    best = -1e300
    for i in range(NA):
        a = a_lo + i * da
        row = []
        for j in range(NB):
            b = b_lo + j * db
            lp = loglik(a, b) + logprior(a, b)
            best = max(best, lp)
            row.append(lp)
        grid.append(row)
    # marginal over b
    marg = [0.0] * NB
    for i in range(NA):
        for j in range(NB):
            marg[j] += math.exp(grid[i][j] - best)
    tot = sum(marg) * db
    marg = [m / tot for m in marg]

    # grid summaries
    mean_b = sum((b_lo + j * db) * marg[j] for j in range(NB)) * db
    cdf, cum = [], 0.0
    for j in range(NB):
        cum += marg[j] * db
        cdf.append(cum)

    def grid_q(p):
        for j in range(NB):
            if cdf[j] >= p:
                return b_lo + j * db
        return b_hi

    p_gt0 = 1.0 - cdf[int((0.0 - b_lo) / db)]

    # ---- MCMC
    y = [d[0] for d in data]
    X = [[1.0, d[1]] for d in data]
    chains, npar = fit_count(y, X, prior_sd=[1.5, 0.8])
    s = flatten(chains, 1)
    mc = summarize(s)

    print("\n                     grid quadrature      MCMC sampler")
    print("  posterior mean b   %14.4f %17.4f" % (mean_b, mc["mean"]))
    print("  2.5%%              %14.4f %17.4f" % (grid_q(0.025), mc["q025"]))
    print("  50%%               %14.4f %17.4f" % (grid_q(0.5), mc["median"]))
    print("  97.5%%             %14.4f %17.4f" % (grid_q(0.975), mc["q975"]))
    print("  P(b > 0)           %14.4f %17.4f" % (p_gt0, mc["p_gt0"]))

    diffs = [abs(mean_b - mc["mean"]), abs(grid_q(0.025) - mc["q025"]),
             abs(grid_q(0.975) - mc["q975"]), abs(p_gt0 - mc["p_gt0"])]
    print("\n  max abs discrepancy: %.4f  -> %s"
          % (max(diffs), "SAMPLER OK" if max(diffs) < 0.05 else "INVESTIGATE"))


if __name__ == "__main__":
    main()
