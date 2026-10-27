#!/usr/bin/env python3
"""Per-tier summaries of items.csv. Prints tables and writes summary.json."""
import csv, os, json, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(HERE, 'items.csv'))))

def f(x):
    try:
        return float(x)
    except (TypeError, ValueError):
        return None

def q(v, p):
    v = sorted(v); k = (len(v) - 1) * p; i = int(k)
    return v[i] if i + 1 >= len(v) else v[i] + (v[i + 1] - v[i]) * (k - i)

def worked(r):
    return r['impl_pr'] and (f(r['pr_lines_code']) or 0) > 0

POP = {
    'solo_post': lambda r: worked(r) and r['bundle_size'] == '1' and r['post_redef'] == 'True',
    'solo_all': lambda r: worked(r) and r['bundle_size'] == '1',
    'all_post_eqshare': lambda r: worked(r) and r['post_redef'] == 'True',
}
PROXIES = ['pr_active_min_T30', 'pr_active_min_T60', 'pr_active_min_T90', 'pr_active_min_T120', 'pr_active_min_T240',
           'pr_commit_active_min_T90', 'pr_raw_elapsed_h', 'pr_open_to_merge_h',
           'pr_n_commits', 'pr_lines_code', 'pr_lines_prose', 'pr_lines_work', 'pr_lines_data',
           'pr_files', 'pr_review_entries', 'pr_fix_passes', 'pr_findings', 'pr_log_words', 'pr_dual_findings',
           'followups_new_in_pr', 'followups_parent_refs']
SHARED = {'followups_parent_refs'}   # item-level, not divided by bundle

out = {}
for pname, pred in POP.items():
    sel = [r for r in rows if pred(r)]
    out[pname] = {}
    for proxy in PROXIES:
        res = {}
        for tier in 'EMHD':
            vals = []
            for r in sel:
                if r['cost'] != tier:
                    continue
                v = f(r.get(proxy))
                if v is None:
                    continue
                if pname == 'all_post_eqshare' and proxy not in SHARED:
                    v = v / float(r['bundle_size'])
                vals.append(v)
            if vals:
                res[tier] = dict(n=len(vals), median=q(vals, .5), mean=st.mean(vals), p25=q(vals, .25), p75=q(vals, .75), p90=q(vals, .9))
        if 'E' in res:
            for t in 'MH':
                if t in res:
                    res[t + ':E_median'] = res[t]['median'] / res['E']['median'] if res['E']['median'] else None
                    res[t + ':E_mean'] = res[t]['mean'] / res['E']['mean'] if res['E']['mean'] else None
        out[pname][proxy] = res
json.dump(out, open(os.path.join(HERE, 'summary.json'), 'w'), indent=1)

for pname in POP:
    print('\n==', pname)
    for proxy in PROXIES:
        res = out[pname][proxy]
        s = []
        for t in 'EMHD':
            if t in res:
                d = res[t]; s.append(f"{t} n={d['n']} med={d['median']:.1f} mean={d['mean']:.1f} [{d['p25']:.1f},{d['p75']:.1f}]")
        rat = ' '.join(f"{k}={res[k]:.2f}" for k in ('M:E_median', 'H:E_median', 'M:E_mean', 'H:E_mean') if res.get(k) is not None)
        print(f'{proxy:28s} ' + ' | '.join(s) + '  ' + rat)
