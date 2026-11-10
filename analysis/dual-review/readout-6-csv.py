#!/usr/bin/env python3
"""Readout 6: the derived tables behind the readout's dashboard, as CSV.

    python3 analysis/dual-review/readout-6-csv.py [repo-root] [out-dir]

Stdlib only. Every figure comes from readout-6-numbers.py's loading,
corrections and constants (imported, not copied), so the CSVs and the
readout agree. Writes into analysis/dual-review/readout-6-csv/ by default.
"""

import csv
import importlib.util
import os
import re
import statistics as st
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("numbers6", os.path.join(HERE, "readout-6-numbers.py"))
N = importlib.util.module_from_spec(_s)
_s.loader.exec_module(N)

R5 = N.R5
ARMS_START = N.ARMS_START

# Where each traced escape sits, read from the work item's own title or
# text (the readout's section 4 claim that most are rows the per-PR gate
# does not select). "eps rows" = the 1e-6 / 1e-12 / Interval rows.
ESCAPE_WHERE = {
    158: "slow set", 174: "main build (crossed merge)", 176: "slow set", 177: "suite the PR gate did not select",
    200: "feature-gated / nightly job", 237: "eps rows", 259: "suite the PR gate did not select", 264: "eps rows",
    266: "suite the PR gate did not select", 279: "eps rows", 284: "eps rows", 288: "slow set",
    308: "eps rows", 331: "eps rows", 335: "suite the PR gate did not select", 343: "fast set (CI profile)",
    345: "slow set", 348: "eps rows", 360: "slow set", 363: "eps rows", 368: "eps rows", 377: "eps rows",
    382: "slow set", 390: "eps rows", 392: "not stated", 397: "suite the PR gate did not select",
    405: "slow set", 435: "suite the PR gate did not select", 455: "feature-gated / nightly job",
}

DATA_QUALITY = [
    ("Rule 11 under-recorded", "5 later-escape lines; work items attribute a main red or regression to 28 units since readout 5", "see escapes"),
    ("Arm mislabelled", "front matter says concurrent; class cell says HOLDOUT (byte 198)", "DR-410"),
    ("One pair, two rows", "one stacked-unit holdout pair recorded as two holdout rows", "DR-207, DR-208"),
    ("Table shape", "127 rows lack the 11 cells (74 fewer, 53 more from unescaped pipes); DR-150 truncated", "74 + 53 rows"),
    ("Foot tally mixes holdouts", "foot reads 91; concurrent-only tally is 82, against the Seams", "foot"),
    ("major field contradicts cells", "false with a MAJOR on both sides; true with none; true from a delta read", "DR-66, DR-57, DR-376"),
    ("Ruling not recorded", "Ev's PR 4645 ruling on readout 5 missing from rulings.md; DR-144 and DR-104 questions unruled", "rulings.md"),
    ("Protocol hashes", "124 rows cite a non-touching commit; 4 cite the pre-PR-4645 text; one does not resolve", "DR-166, 167, 169, 174, 455"),
    ("Cost fields sparse", "tokens on 23 of 526 reviews since readout 5, minutes on 132, mixed units", "since DR-166"),
    ("Weak coding of tallied pairs", "8 of 38 tallied pairs since readout 5 coded unblinded, by the orchestrator, or unnamed", "DR-166, 171, 208, 254, 304, 308, 353, 411"),
    ("Sequential deviations", "second review on the first's head (2); briefed blind (1)", "DR-246, DR-263, DR-316"),
    ("Draw time missing", "on 67 of 126 arm rows since readout 5", "since DR-166"),
    ("Stale Running: counters", "hand-kept counters left in tally cells", "DR-366, DR-448"),
    ("Model version", "reviewers recorded only as Opus, never a version", "all rows"),
]

OPTIONS = [
    ("Continue unchanged", "2 reviews per H unit, about 1.5 per M unit",
     "M miss rate narrows slowly: halving its interval takes about 150 more holdouts (about four months)",
     "H estimate already stationary; H pairs now buy defects, not information"),
    ("End the H experiment, keep H pairs as policy", "saves a coder lane per MAJOR pair and the row-keeping",
     "nothing more on H", "loses nothing the estimate needs at 258 fair H pairs"),
    ("Raise the holdout share (mod 2)", "about 0.12 more reviews per M unit (1.64 vs 1.52)",
     "holdouts accrue 1.5x as fast", "fewer sequential units and second reviews"),
    ("Make the sequential second review unconditional", "2 reviews per sequential unit, no concurrency",
     "what a fix-head review finds when the first raised no MAJOR (63 such units since readout 5)",
     "the conditional trigger is no longer measured"),
    ("End the M arms: single review", "saves 0.28 reviews per unit vs today's sequential arm",
     "nothing in this log", "loses the second review's 7 new MAJORs in 24 runs"),
    ("End the M arms: all pairs", "0.5 more reviews per unit than today's mix",
     "the unilateral-MAJOR rate at M directly", "buys back about 0.05-0.07 misses per unit"),
    ("End the stream", "none further", "none further",
     "record stands: 281 fair concurrent pairs, 48 holdouts, 113 sequential units"),
    ("Alongside any: record escapes and fix bookkeeping", "a line per traced red; one docs pass and a cell-count lint",
     "escapes, the only test of single-review units", "none to the method"),
    ("Alongside any: spend on CI selection", "run the slow set and eps rows for touched crates",
     "fewer escapes of the class both reviewers miss", "CI time"),
    ("Cross-model pairs (Opus + Fable)", "ends this Opus/Opus stream",
     "the Opus/Opus H baseline (0.217, 0.168-0.272) becomes the reference",
     "muddles the M holdout counterfactual unless the M arms stay Opus/Opus or stop in one recorded step"),
]


