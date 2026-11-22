#!/usr/bin/env python3
"""Per fast-set test: PR-gate default-eps times (median, p90, n) beside its
nightly opt-2 default-eps time, and counts at each threshold.

usage: analyze.py <repo checkout> <times.tsv> <out.tsv> <nightly junit.xml>...

Kept for the drift check PR 7397 proposes, which reuses this reader.
"""

import statistics
import sys
import xml.etree.ElementTree as ET
from collections import defaultdict

repo, times_path, out_path, *junits = sys.argv[1:]
sys.path.insert(0, f"{repo}/scripts")
import slow_set  # noqa: E402

slow = set(
    slow_set.entries(slow_set.ci_filter(open(f"{repo}/.config/nextest.toml").read()))
)
slow_keys = {f"{b} {n}" for b, n in slow}

pr = defaultdict(list)
shard_of, runs = {}, set()
for line in open(times_path):
    run, job, shard, sha, concl, test, secs = line.rstrip("\n").split("\t")
    pr[test].append(float(secs))
    shard_of[test] = shard
    runs.add(run)

night = defaultdict(list)
for j in junits:
    for tc in ET.parse(j).iter("testcase"):  # noqa: S314 (our own CI artifacts)
        night[f"{tc.get('classname')} {tc.get('name')}"].append(float(tc.get("time")))


def p90(xs):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(0.9 * len(xs)))] if len(xs) >= 2 else xs[0]


rows = []
for test, xs in pr.items():
    if test in slow_keys:
        continue
    ny = statistics.median(night[test]) if night.get(test) else None
    rows.append(
        (p90(xs), statistics.median(xs), max(xs), len(xs), ny, shard_of[test], test)
    )
rows.sort(key=lambda r: (r[0], r[1]), reverse=True)
with open(out_path, "w") as f:
    f.write("p90\tmedian\tmax\tn\tnightly_opt2_median\tshard\ttest\n")
    for r in rows:
        f.write(
            "\t".join(
                "" if v is None else (f"{v:.3f}" if isinstance(v, float) else str(v))
                for v in r
            )
            + "\n"
        )

print(f"runs={len(runs)} fast-set tests seen={len(rows)} slow-set entries={len(slow)}")
total_med = sum(r[1] for r in rows)
print(f"sum of medians over seen fast-set tests: {total_med:.0f} s")
for thr in (0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0):
    hit = [r for r in rows if r[0] >= thr]
    nhit = [r for r in hit if r[4] is not None and r[4] >= thr]
    nlow = [r for r in hit if r[4] is not None and r[4] < 0.1]
    med = [r for r in rows if r[1] >= thr]
    print(
        f"p90>={thr}: {len(hit):4d} (n>=3: {sum(1 for r in hit if r[3] >= 3):4d}; "
        f"median>={thr}: {len(med):4d}; nightly opt2 also >={thr}: {len(nhit):3d}; nightly <0.1 s: {len(nlow):4d}; "
        f"sum of their medians {sum(r[1] for r in hit):.0f} s)"
    )
fast_night = {t: statistics.median(v) for t, v in night.items() if t not in slow_keys}
for thr in (0.25, 0.5, 1.0):
    print(
        f"nightly opt2 fast-set tests >= {thr}: {sum(1 for v in fast_night.values() if v >= thr)} of {len(fast_night)}"
    )
never = [t for t in fast_night if t not in pr]
print(f"fast-set tests in nightly but in no sampled PR run: {len(never)}")
