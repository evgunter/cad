#!/usr/bin/env python3
"""Minimal dependency-free Bayesian machinery.

No numpy/scipy on this box, so: componentwise adaptive random-walk
Metropolis, multi-chain, with R-hat and bulk-ESS diagnostics. Adequate
for the 3-8 parameter GLMs this analysis needs at n<=45.
"""
import math
import random

LOG2PI = math.log(2.0 * math.pi)


# ---------- log densities ----------

def log_normal_pdf(x, mu, sd):
    z = (x - mu) / sd
    return -0.5 * (z * z + LOG2PI) - math.log(sd)


def log_poisson_pmf(k, lam):
    if lam <= 0:
        return -1e300
    return k * math.log(lam) - lam - math.lgamma(k + 1.0)


def log_negbin_pmf(k, mu, phi):
    """NB2: mean mu, variance mu + mu^2/phi."""
    if mu <= 0 or phi <= 0:
        return -1e300
    return (math.lgamma(k + phi) - math.lgamma(phi) - math.lgamma(k + 1.0)
            + phi * (math.log(phi) - math.log(phi + mu))
            + k * (math.log(mu) - math.log(phi + mu)))


def log_bernoulli(y, p):
    eps = 1e-12
    p = min(max(p, eps), 1.0 - eps)
    return math.log(p) if y else math.log(1.0 - p)


def logistic(x):
    if x >= 0:
        return 1.0 / (1.0 + math.exp(-x))
    e = math.exp(x)
    return e / (1.0 + e)


def log_half_normal(x, sd):
    if x <= 0:
        return -1e300
    return log_normal_pdf(x, 0.0, sd) + math.log(2.0)


# ---------- sampler ----------

def metropolis(logpost, init, n_iter=40000, n_warmup=10000, seed=0,
               init_scale=None):
    """Componentwise adaptive random-walk Metropolis. Returns list of draws."""
    rng = random.Random(seed)
    d = len(init)
    x = list(init)
    lp = logpost(x)
    if lp <= -1e299 or lp != lp:
        raise ValueError("initial point has zero posterior density")
    step = list(init_scale) if init_scale else [0.4] * d
    acc = [0] * d
    att = [0] * d
    draws = []
    for it in range(n_iter):
        for j in range(d):
            prop = list(x)
            prop[j] = x[j] + rng.gauss(0.0, step[j])
            lp_prop = logpost(prop)
            att[j] += 1
            if lp_prop == lp_prop and math.log(rng.random() + 1e-300) < lp_prop - lp:
                x, lp = prop, lp_prop
                acc[j] += 1
        # Robbins-Monro style adaptation, warmup only
        if it < n_warmup and (it + 1) % 50 == 0:
            for j in range(d):
                rate = acc[j] / float(att[j])
                # target 0.44 for componentwise updates
                step[j] *= math.exp((rate - 0.44) * 0.6)
                step[j] = min(max(step[j], 1e-4), 50.0)
                acc[j] = 0
                att[j] = 0
        if it >= n_warmup:
            draws.append(list(x))
    return draws


def run_chains(logpost, init_fn, n_chains=4, n_iter=40000, n_warmup=10000,
               seed=0):
    chains = []
    for c in range(n_chains):
        chains.append(metropolis(logpost, init_fn(c), n_iter, n_warmup,
                                 seed=seed * 1000 + c))
    return chains


# ---------- diagnostics ----------

def _mean(v):
    return sum(v) / float(len(v))


def _var(v):
    m = _mean(v)
    return sum((x - m) ** 2 for x in v) / float(len(v) - 1)


def rhat(chains, j):
    """Split-R-hat for parameter j."""
    segs = []
    for ch in chains:
        h = len(ch) // 2
        segs.append([d[j] for d in ch[:h]])
        segs.append([d[j] for d in ch[h:2 * h]])
    m = len(segs)
    n = len(segs[0])
    means = [_mean(s) for s in segs]
    variances = [_var(s) for s in segs]
    W = _mean(variances)
    B = n * _var(means)
    if W <= 0:
        return float('nan')
    var_hat = ((n - 1) / float(n)) * W + B / float(n)
    return math.sqrt(var_hat / W)


def ess(chains, j):
    """Crude bulk-ESS via initial positive sequence on the pooled chains."""
    m = len(chains)
    n = len(chains[0])
    series = [[d[j] for d in ch] for ch in chains]
    W = _mean([_var(s) for s in series])
    if W <= 0:
        return 0.0
    means = [_mean(s) for s in series]
    B = n * _var(means) if m > 1 else 0.0
    var_hat = ((n - 1) / float(n)) * W + B / float(n)
    max_lag = min(400, n - 2)
    rho_sum = 0.0
    t = 1
    while t < max_lag:
        # average autocorrelation across chains at lag t and t+1
        def rho_at(lag):
            tot = 0.0
            for s in series:
                mu = _mean(s)
                num = sum((s[i] - mu) * (s[i + lag] - mu)
                          for i in range(n - lag)) / float(n - lag)
                tot += num
            return (tot / m) / var_hat
        r1 = rho_at(t)
        r2 = rho_at(t + 1) if t + 1 < max_lag else -1.0
        if r1 + r2 < 0:
            break
        rho_sum += r1 + r2
        t += 2
    return (m * n) / (1.0 + 2.0 * rho_sum)


# ---------- summaries ----------

def quantile(sorted_v, q):
    if not sorted_v:
        return float('nan')
    idx = q * (len(sorted_v) - 1)
    lo = int(math.floor(idx))
    hi = int(math.ceil(idx))
    if lo == hi:
        return sorted_v[lo]
    return sorted_v[lo] + (sorted_v[hi] - sorted_v[lo]) * (idx - lo)


def summarize(samples):
    s = sorted(samples)
    return {
        "mean": _mean(samples),
        "median": quantile(s, 0.5),
        "q025": quantile(s, 0.025),
        "q10": quantile(s, 0.10),
        "q90": quantile(s, 0.90),
        "q975": quantile(s, 0.975),
        "p_gt0": sum(1 for x in samples if x > 0) / float(len(samples)),
        "n": len(samples),
    }


def flatten(chains, j):
    return [d[j] for ch in chains for d in ch]