def row_rate(k, n):
    lo, hi = N.cp(k, n)
    return {"k": k, "n": n, "rate": round(k / n, 4), "lo": round(lo, 4), "hi": round(hi, 4)}


def holdout_stats(hs):
    noMAJ = partner_tal = misses = 0
    for r in hs:
        a, b = N.MAJ_MANUAL.get(r["dr"], (N.majc(r["r1"]), N.majc(r["r2"])))
        if a + b == 0:
            noMAJ += 2
        elif not (a and b):
            noMAJ += 1
            if r["tallied"]:
                partner_tal += 1
                misses += 1
    return noMAJ, partner_tal, misses


def main(root, out):
    os.makedirs(out, exist_ok=True)
    rows = N.load(root)
    hold = [r for r in rows if r["arm"] == "holdout" and r["dr"] not in N.MERGED_INTO]
    seq = [r for r in rows if r["arm"] == "sequential"]
    conc_h = [r for r in rows if r["arm"] == "concurrent" and r["letter"] == "H" and not r["excluded"]]
    wins = [("before readout 5", lambda d: ARMS_START <= d <= R5), ("since readout 5", lambda d: d > R5),
            ("all arms", lambda d: True)]

    def write(name, fields, recs):
        with open(os.path.join(out, name), "w", newline="", encoding="utf-8") as f:
            w = csv.DictWriter(f, fieldnames=fields)
            w.writeheader()
            w.writerows(recs)

    # headline
    h = row_rate(sum(1 for r in conc_h if r["tallied"]), len(conc_h))
    hs_since = [r for r in hold if r["dr"] > R5]
    _, _, miss_since = holdout_stats(hs_since)
    _, _, miss_all = holdout_stats(hold)
    s_since = [r for r in seq if r["dr"] > R5]
    sec_since = sum(1 for r in s_since if r["major"])
    head = [
        {"id": "h_tally", "label": "H fair pairs with a tallied finding", **h, "value": h["rate"]},
        {"id": "holdout_miss_since", "label": "M-tier misses per holdout pair, since readout 5",
         **row_rate(miss_since, len(hs_since)), "value": round(miss_since / len(hs_since), 4)},
        {"id": "holdout_miss_all", "label": "M-tier misses per holdout pair, all arms",
         **row_rate(miss_all, len(hold)), "value": round(miss_all / len(hold), 4)},
        {"id": "seq_reviews_per_unit", "label": "Reviews per sequential unit, since readout 5",
         "k": len(s_since) + sec_since, "n": len(s_since), "rate": "", "lo": "", "hi": "",
         "value": round((len(s_since) + sec_since) / len(s_since), 3)},
        {"id": "h_findings_per_pair", "label": "Tallied findings per fair H pair",
         "k": sum(r["tallied"] for r in conc_h), "n": len(conc_h), "rate": "", "lo": "", "hi": "",
         "value": round(sum(r["tallied"] for r in conc_h) / len(conc_h), 3)},
    ]
    write("headline.csv", ["id", "label", "value", "k", "n", "rate", "lo", "hi"], head)

    # H blocks (concurrent fair pairs, as the readout's trend table)
    recs = []
    for lo, hi in [(1, 47), (48, 165), (166, 260), (261, 360), (361, 458)]:
        fair = [r for r in rows if lo <= r["dr"] <= hi and r["arm"] == "concurrent" and not r["excluded"]]
        t = sum(1 for r in fair if r["tallied"])
        recs.append({"block": f"DR-{lo}..{hi}", "from": min(r["date"] for r in fair),
                     "to": max(r["date"] for r in fair), "fair": len(fair),
                     "major": sum(r["major"] for r in fair), **row_rate(t, len(fair)),
                     "findings": sum(r["tallied"] for r in fair)})
    write("h_blocks.csv", ["block", "from", "to", "fair", "major", "k", "n", "rate", "lo", "hi", "findings"], recs)

    # M arms rates
    recs = []
    for w, f in wins:
        hs = [r for r in hold if f(r["dr"])]
        ss = [r for r in seq if f(r["dr"])]
        noMAJ, ptal, miss = holdout_stats(hs)
        first = sum(1 for r in ss if r["major"])
        new = sum(1 for d in N.SECOND_NEW_MAJOR if f(d))
        for measure, k, n in [("M-tier misses per holdout pair", miss, len(hs)),
                              ("No-MAJOR holdout review, twin raised a tallied MAJOR", ptal, noMAJ),
                              ("Sequential first review raised a MAJOR", first, len(ss)),
                              ("Sequential second review raised a new MAJOR", new, first)]:
            recs.append({"key": f"{w}|{measure}", "window": w, "measure": measure, **row_rate(k, n)})
    write("m_rates.csv", ["key", "window", "measure", "k", "n", "rate", "lo", "hi"], recs)

    # M cost: reviews per unit
    recs = []
    for w, f in wins:
        hs = [r for r in hold if f(r["dr"])]
        ss = [r for r in seq if f(r["dr"])]
        sec = sum(1 for r in ss if r["major"])
        n = len(hs) + len(ss)
        for arm, units, reviews in [("holdout", len(hs), 2 * len(hs)), ("sequential", len(ss), len(ss) + sec),
                                    ("arms together", n, 2 * len(hs) + len(ss) + sec)]:
            recs.append({"key": f"{w}|{arm}", "window": w, "arm": arm, "units": units, "reviews": reviews,
                         "per_unit": round(reviews / units, 3)})
    write("m_cost.csv", ["key", "window", "arm", "units", "reviews", "per_unit"], recs)

    # escapes by arm, since readout 5
    recs = []
    for arm in ("sequential", "holdout", "concurrent"):
        units = [r for r in rows if r["dr"] > R5 and r["arm"] == arm and r["dr"] not in N.MERGED_INTO]
        hit = [r for r in units if r["dr"] in N.ESCAPE_CANDIDATES]
        rec_lines = sum(len(r["notes"]) for r in units)
        kinds = Counter(N.ESCAPE_CANDIDATES[r["dr"]][0].split(",")[0] for r in hit)
        recs.append({"arm": arm, "recorded_lines": rec_lines, **row_rate(len(hit), len(units)),
                     "pin": kinds.get("pin", 0), "build": kinds.get("build", 0),
                     "behaviour": kinds.get("behaviour", 0)})
    write("escapes_by_arm.csv", ["arm", "recorded_lines", "k", "n", "rate", "lo", "hi", "pin", "build", "behaviour"],
          recs)

    by_dr = {r["dr"]: r for r in rows}
    recs = []
    for d, (kind, note) in sorted(N.ESCAPE_CANDIDATES.items()):
        r = by_dr[d]
        recs.append({"dr": f"DR-{d}", "arm": r["arm"], "letter": r["letter"], "kind": kind,
                     "where": ESCAPE_WHERE[d], "recorded": "yes" if r["notes"] else "no",
                     "since_readout_5": "yes" if d > R5 else "no", "note": note})
    write("escape_list.csv", ["dr", "arm", "letter", "kind", "where", "recorded", "since_readout_5", "note"], recs)
    where = Counter(ESCAPE_WHERE[d] for d in N.ESCAPE_CANDIDATES if d > R5)
    write("escape_where.csv", ["where", "units"],
          [{"where": k, "units": v} for k, v in where.most_common()])

    # cost per review
    recs = []
    for w, f in [("before readout 5", lambda d: ARMS_START <= d <= R5), ("since readout 5", lambda d: d > R5)]:
        for arm in ("concurrent", "holdout", "sequential"):
            tk, mn, n = [], [], 0
            for r in rows:
                if not f(r["dr"]) or r["arm"] != arm:
                    continue
                for c in ("r1", "r2"):
                    cell = r[c]
                    if not cell.strip() or cell.strip().startswith(("—", "-")) or re.match(r"not run", cell.strip()):
                        continue
                    n += 1
                    t, m = N.tokens(cell), N.minutes(cell)
                    if t:
                        tk.append(t)
                    if m and 5 <= m <= 600:
                        mn.append(m)
            recs.append({"key": f"{w}|{arm}", "window": w, "arm": arm, "reviews": n,
                         "tokens_recorded": len(tk), "tokens_median": round(st.median(tk)) if tk else "",
                         "minutes_recorded": len(mn), "minutes_median": round(st.median(mn)) if mn else ""})
    write("cost.csv", ["key", "window", "arm", "reviews", "tokens_recorded", "tokens_median",
                       "minutes_recorded", "minutes_median"], recs)

    write("data_quality.csv", ["problem", "detail", "rows"],
          [{"problem": a, "detail": b, "rows": c} for a, b, c in DATA_QUALITY])
    write("options.csv", ["option", "cost", "measures", "risk"],
          [{"option": a, "cost": b, "measures": c, "risk": d} for a, b, c, d in OPTIONS])


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".",
         sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "readout-6-csv"))
