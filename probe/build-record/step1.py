#!/usr/bin/env python3
"""Step 1's numbers: f64 against the box scalar at the nominal, over every
distinct build the probe saw.  Usage: step1.py <dir> [<dir> ...]"""
import collections, glob, json, os, sys

def rows(dirs, kind, lane=None):
    seen = {}
    for d in dirs:
        for p in glob.glob(os.path.join(d, "*.jsonl")):
            for raw in open(p):
                try:
                    l = json.loads(raw)
                except json.JSONDecodeError:
                    continue
                if l.get("k") != kind or (lane and l.get("lane") != lane):
                    continue
                key = (l.get("a_strict_hash") or l["doc"], l["rec_hash"], l["strict_hash"])
                seen.setdefault(key, l)
    return list(seen.values())

def turn_class(log):
    if log["turned"]:
        return "a choice turned (first_turn)"
    if any(p[1] == "mark" for p in log["parted"]):
        return "parted at a marked query"
    if log["parted"]:
        return "parted elsewhere (step or end)"
    if log["check_turned"]:
        return "choices agree, a check turned"
    if log["check_parted"]:
        return "choices agree, check sequences part"
    return "?"

def report(name, rs, corpus_only=False):
    if corpus_only:
        rs = [l for l in rs if "corpus" in l.get("thread", "")]
    n = len(rs)
    if not n:
        return
    same_standing = [l for l in rs if not l["log"]["standing"]]
    decided = [l for l in same_standing if l["log"]["esc_b"] == 0]
    print(f"\n## {name}: {n} distinct builds")
    print(f"standing equal in both lanes: {len(same_standing)}; of those")
    print(f"  record differs: {sum(not l['rec_eq'] for l in same_standing)}")
    print(f"  name tables differ: {sum(not l['names_eq'] for l in same_standing)}")
    print(f"  extras differ: {sum(not l['extras_eq'] for l in same_standing)}")
    print(f"  unnamed cells, either lane: {sum(any(l['unnamed']) or any(l['unnamed_a']) for l in rs)}")
    print(f"  whole log differs: {sum(not l['log']['strict_equal'] for l in same_standing)}")
    print(f"  box lane holds an escalation: {len(same_standing) - len(decided)}")
    bad = [l for l in decided if not l["log"]["strict_equal"]]
    print(f"box lane fully decided, standing equal: {len(decided)}; whole log differs in {len(bad)} "
          f"({100*len(bad)/max(1,len(decided)):.1f}%), choices differ in {sum(not l['log']['choices_equal'] for l in bad)}, "
          f"marks differ in {sum(not l['log']['marks_equal'] for l in decided)}")
    c = collections.Counter(turn_class(l["log"]) for l in bad)
    for k, v in c.most_common():
        print(f"    {v:6d}  {k}")
    c = collections.Counter()
    for l in bad:
        f = l["log"]["first"]
        c[(f["a"].rsplit(" ", 2)[0] + " (" + f["a"].rsplit(" ", 1)[1] + ")", f["b"].split(" ")[0], f["kind"])] += 1
    print("  first difference (f64's entry, the box lane's, how):")
    for k, v in c.most_common(14):
        print(f"    {v:6d}  {k}")
    st = collections.Counter()
    for l in rs:
        for s in l["log"]["standing"][:1]:
            st[(s[1], s[2], "escalation logged" if l["log"]["esc_b"] else "no escalation")] += 1
    print(f"standing differs: {n - len(same_standing)}; first differing node (f64, box lane):")
    for k, v in st.most_common(12):
        print(f"    {v:6d}  {k}")
    e = collections.Counter()
    for l in rs:
        if l["log"]["esc_b"] and l["log"]["first_esc_b"]:
            e[l["log"]["first_esc_b"][1]] += 1
    print(f"box lane's first escalated predicate at the nominal ({sum(e.values())} builds): {e.most_common(8)}")
    print(f"f64 lane holds an escalation: {sum(l['log']['esc_a'] > 0 for l in rs)}")

dirs = sys.argv[1:]
par = rows(dirs, "parity")
report("f64 vs Interval, every top-level f64 evaluation", par)
report("... the subset from tests named *corpus*", par, corpus_only=True)
report("drive witnesses, zero-width box, Interval", rows(dirs, "zero", "Interval"))
report("drive witnesses, zero-width box, Sym<Interval>", rows(dirs, "zero", "Sym<Interval>"))
