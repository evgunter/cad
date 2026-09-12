#!/usr/bin/env python3
"""Join a blinded coding pass back to its key and apply the item-3 instrument.

The coder works in A/B (see `blind_reviews.py`) and never learns which reviewer
was which model. This reads the coded findings and the withheld key, restores
the slot and the model behind each raiser, applies protocol v6 item 3
mechanically, and reports the adjudicated unilateral-MAJOR tally against the
pre-registered stopping rule.

The gate, from item 3 — a finding counts iff ALL of:

    (a) UNILATERAL      raised MAJOR by one reviewer, never mentioned by the
                        other at any severity
    (b) DEFECT CLASS    code / test-gap / contract-API; doc- or claim-only
                        findings are recorded but excluded
    (c) DEDUP           findings tracing to one underlying defect count once
    (d) DEMONSTRATED    by execution — a red probe, a mutant gone red, a
                        compile-fail pin, a red CI row, a measured wrong value
    (e) FAIR PAIR       pairs with an interrupted or truncated review are
                        excluded from the tally though still recorded

and the stopping rule from item 2: the stream STOPS at EIGHT adjudicated
unilateral MAJORs or TWELVE new pairs, whichever comes first.

Nothing here rules on the stream. It reports what the instrument says about the
coded data, including the era split the 5.1 instrument note requires, and leaves
the reading to Ev.

`--coder-model` is required because item 4 ends "the readout states the coding
session's own model", and a readout that cannot say which model did the coding
is not the instrument the protocol pre-registered.

Usage:
    python3 unblind_adjudication.py \
        --key keys/blind-key-<stamp>.csv \
        --coded labels/v6-unilateral-adjudication.csv \
        --coder-model <the coding session's model>
    python3 unblind_adjudication.py --selftest
"""
import argparse
import csv
import io
import signal
import sys
from collections import defaultdict

COUNTING_CLASSES = {"code", "test_gap", "contract_api"}
RECORDED_CLASSES = {"doc_or_claim"}
TALLY_STOP = 8
PAIR_STOP = 12
TRUE = {"yes", "y", "true", "1"}
FALSE = {"no", "n", "false", "0"}


def read_csv(path_or_text):
    if isinstance(path_or_text, str) and "\n" in path_or_text:
        return list(csv.DictReader(io.StringIO(path_or_text)))
    with open(path_or_text) as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def flag(value, field, row_id):
    v = (value or "").strip().lower()
    if v in TRUE:
        return True
    if v in FALSE:
        return False
    raise SystemExit("row %s: %s is %r — must be yes or no. Unrecoverable "
                     "counts are missing data, never zeros (item 3e)."
                     % (row_id, field, value))


