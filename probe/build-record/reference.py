#!/usr/bin/env python3
"""M13b items 1, 2 and 4: each witness's same-lane zero-width replay (the
reference), and today's decided leaves re-sorted against it.
Usage: reference.py <dir> [<dir> ...]"""
import collections, glob, json, os, sys


def whole_class(log):
    """By the whole log, checks read."""
    if log["standing"]:
        return ("standing", log["standing"][0][1] + " -> " + log["standing"][0][2])
    if log["strict_equal"]:
        return ("same", "")
    f = log["first"]
    if f and f["kind"] == "turned":
        role = f["a"].rsplit(" ", 1)[1]
        return ("turned " + role.lower(), f["a"].split(" ")[0] + " " + f["a"].split(" ")[1] + "->" + f["b"].split(" ")[1])
    return ("parted", (f["a"].rsplit(" ", 2)[0] + " | " + f["b"].rsplit(" ", 2)[0]) if f else "")


def choice_class(log):
    """By choices and standings only (first_turn per node)."""
    if log["standing"]:
        return ("standing", log["standing"][0][1] + " -> " + log["standing"][0][2])
    if log["turned"]:
        t = log["turned"][0]
        return ("turned choice", f"{t[1]} {t[2]}->{t[3]}")
    if log["parted"]:
        p = log["parted"][0]
        return ("parted", f"{p[1]} {p[2]} step {p[3]}")
    if not log["choices_equal"]:
        return ("parted", "choices differ")
    return ("same", "")


def decided(l):
    if l["log"]["esc_b"] > 0 or l["log"].get("esc_all_b", 0) > 0 or l["verdict"] in ("refused", "bisect"):
        return False
    return True


W = []
for d in sys.argv[1:]:
    for p in sorted(glob.glob(os.path.join(d, "*.jsonl"))):
        g = collections.OrderedDict()
        test = ""
        for raw in open(p):
            try:
                l = json.loads(raw)
            except json.JSONDecodeError:
                continue
            k = l.get("k")
            if k == "proc":
                test = " ".join(a for a in l["args"][1:] if not a.startswith("--"))
            if k in ("drive", "ref", "leaf", "final", "drive_end"):
                e = g.setdefault(l["w"], {"drive": None, "ref": None, "leaf": [], "final": [], "test": test})
                if l.get("thread") and l["thread"] != "main":
                    e["test"] = l["thread"]
                if k == "drive":
                    e["drive"] = l
                elif k == "ref":
                    e["ref"] = l
                elif k == "leaf":
                    e["leaf"].append(l)
                elif k == "final":
                    e["final"].append(l)
        W.extend(v for v in g.values() if v["ref"])

print(f"witnesses with a same-lane reference: {len(W)} ({sum(1 for w in W if w['drive'])} drives, {sum(1 for w in W if not w['drive'])} range searches)")
by_lane = collections.Counter(w["ref"]["lane"] for w in W)
print("lanes:", dict(by_lane))

print("\n# 1. the reference against the f64 witness")
und = [w for w in W if w["ref"]["log"].get("esc_all_b", w["ref"]["log"]["esc_b"]) > 0]
print(f"reference undecided (holds an escalation): {len(und)}")
ce = collections.Counter()
for w in und:
    f = w["ref"]["log"]["first_esc_b"]
    ce[f[1] if f else "(assertion node)"] += 1
print("   first escalated predicate:", ce.most_common())
cw = collections.Counter()
cc = collections.Counter()
for w in W:
    cw[whole_class(w["ref"]["log"])] += 1
    cc[choice_class(w["ref"]["log"])] += 1
print("by the whole log, checks read:")
for k, v in cw.most_common():
    print(f"   {v:5d}  {k}")
print("by choices and standings:")
for k, v in cc.most_common():
    print(f"   {v:5d}  {k}")
print(f"record differs from the witness's: {sum(not w['ref']['rec_eq'] for w in W)}")
cp = collections.Counter()
for w in W:
    for e in w["ref"]["log"].get("check_pop", []):
        cp[e[0]] += 1
