#!/usr/bin/env python3
"""Items 2 and 4: every leaf replay the probe saw, re-sorted by the record.
Usage: leaves.py <dir> [<dir> ...] [--only SUBSTR] [--per-test]"""
import collections, glob, json, os, struct, sys

args = [a for a in sys.argv[1:] if not a.startswith("--")]
only = None
if "--only" in sys.argv:
    only = sys.argv[sys.argv.index("--only") + 1]
    args.remove(only)
per_test = "--per-test" in sys.argv


def box_of(s):
    out = {}
    for part in s.split(" "):
        name, span = part.split("=")
        lo, hi = span.strip("[]").split(",")
        f = lambda h: struct.unpack(">d", bytes.fromhex(h))[0]
        out[name] = (f(lo), f(hi))
    return out


def inside(inner, outer):
    return all(k in outer and outer[k][0] <= v[0] and v[1] <= outer[k][1] for k, v in inner.items())


def today(l, fate):
    v = l["verdict"]
    if v == "bisect":
        return "split" if fate is None else fate.split(" ")[0]
    if v == "refused":
        return l["detail"].split(" ")[0]
    return {"flip": "flip_crossing"}.get(v, v)


def decidedness(l):
    if l["log"]["esc_b"] > 0 or l["verdict"] == "refused":
        return "undecided"
    if l["verdict"] == "bisect":
        return "undecided" if " Undecided" in l["detail"] else "a node fails, no escalation logged"
    return "decided"


def by_record(l):
    d = decidedness(l)
    if d != "decided":
        return d
    return "same build" if l["rec_eq"] else "another build"


def log_kind(l):
    log = l["log"]
    if log["standing"]:
        return "standing"
    if log["turned"]:
        return "a choice turned"
    if any(p[1] == "mark" for p in log["parted"]):
        return "parted at a pruning query (mark)"
    if log["parted"]:
        return "parted elsewhere"
    if log["check_turned"]:
        return "a check turned"
    if log["check_parted"]:
        return "check sequences part"
    return "logs equal"


groups = []
for d in args:
    for path in sorted(glob.glob(os.path.join(d, "*.jsonl"))):
        g = collections.OrderedDict()
        test = ""
        for raw in open(path):
            try:
                l = json.loads(raw)
            except json.JSONDecodeError:
                continue
            k = l.get("k")
            if k == "proc":
                test = " ".join(a for a in l["args"][1:] if not a.startswith("--"))
            if k in ("drive", "leaf", "fate", "final", "drive_end"):
                e = g.setdefault(l["w"], {"drive": None, "leaf": [], "fate": {}, "final": {}, "test": l.get("thread") or test})
                if l.get("thread") and l["thread"] != "main":
                    e["test"] = l["thread"]
                if k == "drive":
                    e["drive"] = l
                elif k == "leaf":
                    e["leaf"].append(l)
                elif k == "fate":
                    e["fate"][l["box"]] = l["reason"]
                elif k == "final":
                    e["final"][l["box"]] = l
        groups.extend(g.values())
if only:
    groups = [g for g in groups if only in g["test"]]

n_leaf = sum(len(g["leaf"]) for g in groups)
print(f"witnesses: {len(groups)} ({sum(1 for g in groups if g['drive'])} drives, the rest range searches); leaf replays: {n_leaf}")