def adjudicate(key_rows, coded_rows):
    key = {r["row_id"]: r for r in key_rows}
    unknown_rows = sorted(r["row_id"] for r in key_rows if r.get("a_model") == "UNKNOWN")

    stray = sorted({r["row_id"] for r in coded_rows} - set(key))
    if stray:
        raise SystemExit("coded findings name %d row(s) absent from the key: %s\n"
                         "The key is the record of what was blinded; a finding "
                         "outside it was coded against something else."
                         % (len(stray), ", ".join(stray)))

    # (e) pair fairness is a property of the pair; a coder who marks it
    # inconsistently within one pair has to be told, not averaged.
    fair = {}
    for r in coded_rows:
        row_id = r["row_id"]
        value = flag(r.get("fair_pair"), "fair_pair", row_id)
        if row_id in fair and fair[row_id] != value:
            raise SystemExit("row %s: fair_pair coded both ways within one pair."
                             % row_id)
        fair[row_id] = value

    counted, excluded = [], []
    for r in coded_rows:
        row_id = r["row_id"]
        cls = (r.get("defect_class") or "").strip().lower()
        if cls not in COUNTING_CLASSES | RECORDED_CLASSES:
            raise SystemExit("row %s: defect_class %r is not one of %s"
                             % (row_id, cls,
                                sorted(COUNTING_CLASSES | RECORDED_CLASSES)))
        reasons = []
        if not flag(r.get("unilateral"), "unilateral", row_id):
            reasons.append("3a not unilateral")
        if cls not in COUNTING_CLASSES:
            reasons.append("3b %s" % cls)
        if not flag(r.get("demonstrated"), "demonstrated", row_id):
            reasons.append("3d not demonstrated by execution")
        if not fair[row_id]:
            reasons.append("3e unfair pair")
        (excluded if reasons else counted).append((r, reasons))

    # (c) dedup: one underlying defect counts once, however described. A blank
    # dedup_group means the finding is its own group.
    groups = {}
    for r, _ in counted:
        gid = (r["row_id"], (r.get("dedup_group") or "").strip() or r.get("finding_id"))
        raiser = (r.get("raiser") or "").strip().upper()
        if raiser not in ("A", "B"):
            raise SystemExit("row %s: raiser %r — the coder writes A or B."
                             % (r["row_id"], r.get("raiser")))
        if gid in groups and groups[gid]["raiser"] != raiser:
            raise SystemExit("row %s: dedup group %r has findings from both "
                             "reviewers, so they are not one unilateral defect."
                             % (gid[0], gid[1]))
        groups.setdefault(gid, {"raiser": raiser, "members": []})["members"].append(r)

    tally = []
    for (row_id, gid), g in sorted(groups.items()):
        k = key[row_id]
        side = g["raiser"].lower()
        tally.append({
            "row_id": row_id,
            "dedup_group": gid,
            "raiser": g["raiser"],
            "slot": k["%s_slot" % side],
            "model": k["%s_model" % side],
            "date": k["date"],
            "era": k["era"],
            "n_described": len(g["members"]),
            "short_desc": g["members"][0].get("short_desc", ""),
        })

    coded_pairs = set(fair)
    fair_pairs = {p for p, ok in fair.items() if ok}
    uncoded = sorted(set(key) - coded_pairs)
    return {
        "tally": tally,
        "excluded": excluded,
        "key_pairs": len(key),
        "coded_pairs": len(coded_pairs),
        "fair_pairs": len(fair_pairs),
        "uncoded": uncoded,
        "unknown_rows": unknown_rows,
    }


def split(rows, field):
    out = defaultdict(int)
    for r in rows:
        out[r[field]] += 1
    return dict(out)


def report(result, coder_model, out=sys.stdout):
    tally = result["tally"]
    w = out.write
    w("# v6 unilateral-MAJOR adjudication readout\n\n")
    w("Coding session's model (item 4): %s\n" % coder_model)
    w("Pairs in key: %d   coded: %d   fair (3e): %d\n"
      % (result["key_pairs"], result["coded_pairs"], result["fair_pairs"]))
    if result["uncoded"]:
        w("Pairs in the key with no coded findings: %d (%s)\n"
          % (len(result["uncoded"]), ", ".join(result["uncoded"][:12])
             + (" ..." if len(result["uncoded"]) > 12 else "")))
        w("  A pair coded as clean and a pair never coded are different facts;\n"
          "  this counts them as clean. Check the list before reading the tally.\n")
    w("\n## Tally\n\n")
    w("Adjudicated unilateral MAJORs: **%d**\n" % len(tally))
    w("Stopping rule (item 2): %d/%d tally, %d/%d fair pairs — %s\n"
      % (len(tally), TALLY_STOP, result["fair_pairs"], PAIR_STOP,
         "PASSED" if (len(tally) >= TALLY_STOP or result["fair_pairs"] >= PAIR_STOP)
         else "not reached"))
    w("\nBy raising model: %s\n" % (split(tally, "model") or "—"))
    w("By raising slot : %s\n" % (split(tally, "slot") or "—"))
    w("  The slot split is the confound v6 exists to measure: under randomized\n"
      "  assignment it has no reason to depart from even.\n")
    w("\n## By era (5.1 released 2026-09-01; the instrument note requires the split)\n\n")
    for era in ("pre-5.1", "post-5.1"):
        rows = [r for r in tally if r["era"] == era]
        w("- **%s**: %d — by model %s, by slot %s\n"
          % (era, len(rows), split(rows, "model") or "—", split(rows, "slot") or "—"))
    if result["unknown_rows"]:
        w("\n**%d pair(s) carry UNKNOWN models in the key** (%s): their findings "
          "are in the tally and in the slot split, but cannot enter the model "
          "split. Fill the key by hand from the row and re-run.\n"
          % (len(result["unknown_rows"]), ", ".join(result["unknown_rows"])))
    w("\n## Counted findings\n\n")
    if not tally:
        w("none\n")
    for r in sorted(tally, key=lambda r: (r["date"], r["row_id"])):
        w("- %s %-12s %s (%s, %s) — %s%s\n"
          % (r["date"], r["row_id"], r["raiser"], r["slot"], r["model"],
             r["short_desc"],
             "" if r["n_described"] == 1 else "  [%d descriptions deduped]"
             % r["n_described"]))
    w("\n## Recorded but not counted\n\n")
    if not result["excluded"]:
        w("none\n")
    for r, reasons in sorted(result["excluded"], key=lambda x: x[0]["row_id"]):
        w("- %-12s %s — %s\n" % (r["row_id"], r.get("short_desc", ""),
                                 "; ".join(reasons)))


