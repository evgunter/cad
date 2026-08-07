#!/usr/bin/env python3
"""Model definitions: GLMs over a design matrix, fit by adaptive RWM."""
import math

from mcmc import (log_normal_pdf, log_poisson_pmf, log_negbin_pmf,
                  log_bernoulli, logistic, run_chains, rhat, ess, flatten,
                  summarize)


def _eta(beta, xrow):
    s = 0.0
    for b, x in zip(beta, xrow):
        s += b * x
    return s


def fit_count(y, X, family="poisson", prior_sd=None, seed=1,
              n_iter=9000, n_warmup=3000, n_chains=4):
    """log-link count regression. X includes an intercept column."""
    p = len(X[0])
    if prior_sd is None:
        prior_sd = [1.5] + [0.8] * (p - 1)
    nb = family == "negbin"
    npar = p + (1 if nb else 0)

    def logpost(th):
        beta = th[:p]
        lp = 0.0
        for j in range(p):
            lp += log_normal_pdf(beta[j], 0.0, prior_sd[j])
        if nb:
            log_phi = th[p]
            lp += log_normal_pdf(log_phi, 1.5, 1.5)
            phi = math.exp(log_phi)
        for yi, xi in zip(y, X):
            e = _eta(beta, xi)
            if e > 20 or e < -20:
                return -1e300
            mu = math.exp(e)
            lp += log_negbin_pmf(yi, mu, phi) if nb else log_poisson_pmf(yi, mu)
        return lp

    def init(c):
        base = [0.0] * npar
        base[0] = math.log(max(sum(y) / float(len(y)), 0.05))
        for j in range(1, p):
            base[j] = 0.1 * (c - 1.5)
        return base

    chains = run_chains(logpost, init, n_chains, n_iter, n_warmup, seed)
    return chains, npar


def fit_gauss(y, X, seed=1, n_iter=9000, n_warmup=3000, n_chains=4,
              prior_sd=None, sigma_prior=2.0):
    """Linear regression with half-normal prior on sigma. Params: beta..., log_sigma."""
    p = len(X[0])
    if prior_sd is None:
        prior_sd = [10.0] + [2.0] * (p - 1)
    npar = p + 1

    def logpost(th):
        beta = th[:p]
        log_sigma = th[p]
        if log_sigma > 5 or log_sigma < -8:
            return -1e300
        sigma = math.exp(log_sigma)
        lp = 0.0
        for j in range(p):
            lp += log_normal_pdf(beta[j], 0.0, prior_sd[j])
        # half-normal on sigma, + jacobian for log parameterisation
        lp += log_normal_pdf(sigma, 0.0, sigma_prior) + log_sigma
        for yi, xi in zip(y, X):
            lp += log_normal_pdf(yi, _eta(beta, xi), sigma)
        return lp

    ybar = sum(y) / float(len(y))

    def init(c):
        base = [0.0] * npar
        base[0] = ybar
        for j in range(1, p):
            base[j] = 0.05 * (c - 1.5)
        base[p] = math.log(max(0.3, abs(ybar) * 0.2))
        return base

    chains = run_chains(logpost, init, n_chains, n_iter, n_warmup, seed)
    return chains, npar


def fit_logit(y, X, seed=1, n_iter=9000, n_warmup=3000, n_chains=4,
              prior_sd=None):
    p = len(X[0])
    if prior_sd is None:
        prior_sd = [2.0] + [1.5] * (p - 1)

    def logpost(th):
        lp = 0.0
        for j in range(p):
            lp += log_normal_pdf(th[j], 0.0, prior_sd[j])
        for yi, xi in zip(y, X):
            lp += log_bernoulli(yi, logistic(_eta(th, xi)))
        return lp

    def init(c):
        return [0.05 * (c - 1.5)] * p

    chains = run_chains(logpost, init, n_chains, n_iter, n_warmup, seed)
    return chains, p


def fit_ordered_logit(y, X, n_cat, seed=1, n_iter=9000, n_warmup=3000,
                      n_chains=4, prior_sd=None):
    """Proportional-odds model. y in 0..n_cat-1. X has NO intercept column.

    Params: beta (p), c1, log-increments (n_cat-2).
    """
    p = len(X[0])
    if prior_sd is None:
        prior_sd = [1.5] * p
    n_cut = n_cat - 1
    npar = p + n_cut

    def cutpoints(th):
        cs = [th[p]]
        for k in range(1, n_cut):
            cs.append(cs[-1] + math.exp(th[p + k]))
        return cs

    def logpost(th):
        lp = 0.0
        for j in range(p):
            lp += log_normal_pdf(th[j], 0.0, prior_sd[j])
        lp += log_normal_pdf(th[p], 0.0, 4.0)
        for k in range(1, n_cut):
            if th[p + k] > 4 or th[p + k] < -8:
                return -1e300
            lp += log_normal_pdf(th[p + k], 0.0, 2.0)
        cs = cutpoints(th)
        beta = th[:p]
        for yi, xi in zip(y, X):
            e = _eta(beta, xi)
            if yi == 0:
                pr = logistic(cs[0] - e)
            elif yi == n_cat - 1:
                pr = 1.0 - logistic(cs[-1] - e)
            else:
                pr = logistic(cs[yi] - e) - logistic(cs[yi - 1] - e)
            if pr <= 1e-12:
                return -1e300
            lp += math.log(pr)
        return lp

    def init(c):
        base = [0.05 * (c - 1.5)] * p
        base.append(-1.0)
        for _ in range(1, n_cut):
            base.append(0.0)
        return base

    chains = run_chains(logpost, init, n_chains, n_iter, n_warmup, seed)
    return chains, npar


def diagnose(chains, npar, names):
    out = []
    for j in range(npar):
        out.append({
            "param": names[j] if j < len(names) else "p%d" % j,
            "rhat": round(rhat(chains, j), 4),
            "ess": round(ess(chains, j), 1),
        })
    return out


def contrast(chains, j, transform=None):
    s = flatten(chains, j)
    if transform:
        s = [transform(x) for x in s]
    return summarize(s), s
