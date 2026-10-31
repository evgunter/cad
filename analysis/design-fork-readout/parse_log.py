"""Parse docs/DESIGN-FORK-LOG.md's row table into rows.json.

Cells are split on ' | ' outside backticks (prose contains bare
'StepId|None'-style pipes). Fails loudly if any row does not yield 13
cells.
"""
import json, re, sys

COLS = ["n", "date", "protocol", "fork", "problem", "A", "B", "agree",
        "reconciliation", "to_ev", "ev", "match", "ab"]


def split(line):
    body = line.strip()[2:-1].rstrip()
    cells, cur, tick, i = [], "", False, 0
    while i < len(body):
        ch = body[i]
        if ch == "`":
            tick = not tick
        if not tick and body.startswith(" | ", i):
            cells.append(cur.strip()); cur = ""; i += 3; continue
        cur += ch; i += 1
    cells.append(cur.strip())
    return cells


def main(src, dst):
    lines = open(src).read().split("\n")
    rows = []
    for ln, l in enumerate(lines, 1):
        if not (l.startswith("| ") and l[2].isdigit()):
            continue
        c = split(l)
        if len(c) != 13:  # an unbalanced backtick: try a plain split
            c = [x.strip() for x in
                 re.split(r"(?<=\s)\|(?=\s)", l.strip()[1:-1])]
        if len(c) > 13:  # a stray cell: fold the excess into 'ev', keep
            # the last two (match, A/B) anchored at the end
            c = c[:10] + [" / ".join(c[10:-2])] + c[-2:]
        if len(c) != 13:
            sys.exit(f"line {ln}: {len(c)} cells")
        r = dict(zip(COLS, c)); r["line"] = ln
        rows.append(r)
    # duplicate row numbers come from merge collisions; key by line
    for k, r in enumerate(rows):
        r["key"] = f"L{r['line']}"
    json.dump(rows, open(dst, "w"), indent=1)
    print(len(rows), "rows")


if __name__ == "__main__":
    main(*sys.argv[1:])