KEY_FIXTURE = """row_id,line_no,date,era,instrument,recorded_byte,recorded_parity,blind_byte,blind_parity,a_slot,b_slot,a_model,b_model,assignment_source
PAIR-1,10,2026-08-28,pre-5.1,v6 instrument,146,0,146,0,R1,R2,opus,fable,parsed
PAIR-2,20,2026-09-03,post-5.1,v6 instrument,99,1,147,1,R2,R1,opus,fable,parsed
PAIR-3,30,2026-09-04,post-5.1,v6 instrument,12,0,200,0,R1,R2,UNKNOWN,UNKNOWN,UNKNOWN
PAIR-4,40,2026-09-05,post-5.1,v6 instrument,55,1,201,1,R2,R1,fable,opus,parsed
"""

CODED_FIXTURE = """row_id,finding_id,raiser,short_desc,unilateral,defect_class,dedup_group,demonstrated,fair_pair,evidence
PAIR-1,f1,A,setback sign survives its mutant,yes,test_gap,,yes,yes,mutant red
PAIR-1,f2,A,the same sign said another way,yes,test_gap,g-sign,yes,yes,same defect
PAIR-1,f3,A,the same sign a third way,yes,code,g-sign,yes,yes,same defect
PAIR-1,f4,B,stale doc sentence,yes,doc_or_claim,,yes,yes,grep
PAIR-2,f1,A,bilateral refusal payload,no,code,,yes,yes,both raised it
PAIR-2,f2,B,accepted by inspection,yes,code,,no,yes,not executed
PAIR-2,f3,A,mutant survived the whole suite,yes,code,,yes,yes,mutant red then green
PAIR-3,f1,B,reachable NaN from a public door,yes,code,,yes,yes,red probe
PAIR-4,f1,A,guard deleted by the rewrite,yes,code,,yes,no,review was killed mid-run
"""


