#!/usr/bin/env python3
"""Reads the build-record probe's JSONL files (one per process, written under
BUILD_RECORD_PROBE_DIR by `editor_core::build_record`) and prints the tables
`probe/M13-REPORT.md` quotes.  Usage: analyze.py <dir> [<dir> ...]"""
import collections
import glob
import json
import os
import sys


def load(dirs):
    procs = []
    for d in dirs:
        for path in sorted(glob.glob(os.path.join(d, "*.jsonl"))):
            lines = []
            with open(path) as f:
                for raw in f:
                    try:
                        lines.append(json.loads(raw))
                    except json.JSONDecodeError:
                        pass
            procs.append((path, lines))
    return procs


def test_of(line, proc_args):
    t = line.get("thread") or ""
    if t and t != "main":
        return t
    return " ".join(a for a in proc_args[1:] if not a.startswith("--"))


def kind_of_first(log):
    """What a log pair's difference is, by the split item 4 asks for."""
    if log["standing"]:
        return "standing"
    if log["turned"]:
        return "turned choice"
    if any(p[1] == "mark" for p in log["parted"]):
        return "parting at a pruning query"
    if log["parted"]:
        return "parting elsewhere"
    if log["check_turned"]:
        return "turned check"
    if log["check_parted"]:
        return "check sequences part"
    if not log["strict_equal"]:
        return "other"
    return "logs equal"


def short(x, n=260):
    s = json.dumps(x) if not isinstance(x, str) else x
    return s if len(s) <= n else s[:n] + "…"


