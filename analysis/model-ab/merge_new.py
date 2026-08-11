#!/usr/bin/env python3
"""Merge the second-pass label files into the master CSVs.

Idempotent: rebuilds each master from (original rows) + (new rows), so
re-running never duplicates. Normalises the difficulty scale per
DECISIONS.md Q18.
"""
import csv
import os
import shutil

HERE = os.path.dirname(os.path.abspath(__file__))

DIFF_MAP = {"low-M": "M", "M-L": "L", "XL": "L", "infra-class": "NA"}

MERGES = [
    ("core-rows.csv", "labels/core-rows-new.csv"),
    ("labels/cost-parsed.csv", "labels/cost-parsed-new.csv"),
    ("labels/maj-severity.csv", "labels/maj-severity-new.csv"),
    ("labels/task-character.csv", "labels/task-character-new.csv"),
]


def read_rows(path):
    """Return (fieldnames, data rows, comment lines)."""
    comments, lines = [], []
    with open(path) as f:
        for line in f:
            if line.lstrip().startswith("#"):
                comments.append(line.rstrip("\n"))
            else:
                lines.append(line)
    rdr = csv.DictReader(lines)
    return rdr.fieldnames, list(rdr), comments


def main():
    for master, new in MERGES:
        mpath = os.path.join(HERE, master)
        npath = os.path.join(HERE, new)
        if not os.path.exists(npath):
            print("SKIP (no new file): %s" % new)
            continue
        bak = mpath + ".pre-merge"
        if not os.path.exists(bak):
            shutil.copy(mpath, bak)          # keep the first-pass original
        fn_m, rows_m, com_m = read_rows(bak)
        fn_n, rows_n, com_n = read_rows(npath)
        if fn_m != fn_n:
            raise SystemExit("schema mismatch in %s:\n  master=%s\n  new=%s"
                             % (new, fn_m, fn_n))

        key = "row_id"
        # maj-severity has multiple lines per row_id; dedupe on the whole line
        if "maj_index" in fn_m:
            seen = {(r[key], r["maj_index"]) for r in rows_m}
            add = [r for r in rows_n if (r[key], r["maj_index"]) not in seen]
        else:
            seen = {r[key] for r in rows_m}
            add = [r for r in rows_n if r[key] not in seen]

        out = rows_m + add
        if "difficulty" in fn_m:
            for r in out:
                d = r["difficulty"]
                r["difficulty"] = DIFF_MAP.get(d, d)

        with open(mpath, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=fn_m)
            w.writeheader()
            w.writerows(out)
            for c in com_m + com_n:
                f.write(c + "\n")
        print("%-28s %d original + %d new = %d rows"
              % (master, len(rows_m), len(add), len(out)))


if __name__ == "__main__":
    main()
