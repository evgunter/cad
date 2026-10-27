#!/usr/bin/env python3
"""Slim per-item CSV for the dashboard: worked items (implementing PR touches code), post-2026-09-27."""
import csv, os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(HERE, 'items.csv'))))
cols = ['item', 'program', 'cost', 'impl_pr', 'bundle_size', 'active_T30', 'active_T60', 'active_T90', 'active_T120',
        'active_T240', 'commits', 'code_lines', 'work_lines', 'open_to_merge_h', 'followup_points']
src = dict(active_T30='pr_active_min_T30', active_T60='pr_active_min_T60', active_T90='pr_active_min_T90',
           active_T120='pr_active_min_T120', active_T240='pr_active_min_T240', commits='pr_n_commits',
           code_lines='pr_lines_code', work_lines='pr_lines_work', open_to_merge_h='pr_open_to_merge_h')
out = []
for r in rows:
    if not r['impl_pr'] or float(r['pr_lines_code'] or 0) <= 0 or r['post_redef'] != 'True' or r['cost'] not in 'EMH':
        continue
    o = {c: r.get(src.get(c, c), '') for c in cols}
    out.append(o)
with open(os.path.join(HERE, 'dash_items.csv'), 'w', newline='') as f:
    w = csv.DictWriter(f, cols); w.writeheader(); w.writerows(out)
print(len(out))