count = collections.Counter()
mass = collections.defaultdict(float)
flip_kind = collections.Counter()
flip_pred = collections.Counter()
decided_kind = collections.Counter()
rec_ne_standing_eq = 0
same_choice_other_record = 0
same_names_other_record = 0
same_record_other_extras = collections.Counter()
under_failed = [0.0, 0.0, 0]
rows = []
for g in groups:
    choice = collections.defaultdict(set)
    names = collections.defaultdict(set)
    w = g["drive"]
    if w:
        choice[w["w_hashes"][0]].add(w["w_rec_hash"])
        names[w["w_names_hash"]].add(w["w_rec_hash"])
    tm = collections.defaultdict(float)
    tc = collections.Counter()
    cert_boxes = [(box_of(b), f["mass"] or 0.0) for b, f in g["final"].items() if f["class"] == "certified"]
    for l in g["leaf"]:
        t = today(l, g["fate"].get(l["box"]))
        r = by_record(l)
        count[(t, r)] += 1
        tc[(t, r)] += 1
        fin = g["final"].get(l["box"])
        if fin and fin.get("mass") is not None:
            mass[(fin["class"], r)] += fin["mass"]
            tm[(fin["class"], r)] += fin["mass"]
        if not l["rec_eq"] and not l["log"]["standing"]:
            rec_ne_standing_eq += 1
        if decidedness(l) == "decided":
            choice[l["choice_hash"]].add(l["rec_hash"])
            names[l["names_hash"]].add(l["rec_hash"])
            decided_kind[(log_kind(l), "same build" if l["rec_eq"] else "another build")] += 1
            if l["rec_eq"] and not l["extras_eq"]:
                a = l["extras_first"]
                same_record_other_extras[(a[1].split(" ")[0], a[1].split(" ")[-1], a[2].split(" ")[-1])] += 1
        if l["verdict"] == "flip":
            k = log_kind(l) + (" [evidence names nothing]" if l["evidence_empty"] else "")
            flip_kind[(k, "same build" if l["rec_eq"] else "another build")] += 1
            for tp in l["log"]["turned"]:
                flip_pred[tp[1]] += 1
        if t == "split" and r == "a node fails, no escalation logged" and cert_boxes:
            b = box_of(l["box"])
            m = sum(mm for cb, mm in cert_boxes if inside(cb, b))
            if m > 0:
                under_failed[0] += m
                under_failed[2] += 1
    same_choice_other_record += sum(1 for v in choice.values() if len(v) > 1)
    same_names_other_record += sum(1 for v in names.values() if len(v) > 1)
    rows.append((g["test"], tc, tm))

print("\n# leaf replays: today's fate x the record")
for k, v in sorted(count.items()):
    print(f"  {v:7d}  {k[0]:20s} {k[1]}")
print("\n# final leaves' mass (sum over the drives): today's class x the record")
tot = sum(mass.values())
for k, v in sorted(mass.items()):
    print(f"  {v:12.6f} ({100*v/max(tot,1e-300):5.1f}%)  {k[0]:20s} {k[1]}")
print("\n# today's FlipCrossing leaves by what the log shows x the record")
for k, v in sorted(flip_kind.items(), key=lambda kv: -kv[1]):
    print(f"  {v:7d}  {k}")
print("  turned predicates named:", flip_pred.most_common())
print("\n# every decided leaf, by what its log shows against the witness's x the record")
for k, v in sorted(decided_kind.items(), key=lambda kv: -kv[1]):
    print(f"  {v:7d}  {k}")
print(f"\n# leaves whose record differs while every standing is equal: {rec_ne_standing_eq}")
print(f"# groups of decided leaves (with the witness) of EQUAL choices and DIFFERENT records: {same_choice_other_record}")
print(f"# groups of EQUAL standing and name tables and DIFFERENT records: {same_names_other_record}")
print(f"# decided leaves with the witness's record and other extras: {dict(same_record_other_extras)}")
print(f"# boxes split today because a node failed with no escalation logged, that hold certified mass below them: "
      f"{under_failed[2]} boxes (certified mass counted once per ancestor: {under_failed[0]:.4f})")
if per_test:
    print("\n# per witness (those with any flip or refusal)")
    for test, tc, tm in rows:
        if any(k[0] not in ("certified", "split") for k in tc):
            print(f"  {test[:90]}")
            for k, v in sorted(tc.items()):
                print(f"      {v:6d} leaves  {tm.get((k[0] if k[0] != 'split' else '', k[1]), 0.0):10.6f} mass  {k[0]:18s} {k[1]}")
