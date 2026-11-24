#!/usr/bin/env python3
"""The orchestrator's follow-up tables, from the probe's lines.
Usage: addendum.py <dir> [<dir> ...]"""
import collections, glob, json, os, sys

par = {}
leaves = []
for d in sys.argv[1:]:
    for p in glob.glob(os.path.join(d, "*.jsonl")):
        fates = {}
        mine = []
        for raw in open(p):
            try:
                l = json.loads(raw)
            except json.JSONDecodeError:
                continue
            if l.get("k") == "parity":
                par.setdefault((l["a_strict_hash"], l["rec_hash"], l["strict_hash"]), l)
            elif l.get("k") == "leaf":
                mine.append(l)
            elif l.get("k") == "fate":
                fates[(l["w"], l["box"])] = l["reason"]
        for l in mine:
            l["fate"] = fates.get((l["w"], l["box"]))
        leaves.extend(mine)
par = list(par.values())

print("# 1. decided builds, equal standing and record, different logs: every first difference")
bad = [l for l in par if l["log"]["esc_b"] == 0 and not l["log"]["standing"] and not l["log"]["strict_equal"]]
c = collections.Counter()
for l in bad:
    f = l["log"]["first"]
    c[(f["a"], f["b"], f["kind"])] += 1
print(f"builds: {len(bad)}")
for k, v in c.most_common():
    print(f"  {v:5d}  f64: {k[0]:48s} box lane: {k[1]:48s} ({k[2]})")
ch = [l for l in bad if not l["log"]["choices_equal"]]
pk = collections.Counter()
for l in ch:
    for p in l["log"]["parted"][:1]:
        pk[(p[1], p[2])] += 1
    if not l["log"]["parted"]:
        pk[("turned", json.dumps([t[1:] for t in l["log"]["turned"][:1]]))] += 1
print(f"choices differ in {len(ch)}; how first_turn reads the first node that parts (kind, marked site):")
for k, v in pk.most_common():
    print(f"  {v:5d}  {k}")
print(f"of those, marks also differ in {sum(not l['log']['marks_equal'] for l in ch)}")

print("\n# 2. the box lane's first escalation at the nominal, by predicate: builds under a drive's box / plain")
e = collections.defaultdict(lambda: [0, 0, None])
for l in par:
    f = l["log"]["first_esc_b"]
    if l["log"]["esc_b"] and f:
        r = e[f[1]]
        r[0 if l["boxed"] else 1] += 1
        if r[2] is None and len(f) > 2:
            r[2] = (l["thread"], f[2])
for k, v in sorted(e.items(), key=lambda kv: -(kv[1][0] + kv[1][1])):
    print(f"  {v[0]:5d} boxed {v[1]:5d} plain  {k}" + (f"\n        e.g. {v[2][0]}: {v[2][1]}" if v[2] else ""))

print("\n# 3. leaves with a node that fails where the witness builds, no escalation logged")
fl = [l for l in leaves if l["verdict"] == "bisect" and " Failed" in l["detail"] and l["log"]["esc_b"] == 0]
by = collections.Counter()
up = collections.Counter()
for l in fl:
    cls = l["detail"].split("class: ")[1].split(",")[0]
    cause = l["detail"].split('Words("')[1][:110] if 'Words("' in l["detail"] else ""
    fate = "split" if l["fate"] is None else l["fate"].split(" ")[0]
    by[(fate, cls, cause)] += 1
    up[(fate, "a choice turned at another node" if l["log"]["turned"] else "no turned choice")] += 1
    for t in l["log"]["turned"][:1]:
        up[(fate, "turned: " + t[1])] += 1
print(f"leaves: {len(fl)}")
for k, v in by.most_common(12):
    print(f"  {v:6d}  {k[0]:8s} {k[1]:18s} {k[2]}")
for k, v in sorted(up.items()):
    print(f"  {v:6d}  {k}")