def selftest():
    checks = []

    def check(name, ok):
        checks.append((name, ok))

    result = adjudicate(read_csv(KEY_FIXTURE), read_csv(CODED_FIXTURE))
    tally = result["tally"]
    by_row = defaultdict(list)
    for r in tally:
        by_row[r["row_id"]].append(r)

    check("3c dedup collapses three descriptions to one",
          len([r for r in by_row["PAIR-1"] if r["dedup_group"] == "g-sign"]) == 1)
    check("an ungrouped finding stands alone", len(by_row["PAIR-1"]) == 2)
    check("3b excludes doc-or-claim",
          all("stale doc" not in r["short_desc"] for r in tally))
    check("3a excludes a bilateral finding", "PAIR-2" not in by_row or
          all("bilateral" not in r["short_desc"] for r in by_row["PAIR-2"]))
    check("3d excludes accepted-by-inspection",
          all("inspection" not in r["short_desc"] for r in tally))
    check("3e excludes an interrupted pair", "PAIR-4" not in by_row)
    check("PAIR-4 is not counted as a fair pair", result["fair_pairs"] == 3)

    # Attribution restored through the drawn mapping, both parities.
    p1 = [r for r in by_row["PAIR-1"] if r["dedup_group"] != "g-sign"][0]
    check("parity 0: raiser A resolves to R1", (p1["slot"], p1["model"]) == ("R1", "opus"))
    p2 = by_row["PAIR-2"][0]
    check("parity 1: raiser A resolves to R2 (the mapping swaps)",
          (p2["slot"], p2["model"]) == ("R2", "opus"))
    p3 = by_row["PAIR-3"][0]
    check("parity 0: raiser B resolves to R2", p3["slot"] == "R2")
    check("an UNKNOWN key model survives to the report", p3["model"] == "UNKNOWN")
    check("unknown pairs are named", result["unknown_rows"] == ["PAIR-3"])

    counted_desc = {r["short_desc"] for r in tally}
    check("the tally is exactly the qualifying findings",
          counted_desc == {"setback sign survives its mutant",
                           "the same sign said another way",
                           "mutant survived the whole suite",
                           "reachable NaN from a public door"})
    check("era split is carried", {r["era"] for r in tally} == {"pre-5.1", "post-5.1"})

    # Fail-loud paths: each of these is a coding error that must stop the run.
    def raises(fn):
        try:
            fn()
            return False
        except SystemExit:
            return True

    check("a finding outside the key stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-9,f1,A,x,yes,code,,yes,yes,e\n"))))
    check("a blank fairness flag stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-1,f1,A,x,yes,code,,yes,,e\n"))))
    check("fairness coded both ways in one pair stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-1,f1,A,x,yes,code,,yes,yes,e\n"
            "PAIR-1,f2,B,y,yes,code,,yes,no,e\n"))))
    check("a dedup group spanning both reviewers stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-1,f1,A,x,yes,code,g,yes,yes,e\n"
            "PAIR-1,f2,B,y,yes,code,g,yes,yes,e\n"))))
    check("a slot or model in the raiser column stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-1,f1,R1,x,yes,code,,yes,yes,e\n"))))
    check("an unknown defect class stops the run", raises(
        lambda: adjudicate(read_csv(KEY_FIXTURE), read_csv(
            "row_id,finding_id,raiser,short_desc,unilateral,defect_class,"
            "dedup_group,demonstrated,fair_pair,evidence\n"
            "PAIR-1,f1,A,x,yes,perf,,yes,yes,e\n"))))

    buf = io.StringIO()
    report(result, "test-coder", buf)
    text = buf.getvalue()
    check("the readout states the coding model", "test-coder" in text)
    check("the readout shows the stopping rule", "Stopping rule (item 2)" in text)
    check("the readout reports both eras", "pre-5.1" in text and "post-5.1" in text)

    width = max(len(n) for n, _ in checks)
    failed = 0
    for name, ok in checks:
        print("  %s %s" % ("PASS" if ok else "FAIL", name.ljust(width)))
        failed += 0 if ok else 1
    print("\n%d checks, %d failed" % (len(checks), failed))
    return 1 if failed else 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--key", help="the withheld key written by blind_reviews.py")
    ap.add_argument("--coded", help="the coder's filled adjudication CSV")
    ap.add_argument("--coder-model",
                    help="the coding session's own model — item 4 requires the "
                         "readout to state it")
    ap.add_argument("--out", help="write the readout here instead of stdout")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()

    if args.selftest:
        return selftest()
    missing = [f for f in ("key", "coded", "coder_model") if not getattr(args, f)]
    if missing:
        ap.error("missing required argument(s): %s"
                 % ", ".join("--" + m.replace("_", "-") for m in missing))

    result = adjudicate(read_csv(args.key), read_csv(args.coded))
    if args.out:
        with open(args.out, "w") as f:
            report(result, args.coder_model, f)
        print("wrote %s" % args.out)
    else:
        report(result, args.coder_model)
    return 0


if __name__ == "__main__":
    # These print to stdout and get piped to head; die quietly, not with a
    # BrokenPipeError traceback.
    if hasattr(signal, "SIGPIPE"):
        signal.signal(signal.SIGPIPE, signal.SIG_DFL)

    sys.exit(main())