print("checks whose sign populations differ, reference against witness:", cp.most_common())

rows = collections.defaultdict(lambda: [0, 0.0, []])
for w in W:
    cert = [f for f in w["final"] if f["class"] == "certified"]
    ncert = sum(1 for l in w["leaf"] if l["verdict"] == "certified")
    if not ncert:
        continue
    m = sum(f["mass"] or 0.0 for f in cert)
    keys = []
    if w["ref"]["log"].get("esc_all_b", w["ref"]["log"]["esc_b"]) > 0:
        keys.append("(i) reference undecided")
    c = choice_class(w["ref"]["log"])
    if c[0] == "parted":
        keys.append("(ii) reference parts from the witness in choices")
    if c[0] == "turned choice":
        keys.append("(iii) reference turns a choice against the witness")
    if whole_class(w["ref"]["log"])[0] != "same" and not keys:
        keys.append("(reference differs in checks only)")
    if not keys:
        keys.append("reference decided and equal to the witness")
    for k in keys:
        rows[k][0] += 1
        rows[k][1] += m
        rows[k][2].append(w["test"])
print("\nwitnesses that certify a leaf today, by their reference (count, certified mass of drives):")
for k, v in sorted(rows.items()):
    print(f"   {v[0]:5d}  {v[1]:10.4f}  {k}" + ("   e.g. " + "; ".join(sorted(set(v[2]))[:4]) if not k.startswith("reference decided") else ""))

print("\n# 2. today's decided leaves, against the same-lane reference")
tab = collections.Counter()
diffs_ab = []
diffs_today = collections.Counter()
ex = {}
checks_w = collections.Counter()
checks_r = collections.Counter()
n = 0
for w in W:
    for l in w["leaf"]:
        if not decided(l) or "ref" not in l:
            continue
        n += 1
        a = whole_class(l["ref"]["log"])
        b = choice_class(l["ref"]["log"])
        today = l["verdict"]
        tab[(today, "a:" + a[0], "b:" + b[0])] += 1
        ex.setdefault((today, a[0], b[0]), (w["test"], a[1], b[1], l["ref"]["rec_eq"]))
        if a[0].split(" ")[0] != b[0].split(" ")[0]:
            diffs_ab.append((w["test"], today, a, b))
        ta = "certified" if a[0] == "same" else "flip"
        tb = "certified" if b[0] == "same" else "flip"
        if ta != today:
            diffs_today[("whole record", today, a[0], a[1])] += 1
        if tb != today:
            diffs_today[("choices and standings", today, b[0], b[1])] += 1
        if l["log"]["choices_equal"]:
            for e in l["log"].get("check_pop", []):
                checks_w[e[0]] += 1
        if l["ref"]["log"]["choices_equal"]:
            for e in l["ref"]["log"].get("check_pop", []):
                checks_r[e[0]] += 1
print(f"decided leaves: {n}")
for k, v in sorted(tab.items()):
    t, x, y, r = ex[(k[0], k[1][2:], k[2][2:])]
    print(f"   {v:5d}  today {k[0]:9s} {k[1]:16s} {k[2]:16s}  e.g. {x} / {y}  record equal to the reference's: {r}  [{t[:60]}]")
print(f"leaves whose class differs between (a) and (b): {len(diffs_ab)}")
for d in diffs_ab[:20]:
    print("   ", d)
print("leaves whose class differs from today's verdict:")
for k, v in sorted(diffs_today.items(), key=lambda kv: -kv[1]):
    print(f"   {v:5d}  {k}")
print("\n# 4. decided leaves with equal choices and a check population that differs")
print("   against the f64 witness:", checks_w.most_common())
print("   against the same-lane reference:", checks_r.most_common())
allc = collections.Counter()
for w in W:
    for l in w["leaf"]:
        if decided(l) and l["verdict"] == "flip":
            for e in l["log"].get("check_pop", []):
                allc[e[0]] += 1
print("   checks whose populations differ on today's flip leaves (choices turned or not), against the witness:", allc.most_common())