def main():
    procs = load(sys.argv[1:])
    print(f"processes: {len(procs)}")

    # ---------------- step 1: parity + zero-width ----------------
    for kind in ("parity", "zero"):
        seen = {}
        for path, lines in procs:
            args = next((l["args"] for l in lines if l.get("k") == "proc"), [""])
            for l in lines:
                if l.get("k") != kind:
                    continue
                key = (l.get("lane", "Interval"), l.get("a_strict_hash") or l["doc"], l["rec_hash"], l["strict_hash"])
                seen.setdefault(key, (l, test_of(l, args)))
        by_lane = collections.defaultdict(list)
        for (lane, *_), v in seen.items():
            by_lane[lane].append(v)
        for lane, rows in sorted(by_lane.items()):
            n = len(rows)
            allok = [r for r in rows if r[0]["ok_a"][0] == r[0]["ok_a"][1]]
            print(f"\n== {kind} [{lane}]: {n} distinct builds ({len(allok)} with every node built at f64)")
            cnt = collections.Counter()
            for l, _ in rows:
                cnt["record differs"] += not l["rec_eq"]
                cnt["names differ"] += not l["names_eq"]
                cnt["extras differ"] += not l["extras_eq"]
                cnt["whole log differs"] += not l["log"]["strict_equal"]
                cnt["choices differ"] += not l["log"]["choices_equal"]
                cnt["marks differ"] += not l["log"]["marks_equal"]
                cnt["standing differs"] += bool(l["log"]["standing"])
                cnt["unnamed cells (box lane)"] += any(l["unnamed"])
                cnt["unnamed cells (f64)"] += any(l["unnamed_a"])
                cnt["box lane undecided (escalations)"] += l["log"]["esc_b"] > 0
                cnt["f64 escalations"] += l["log"]["esc_a"] > 0
                cnt["record equal, log differs"] += l["rec_eq"] and not l["log"]["strict_equal"]
                cnt["record differs, log equal"] += (not l["rec_eq"]) and l["log"]["strict_equal"]
                cnt["record differs, standing equal"] += (not l["rec_eq"]) and not l["log"]["standing"]
                cnt["extras differ, record equal"] += l["rec_eq"] and not l["extras_eq"]
                cnt["record differs, box lane decided, standing equal"] += (
                    (not l["rec_eq"]) and not l["log"]["standing"] and l["log"]["esc_b"] == 0)
                cnt["log differs, box lane decided, standing equal"] += (
                    (not l["log"]["strict_equal"]) and not l["log"]["standing"] and l["log"]["esc_b"] == 0)
            for k, v in sorted(cnt.items()):
                print(f"   {v:6d}  {k}")
            kinds = collections.Counter()
            ex = {}
            for l, t in rows:
                if not l["log"]["strict_equal"]:
                    k = kind_of_first(l["log"])
                    f = l["log"]["first"]
                    sub = (k, (f or {}).get("kind"), (f or {}).get("a", "").split(" ")[0], (f or {}).get("b", "").split(" ")[0])
                    kinds[sub] += 1
                    ex.setdefault(sub, (t, l["log"]))
            print("   log differences by first difference (class, how, f64's entry, box lane's entry):")
            for k, v in kinds.most_common(40):
                t, log = ex[k]
                print(f"   {v:6d}  {k}   e.g. {t}: first={short(log['first'])} standing={short(log['standing'], 160)}")
            print("   record differences (standing equal), by first line:")
            rk = collections.Counter()
            rex = {}
            for l, t in rows:
                if not l["rec_eq"] and not l["log"]["standing"]:
                    a = l["rec_first"][1].split(" ")
                    b = l["rec_first"][2].split(" ")
                    k = (" ".join(a[:2])[:40], " ".join(b[:2])[:40], l["log"]["esc_b"] > 0)
                    rk[k] += 1
                    rex.setdefault(k, (t, l["rec_first"]))
            for k, v in rk.most_common(30):
                t, f = rex[k]
                print(f"   {v:6d}  {k}   e.g. {t}: {short(f, 700)}")
            print("   extras differences (record equal), by first line:")
            ek = collections.Counter()
            eex = {}
            for l, t in rows:
                if l["rec_eq"] and not l["extras_eq"]:
                    a = l["extras_first"][1]
                    b = l["extras_first"][2]
                    k = (a.split(" ")[1] if " " in a else a, a.rsplit(" ", 3)[-3:] if "carrier" in a or "held" in a else a[:30],
                         b.rsplit(" ", 3)[-3:] if "carrier" in b or "held" in b else b[:30])
                    k = json.dumps(k)[:150]
                    ek[k] += 1
                    eex.setdefault(k, (t, l["extras_first"]))
            for k, v in ek.most_common(30):
                t, f = eex[k]
                print(f"   {v:6d}  {k}   e.g. {t}: {short(f, 500)}")
            # cost
            tr = sorted(l["t_record_us"] for l, _ in rows)
            te = sorted(l["t_eval_us"] for l, _ in rows)
            if tr:
                print(f"   record cost us: median {tr[len(tr)//2]}, p90 {tr[int(len(tr)*.9)]}, max {tr[-1]}; "
                      f"box-lane eval us: median {te[len(te)//2]}, p90 {te[int(len(te)*.9)]}; "
                      f"sum record {sum(tr)/1e6:.2f}s vs sum eval {sum(te)/1e6:.2f}s")

    # ---------------- drives ----------------
    drives = []  # (test, drive line, leaves, fates, finals)
    for path, lines in procs:
        args = next((l["args"] for l in lines if l.get("k") == "proc"), [""])
        by_w = collections.OrderedDict()
        for l in lines:
            k = l.get("k")
            if k in ("drive", "leaf", "fate", "final", "drive_end"):
                d = by_w.setdefault(l["w"], {"drive": None, "leaf": [], "fate": {}, "final": {}, "test": test_of(l, args), "path": path})
                if k == "drive":
                    d["drive"] = l
                elif k == "leaf":
                    d["leaf"].append(l)
                elif k == "fate":
                    d["fate"][l["box"]] = l["reason"]
                elif k == "final":
                    d["final"][l["box"]] = l
        drives.extend(by_w.values())
    print(f"\n== witnesses with leaf lines: {len(drives)} ({sum(1 for d in drives if d['drive'])} are drives; the rest are the range reading's and other callers of the leaf protocol)")

    tot = collections.Counter()
    mass = collections.defaultdict(float)
    flipsplit = collections.Counter()
    flipsplit_rec = collections.Counter()
    flip_ex = {}
    steer = []       # equal choices, different record
    names_only = []  # equal names, different record
    eq_rec_diff_log = collections.Counter()
    eq_rec_diff_log_ex = {}
    extras_split = collections.Counter()
    extras_ex = {}
    costs = []
    seen_leaf = set()
    for d in drives:
        leaves = d["leaf"]
        w = d["drive"]
        groups_choice = collections.defaultdict(set)
        groups_names = collections.defaultdict(set)
        if w:
            groups_choice[w["w_hashes"][0]].add(w["w_rec_hash"])
            groups_names[w["w_names_hash"]].add(w["w_rec_hash"])
        for l in leaves:
            decided = l["log"]["esc_b"] == 0
            verdict = l["verdict"]
            if verdict == "bisect":
                fate = d["fate"].get(l["box"])
                verdict = "bisect→split" if fate is None else "bisect→" + fate.split(" ")[0]
            if verdict == "refused":
                verdict = l["detail"].split(" ")[0]
            rec = "same build" if l["rec_eq"] else "another build"
            und = "decided" if decided else "undecided"
            tot[(verdict, rec, und)] += 1
            fin = d["final"].get(l["box"])
            if fin and fin.get("mass") is not None:
                mass[(fin["class"], rec, und)] += fin["mass"]
            if decided:
                groups_choice[l["choice_hash"]].add(l["rec_hash"])
                groups_names[l["names_hash"]].add(l["rec_hash"])
            costs.append((l["t_record_us"], l["t_compare_ns"], l["t_eval_us"], l["rec_bytes"]))
            if l["verdict"] == "flip":
                k = kind_of_first(l["log"])
                if l["evidence_empty"]:
                    k += " [evidence names nothing]"
                flipsplit[k] += 1
                flipsplit_rec[(k, rec)] += 1
                flip_ex.setdefault((k, rec), (d["test"], l))
            if decided and l["rec_eq"] and not l["log"]["strict_equal"]:
                k = kind_of_first(l["log"])
                f = l["log"]["first"] or {}
                sub = (k, f.get("kind"), f.get("a", "").split(" ")[0], f.get("b", "").split(" ")[0])
                eq_rec_diff_log[sub] += 1
                eq_rec_diff_log_ex.setdefault(sub, (d["test"], l))
            if l["rec_eq"] and not l["extras_eq"]:
                a = l["extras_first"]
                k = short([a[1].rsplit(" ", 4)[-4:], a[2].rsplit(" ", 4)[-4:]], 200)
                extras_split[(k, und)] += 1
                extras_ex.setdefault((k, und), (d["test"], l))
        for h, recs in groups_choice.items():
            if len(recs) > 1:
                steer.append((d["test"], h, sorted(recs)))
        for h, recs in groups_names.items():
            if len(recs) > 1:
                names_only.append((d["test"], h, sorted(recs)))

    print("\n== leaves by today's fate x record x decidedness (count)")
    for k, v in sorted(tot.items()):
        print(f"   {v:7d}  {k}")
    print("\n== final mass by today's class x record x decidedness (drives only; sums over all drives)")
    for k, v in sorted(mass.items()):
        print(f"   {v:12.6f}  {k}")
    print("\n== today's FlipCrossing leaves, by what their logs show")
    for k, v in flipsplit.most_common():
        print(f"   {v:7d}  {k}")
    print("   ... crossed with the record:")
    for k, v in sorted(flipsplit_rec.items()):
        t, l = flip_ex[k]
        print(f"   {v:7d}  {k}   e.g. {t}: detail={short(l['detail'], 200)} first={short(l['log']['first'], 300)} "
              f"turned={short(l['log']['turned'], 200)} parted={short(l['log']['parted'], 200)} "
              f"rec_first={short(l.get('rec_first'), 400)} cells={l['cells']} measures={short(l['measures'], 120)}")
    print("\n== decided leaves with the witness's record and a different log (certifying rows), by first difference")
    for k, v in eq_rec_diff_log.most_common(40):
        t, l = eq_rec_diff_log_ex[k]
        print(f"   {v:7d}  {k}   e.g. {t}: verdict={l['verdict']} first={short(l['log']['first'], 300)} "
              f"check_parted={short(l['log']['check_parted'], 160)} cells={l['cells']} extras_eq={l['extras_eq']} measures={short(l['measures'], 160)}")
    print(f"\n== converse hunt: groups of decided leaves (and the witness) with EQUAL choices and DIFFERENT records: {len(steer)}")
    for t, h, recs in steer[:40]:
        print(f"   {t}: choice {h} records {recs}")
    print(f"\n== equal standing+name tables, different record (incidence says more than the names): {len(names_only)}")
    for t, h, recs in names_only[:40]:
        print(f"   {t}: names {h} records {recs}")
    print("\n== leaves with the witness's record whose EXTRAS differ, by first difference")
    for k, v in sorted(extras_split.items(), key=lambda kv: -kv[1])[:40]:
        t, l = extras_ex[k]
        print(f"   {v:7d}  {k}   e.g. {t}: {short(l['extras_first'], 500)} verdict={l['verdict']}")
    if costs:
        tr = sorted(c[0] for c in costs)
        tc = sorted(c[1] for c in costs)
        te = sorted(c[2] for c in costs)
        rb = sorted(c[3] for c in costs)
        n = len(costs)
        print(f"\n== per-leaf cost over {n} leaves: take the record median {tr[n//2]} us (p90 {tr[int(n*.9)]}, max {tr[-1]}); "
              f"compare median {tc[n//2]} ns (p90 {tc[int(n*.9)]}); leaf evaluation median {te[n//2]} us (p90 {te[int(n*.9)]}); "
              f"record size median {rb[n//2]} bytes (max {rb[-1]}); "
              f"total record {sum(tr)/1e6:.1f}s vs total evaluation {sum(te)/1e6:.1f}s = {100*sum(tr)/max(1,sum(te)):.2f}%")


if __name__ == "__main__":
    main()
