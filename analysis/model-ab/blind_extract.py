#!/usr/bin/env python3
"""Produce a BLINDED extract of the A/B log rows for labelling subagents.

Drops the `arm` column entirely and redacts any residual model name so a
labeller cannot condition its judgment on which model did the work.
"""
import re
import sys

SRC = "docs/MODEL-AB-LOG.md"
OUT = "analysis/model-ab/blinded-rows.md"

COLS = ["id", "date", "task", "difficulty", "arm", "findings", "silent_devs",
        "idiom", "tests", "docs", "fix_pass", "battery", "tokens", "wall"]

NAME_RE = re.compile(r"\b(opus|fable|claude-opus-5|claude-fable-5)\b", re.I)


def split_row(line):
    # markdown table row -> cells (strip leading/trailing pipe)
    body = line.strip()
    if not body.startswith("|"):
        return None
    cells = [c.strip() for c in body.strip("|").split("|")]
    return cells


def main():
    rows = []
    for line in open(SRC):
        cells = split_row(line)
        if cells is None or len(cells) != len(COLS):
            continue
        if cells[0] in ("#", "---") or set(cells[0]) <= set("-: "):
            continue
        rec = dict(zip(COLS, cells))
        rows.append(rec)

    with open(OUT, "w") as f:
        f.write("# Blinded A/B dispatch rows (arm column removed, model names redacted)\n\n")
        f.write("Source: `docs/MODEL-AB-LOG.md`. The `arm` column has been dropped and\n")
        f.write("any residual model name replaced with `[MODEL]`. Do not attempt to\n")
        f.write("infer the arm; label only from the substance of the row.\n\n")
        for rec in rows:
            f.write("---\n\n")
            f.write("## row_id: %s\n\n" % rec["id"])
            for k in COLS:
                if k in ("arm", "id"):
                    continue
                v = NAME_RE.sub("[MODEL]", rec[k])
                f.write("- **%s**: %s\n" % (k, v))
            f.write("\n")
    print("wrote %s rows to %s" % (len(rows), OUT))
    # sanity: no model name survived
    txt = open(OUT).read()
    leaks = NAME_RE.findall(txt)
    print("residual model-name leaks:", len(leaks))
    if leaks:
        sys.exit(1)


if __name__ == "__main__":
    main()
