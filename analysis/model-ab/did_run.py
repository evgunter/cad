#!/usr/bin/env python3
"""One DiD fit per invocation, so the sampler's draws never co-reside.

Usage: did_run.py <all|cond|v6>
"""
import csv, math, sys
sys.path.insert(0, '.')
import analyze
from models import fit_count
from mcmc import flatten, summarize

mode = sys.argv[1]
slots = {r['row_id']: r for r in csv.DictReader(open('labels/reviewer-slots.csv'))}
rows = [r for r in analyze.load()
        if r['blinded_lane'] == '1' and r['arm'] in ('fable', 'opus')]
for r in rows:
    s = slots.get(r['row_id'])
    r['r1_model'] = s['r1_model'] if s else None

sel = [r for r in rows
       if analyze.y_maj(r) is not None and r['era'] in ('pre_51', 'post_51')]
cond = mode in ('cond', 'v6')
if cond:
    sel = [r for r in sel if r.get('r1_model')]
if mode == 'v6':
    sel = [r for r in sel if r.get('instrument') == 'v6' or r.get('protocol') == 'v6']

y = [analyze.y_maj(r) for r in sel]
X = []
for r in sel:
    o = 1.0 if r['arm'] == 'opus' else 0.0
    p = 1.0 if r['era'] == 'post_51' else 0.0
    row = [1.0, o, p, o * p, analyze.dS(r), analyze.dL(r)]
    if cond:
        row.append(1.0 if r['r1_model'] == 'opus' else 0.0)
    X.append(row)

psd = [1.5, 0.8, 0.8, 1.0, 1.0, 1.0] + ([1.0] if cond else [])
ch, npar = fit_count(y, X, prior_sd=psd, n_iter=6000, n_warmup=2000, n_chains=2)
s = summarize([math.exp(v) for v in flatten(ch, 3)])
nf = sum(1 for r in sel if r['arm'] == 'fable')
print("%-34s n=%-4d (fable %2d)  DiD %.2f [%.2f, %.2f]"
      % (mode, len(y), nf, s['median'], s['q025'], s['q975']))
if cond:
    rv = summarize([math.exp(v) for v in flatten(ch, 6)])
    print("    opus-reviewer-in-R1 multiplies the count by %.2f [%.2f, %.2f]"
          % (rv['median'], rv['q025'], rv['q975']))
