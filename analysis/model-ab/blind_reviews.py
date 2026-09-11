#!/usr/bin/env python3
"""Produce the BLINDED REVIEW material for a protocol v6 item-4 adjudication pass.

`blind_extract.py` blinds the DISPATCH ROWS (one record per unit, arm column
dropped) for the labelling passes. This blinds the REVIEW PROSE, which is what
the correspondence coding and the item-3 unilateral-MAJOR adjudication read, and
it is a different problem: the coder must not learn which reviewer was which
model, so per protocol v6 item 4

    "the coder sees reviewer A/B with the A/B-to-R1/R2 mapping re-randomized
     per pair and model names removed from the material"

Three things therefore have to happen that dropping a column does not do:

  1. **The slot labels are rewritten.** Every `R1`/`R2` token in the retained
     prose becomes `RA`/`RB` under a mapping drawn fresh for that pair, so the
     coder cannot carry an assignment learned from one pair into the next.
     Identifier spellings are rewritten too (`r1_dual_probes` ->
     `ra_dual_probes`): a probe filename names the slot as loudly as the label.
  2. **The draw record is redacted, not just the model names.** A row reading
     "byte 146 parity 0 => R1 OPUS + R2 FABLE" leaks the assignment twice over,
     and redacting only the names leaves "parity 0", which the protocol text
     decodes (item 1: parity 0 = R1 opus + R2 fable). Byte, parity, block and
     slot records all go.
  3. **The cost and rubric cells are withheld.** Per-reviewer token and
     wall-clock figures are an arm signal — cost is one of the things the
     experiment measures — and item 3 needs the findings, not the price.

The mapping is recorded in a key file under `keys/`, which the coder must not
open. `unblind_adjudication.py` joins the coded findings back against it.

Schema note: the log is not one table, and not every row carries its table's
width. Rows are read positionally off the six columns every table starts with —
id, date, task, difficulty, arm, review — and the tail is ignored, so a row that
is short or long still yields its review prose. Rows whose width does not match
their own table's separator are REPORTED (see --audit-widths and the run's
width-anomaly block) rather than dropped: `blind_extract.py` drops them, which
is how it extracts 257 of the log's 310 body rows with no diagnostic, and
dropping a pair from a blinded pass silently is the same failure one level up.
`work/meta/ab-log-rows-do-not-match-their-tables-declared-width` carries the
census and routes the rows to their owners.

Cells are split on the padded separator " | " rather than a bare pipe, because
cells carry maths (`|Δ|≤π−δ`). That rule holds whether or not those pipes are
escaped, so it survives the escaping pass landing on main; once it has, a split
on unescaped pipes only is exact and this heuristic can go.

The review instrument is not the protocol version. Protocol v5 widened the
REVIEW SCOPE (the brief and rubric); protocol v6 is the DUAL-REVIEW DESIGN, and
its item 1 keeps "the v5 instrument ... unchanged". So "v5 instrument" on a row
says which brief the reviewers carried, not that the pair predates v6, and it
selects nothing here. Selection is by date from the first v6 pair.

Usage:
    python3 blind_reviews.py --src <path to MODEL-AB-LOG.md>   # write material
    python3 blind_reviews.py --audit-widths --src <path>       # schema census
    python3 blind_reviews.py --selftest                        # no I/O
"""
import argparse
import csv
import datetime
import os
import re
import signal
import sys

DEFAULT_SRC = "docs/MODEL-AB-LOG.md"
OUT_DIR = os.path.dirname(os.path.abspath(__file__))

# Protocol v6 first pair (PIERCE, 2026-08-27). Rows before it are v5 or earlier.
DEFAULT_SINCE = "2026-08-27"
# Fable 5.1 released 2026-09-01 (instrument note, Ev in-chat 2026-09-02): any
# readout spanning the mark reports the eras separately.
ERA_BOUNDARY = "2026-09-01"

# The six columns every dispatch table in the log shares.
ID, DATE, TASK, DIFFICULTY, ARM, REVIEW = range(6)
ANCHOR = 6

MODEL_RE = re.compile(
    r"\b(opus|fable|sonnet|haiku|claude-opus-\d+|claude-fable-\d+(?:[.-]\d+)?)\b",
    re.I,
)
# A redacted name followed by its version ("[MODEL] 5.1") still dates the arm.
MODEL_VERSION_RE = re.compile(r"\[MODEL\]\s+\d+(?:\.\d+)*")
DRAW_RES = [
    (re.compile(r"\b(?:slot\s+|parity\s+|draw\s+)?bytes?\s+\d+", re.I), "[DRAW]"),
    (re.compile(r"\bparity\s+[01]\b", re.I), "[PARITY]"),
    (re.compile(r"\bblock\s+[A-Za-z0-9._-]+\s+slot\s+\d+", re.I), "[BLOCK/SLOT]"),
    (re.compile(r"\bslot\s+\d+", re.I), "[SLOT]"),
]
# R1/R2 as a word, and as an identifier fragment (r1_probes, gui1_r1).
SLOT_RE = re.compile(r"(?<![A-Za-z0-9])([Rr])([12])(?![0-9])")
# What must not survive into the material.
LEAK_RES = [
    ("model name", MODEL_RE),
    ("slot label", SLOT_RE),
    ("parity", re.compile(r"\bparity\s+[01]\b", re.I)),
    ("draw byte", re.compile(r"\bbytes?\s+\d+", re.I)),
]
# "byte 146 parity 0 => R1 OPUS + R2 FABLE" / "v6 byte 85 parity 1: R1 fable, R2 opus"
ASSIGN_RE = re.compile(
    r"\bR1\s*(?:=|:)?\s*(opus|fable)\b.{0,24}?\bR2\s*(?:=|:)?\s*(opus|fable)\b",
    re.I | re.S,
)
ASSIGN_REV_RE = re.compile(
    r"\bR2\s*(?:=|:)?\s*(opus|fable)\b.{0,24}?\bR1\s*(?:=|:)?\s*(opus|fable)\b",
    re.I | re.S,
)
BYTE_RE = re.compile(r"\bbytes?\s+(\d+)", re.I)
PARITY_RE = re.compile(r"\bparity\s+([01])\b", re.I)
# The REVIEW instrument (the brief and rubric), which is a different axis from
# the dual-review protocol version. v6 item 1 keeps "the v5 instrument ...
# unchanged", so a row reading "v5 instrument" IS a v6 dual — nine rows say both,
# GUI-4 recording "v5 instrument" and "the GUI program's first v6 TALLY
# CANDIDATE" in one cell. Recorded for reference; it selects nothing.
INSTRUMENT_RE = re.compile(r"\bv([3-9])\s+instrument\b", re.I)
SEPARATOR_RE = re.compile(r"^\|[\s:|-]+\|$")
# Some rows date a unit to a RANGE ("2026-08-27/28", "2026-08-31/09-01") because
# the work spanned midnight. Reading only a bare YYYY-MM-DD drops them.
DATE_CELL_RE = re.compile(r"^(\d{4})-(\d{2})-(\d{2})(?:\s*/\s*(?:(\d{2})-)?(\d{2}))?$")
DUAL_RE = re.compile(r"\bDUAL\b")
SINGLE_RE = re.compile(r"\bSINGLE\b")

CORRESPONDENCE_COLS = [
    "row_id", "finding_id", "raiser", "short_desc", "correspondence",
    "counterpart_severity", "notes",
]
ADJUDICATION_COLS = [
    "row_id", "finding_id", "raiser", "short_desc", "unilateral", "defect_class",
    "dedup_group", "demonstrated", "fair_pair", "evidence",
]
KEY_COLS = [
    "row_id", "line_no", "date", "date_raw", "era", "era_straddled", "recorded_byte",
    "recorded_parity", "blind_byte", "blind_parity", "a_slot", "b_slot",
    "a_model", "b_model", "assignment_source",
]


def split_row(line):
    """Cells of a markdown table row, tolerating unescaped pipes inside prose.

    The log writes math as |delta| <= pi inside cells, so splitting on a bare
    pipe shreds the row; splitting on the padded separator does not, because the
    math pipes carry no surrounding spaces.
    """
    s = line.strip()
    if not s.startswith("|"):
        return None
    cells = [c.strip() for c in s.strip("|").split(" | ")]
    if not cells or not cells[0]:
        return None
    if set(cells[0]) <= set("-: ") or cells[0] in ("#", "---", "id", "row"):
        return None
    return cells


def parse_date_cell(cell):
    """(start, end) for a date cell, or None if it is not a date at all."""
    m = DATE_CELL_RE.match(cell.strip())
    if not m:
        return None
    year, month, day = m.group(1), m.group(2), m.group(3)
    start = "%s-%s-%s" % (year, month, day)
    if not m.group(5):
        return start, start
    return start, "%s-%s-%s" % (year, m.group(4) or month, m.group(5))


def separator_width(line):
    """Columns a separator row declares. Separators carry no prose, so a bare
    pipe split is exact here even though it is not for body rows."""
    return len([c for c in line.strip().strip("|").split("|")])


def is_separator(line):
    s = line.strip()
    return bool(s.startswith("|") and SEPARATOR_RE.match(s) and len(s) > 4)


def parse_rows(path):
    with open(path) as f:
        return parse_lines(f.read().split("\n"))


def parse_lines(lines):
    """Body rows with the width their own table declares.

    A separator row sets the table's width and retroactively marks the line
    above it as that table's header, not a body row.
    """
    rows = []
    width = None
    for line_no, line in enumerate(lines, 1):
        if not line.strip().startswith("|"):
            continue
        if is_separator(line):
            width = separator_width(line)
            if rows and rows[-1][0] == line_no - 1:
                rows.pop()          # the line above a separator is its header
            continue
        cells = split_row(line)
        if cells is None or len(cells) < ANCHOR:
            continue
        if parse_date_cell(cells[DATE]) is None:
            continue
        rows.append((line_no, cells, width))
    return rows


def remap_slots(text, parity):
    """R1/R2 -> RA/RB under the drawn mapping. parity 0: A=R1. parity 1: A=R2."""
    def sub(m):
        prefix, slot = m.group(1), int(m.group(2))
        if parity == 0:
            letter = "A" if slot == 1 else "B"
        else:
            letter = "B" if slot == 1 else "A"
        return prefix + (letter if prefix.isupper() else letter.lower())
    return SLOT_RE.sub(sub, text)


def redact(text):
    text = MODEL_RE.sub("[MODEL]", text)
    text = MODEL_VERSION_RE.sub("[MODEL]", text)
    for pattern, replacement in DRAW_RES:
        text = pattern.sub(replacement, text)
    return text


def blind(text, parity):
    """Slot remap first, then redaction: the draw sentence names both at once."""
    return redact(remap_slots(text, parity))


def scan_leaks(text):
    found = []
    for label, pattern in LEAK_RES:
        for m in pattern.finditer(text):
            found.append((label, m.group(0)))
    return found


def parse_assignment(review):
    m = ASSIGN_RE.search(review)
    if m:
        return m.group(1).lower(), m.group(2).lower(), "parsed"
    m = ASSIGN_REV_RE.search(review)
    if m:
        return m.group(2).lower(), m.group(1).lower(), "parsed-reversed"
    return "UNKNOWN", "UNKNOWN", "UNKNOWN"


def era_of(date):
    return "pre-5.1" if date < ERA_BOUNDARY else "post-5.1"


def draw_byte():
    return os.urandom(1)[0]


def select(rows, since, until):
    duals, ambiguous = [], []
    for line_no, cells, width in rows:
        date = parse_date_cell(cells[DATE])[0]
        if date < since or (until and date > until):
            continue
        review = cells[REVIEW]
        if DUAL_RE.search(review):
            duals.append((line_no, cells, width))
        elif SLOT_RE.search(review) and not SINGLE_RE.search(review):
            ambiguous.append((line_no, cells, width))
    return duals, ambiguous


def build(rows, seed_bytes=None):
    """Blind each pair. seed_bytes makes the draw deterministic for the selftest."""
    out = []
    for i, (line_no, cells, width) in enumerate(rows):
        byte = seed_bytes[i] if seed_bytes else draw_byte()
        parity = byte % 2
        start, end = parse_date_cell(cells[DATE])
        r1_model, r2_model, source = parse_assignment(cells[REVIEW])
        a_model, b_model = (r1_model, r2_model) if parity == 0 else (r2_model, r1_model)
        out.append({
            "row_id": cells[ID],
            "line_no": line_no,
            "width": len(cells),
            "table_width": width,
            "date": start,
            "date_raw": cells[DATE],
            "era": era_of(start),
            "era_straddled": era_of(start) != era_of(end),
            "review_instrument": (INSTRUMENT_RE.search(cells[REVIEW]).group(0)
                           if INSTRUMENT_RE.search(cells[REVIEW]) else ""),
            "recorded_byte": (BYTE_RE.search(cells[REVIEW]).group(1)
                              if BYTE_RE.search(cells[REVIEW]) else ""),
            "recorded_parity": (PARITY_RE.search(cells[REVIEW]).group(1)
                                if PARITY_RE.search(cells[REVIEW]) else ""),
            "blind_byte": byte,
            "blind_parity": parity,
            "a_slot": "R1" if parity == 0 else "R2",
            "b_slot": "R2" if parity == 0 else "R1",
            "a_model": a_model,
            "b_model": b_model,
            "assignment_source": source,
            "task_blinded": blind(cells[TASK], parity),
            "review_blinded": blind(cells[REVIEW], parity),
        })
    return out


MATERIAL_HEADER = """# Blinded dual-review material (protocol v6 item 4)

Generated by `blind_reviews.py`. Each pair's two reviewers appear as **RA** and
**RB** under a mapping drawn independently for that pair, so RA in one pair says
nothing about RA in the next. Model names, draw bytes, parity records and block
and slot records are redacted; the per-reviewer cost and rubric cells are not
reproduced at all.

**You are the coder. Do not open `docs/MODEL-AB-LOG.md`, and do not open
`analysis/model-ab/keys/`.** Either one dissolves the blind, and the fourth
readout's lesson — coding and adjudication ran unblinded, in a session of one of
the two models under comparison — is the reason item 4 exists. Record your own
model in the readout.

Code each finding against the item-3 instrument, which is normative and lives in
`docs/MODEL-AB-LOG.md`'s protocol v6 entry. A finding enters the tally iff ALL
of: (a) UNILATERAL — raised as MAJOR by one reviewer and never mentioned by the
other at any severity; (b) DEFECT CLASS code, test-gap or contract-API — doc- or
claim-only findings are recorded but excluded; (c) DEDUP — findings tracing to
one underlying defect count once; (d) DEMONSTRATED BY EXECUTION — a red probe, a
surviving mutant gone red, a compile-fail pin, a red CI row, or a measured wrong
value; (e) FAIR PAIR — a pair where either review was interrupted or truncated
is excluded from the tally though still recorded, and unrecoverable counts are
missing data, never zeros.

Write findings into the blank forms beside this file. `raiser` takes `A` or `B`,
never a slot or a model.

"""


def write_material(pairs, path):
    with open(path, "w") as f:
        f.write(MATERIAL_HEADER)
        f.write("Pairs: %d.\n\n" % len(pairs))
        for p in pairs:
            f.write("---\n\n")
            f.write("## row_id: %s\n\n" % p["row_id"])
            f.write("- **date**: %s\n" % p["date"])
            f.write("- **unit**: %s\n\n" % p["task_blinded"])
            f.write("### reviews\n\n%s\n\n" % p["review_blinded"])


def write_key(pairs, path):
    with open(path, "w", newline="") as f:
        f.write("# WITHHELD FROM THE CODER. The A/B-to-R1/R2 mapping and the\n")
        f.write("# reviewer models for each pair. Opening this during a coding\n")
        f.write("# pass voids the blind; unblind_adjudication.py reads it after.\n")
        w = csv.DictWriter(f, fieldnames=KEY_COLS, extrasaction="ignore")
        w.writeheader()
        for p in pairs:
            w.writerow(p)


def write_forms(directory, pairs):
    written = []
    for name, cols in (("v6-correspondence-BLANK.csv", CORRESPONDENCE_COLS),
                       ("v6-unilateral-adjudication-BLANK.csv", ADJUDICATION_COLS)):
        path = os.path.join(directory, name)
        with open(path, "w", newline="") as f:
            csv.writer(f).writerow(cols)
        written.append(path)
    return written


def audit_widths(path):
    """Census of the log's tables, and of the rows that do not carry their width."""
    from collections import Counter, defaultdict
    with open(path) as f:
        lines = f.read().split("\n")
    widths = Counter()
    for line in lines:
        if is_separator(line):
            widths[separator_width(line)] += 1
    rows = parse_rows(path)
    bad = defaultdict(list)
    for line_no, cells, width in rows:
        if width is not None and len(cells) != width:
            bad[(width, len(cells))].append((line_no, cells[ID]))
    print("tables declared      : %d  (widths %s)"
          % (sum(widths.values()), ", ".join("%d x%d" % (w, n)
                                             for w, n in sorted(widths.items()))))
    print("body rows            : %d" % len(rows))
    print("rows off their width : %d" % sum(len(v) for v in bad.values()))
    for (want, got), entries in sorted(bad.items()):
        print("  table %d, row %2d (%d): %s"
              % (want, got, len(entries), ", ".join(e[1] for e in entries)))
    print("\nA row off its width still yields its review prose — the first six\n"
          "columns are common to every table — but its later columns are shifted\n"
          "or absent. work/meta/ab-log-rows-do-not-match-their-tables-declared-width\n"
          "routes them to their owners.")


def selftest():
    """Red/green on every claim this script makes about its own blinding."""
    fixture = (
        "| FIX-1 | 2026-08-28 | a unit whose prose carries |delta| <= pi inside a cell "
        "| M (pre-draw) | OPUS (block VERBS-5 slot 3) "
        "| **DUAL (ordinal 85; byte 146 parity 0 => R1 OPUS + R2 FABLE, frozen abc123; "
        "v6 instrument)**. R1 found the setback sign; R2 adopted r1_dual_probes and "
        "review_gui1_r2 disagreed. | 0 | R1 4 / R2 3 | 4 | 4 | substantial "
        "| CI GREEN | R1 ~331k / R2 ~258k | R1 ~36m / R2 ~86m |\n"
    )
    checks = []

    def check(name, ok):
        checks.append((name, ok))

    cells = split_row(fixture)
    check("padded split survives an unescaped math pipe", cells is not None and len(cells) >= ANCHOR)
    check("the pipe-bearing cell stays whole", "|delta| <= pi" in cells[TASK])
    bare = [c.strip() for c in fixture.strip().strip("|").split("|")]
    check("a 14-column reader would have dropped this row", len(bare) != 14)

    r1, r2, source = parse_assignment(cells[REVIEW])
    check("assignment parsed from the row", (r1, r2, source) == ("opus", "fable", "parsed"))

    # parity 0: A is R1. parity 1: A is R2.
    for parity, expect in ((0, "RA found the setback"), (1, "RB found the setback")):
        out = blind(cells[REVIEW], parity)
        check("parity %d maps the slot label" % parity, expect in out)
    out0 = blind(cells[REVIEW], 0)
    check("identifier spellings remap too", "ra_dual_probes" in out0)
    check("trailing identifier remaps too", "review_gui1_rb" in out0)
    check("model names go", "OPUS" not in out0 and "FABLE" not in out0)
    check("the draw byte goes", "146" not in out0)
    check("parity goes", "parity 0" not in out0.lower())
    check("block and slot records go", "VERBS-5" not in out0)
    check("no leak survives the full pass", scan_leaks(out0) == [])

    # The leak scan has teeth: redaction removed, it must fire.
    check("leak scan fires on unredacted prose", scan_leaks(cells[REVIEW]) != [])
    check("leak scan fires on names alone", scan_leaks("reviewed by fable") != [])
    check("leak scan fires on a bare slot label", scan_leaks("R2 disagreed") != [])

    # Round trip: a finding the coder attributes to A resolves to the right model.
    # Table tracking: the header above a separator is not a row, and a row that
    # is not its table's width is kept and flagged rather than dropped.
    table = [
        "| # | date | task | difficulty | arm | review | tail |",
        "|---|---|---|---|---|---|---|",
        fixture.rstrip("\n"),
        "| SHORT-1 | 2026-08-29 | a row that stops early "
        "| M (pre-draw) | OPUS (slot 1) | **DUAL** R1 vs R2 |",
    ]
    parsed = parse_lines(table)
    check("the header above a separator is not a body row",
          [r[1][ID] for r in parsed] == ["FIX-1", "SHORT-1"])
    check("each row carries its table's declared width",
          all(r[2] == 7 for r in parsed))
    check("a row off its table's width is kept, not dropped",
          any(r[1][ID] == "SHORT-1" and len(r[1]) != r[2] for r in parsed))
    short = build([r for r in parsed if r[1][ID] == "SHORT-1"], seed_bytes=[2])[0]
    check("the anomaly is visible on the built pair",
          (short["width"], short["table_width"]) == (6, 7))

    pairs = build([(1, cells, 14)], seed_bytes=[146])   # 146 % 2 == 0 -> A = R1
    p = pairs[0]
    check("key records the mapping", (p["a_slot"], p["a_model"]) == ("R1", "opus"))
    check("key records the counterpart", (p["b_slot"], p["b_model"]) == ("R2", "fable"))
    pairs_odd = build([(1, cells, 14)], seed_bytes=[147])
    q = pairs_odd[0]
    check("odd draw swaps the mapping", (q["a_slot"], q["a_model"]) == ("R2", "fable"))
    check("era split lands on the boundary",
          era_of("2026-08-31") == "pre-5.1" and era_of("2026-09-01") == "post-5.1")
    check("a plain date parses", parse_date_cell("2026-08-27") == ("2026-08-27", "2026-08-27"))
    check("a same-month range parses", parse_date_cell("2026-08-27/28")
          == ("2026-08-27", "2026-08-28"))
    check("a month-crossing range parses", parse_date_cell("2026-08-31/09-01")
          == ("2026-08-31", "2026-09-01"))
    check("a non-date is rejected", parse_date_cell("M (pre-draw)") is None)
    ranged = list(cells)
    ranged[DATE] = "2026-08-31/09-01"
    r = build([(3, ranged, 14)], seed_bytes=[10])[0]
    check("a ranged row is kept, not dropped", r["date"] == "2026-08-31")
    check("a range crossing 5.1 is flagged", r["era_straddled"] is True)
    check("review instrument recorded", p["review_instrument"].lower() == "v6 instrument")
    v5row = list(cells)
    v5row[REVIEW] = cells[REVIEW].replace("v6 instrument", "v5 instrument")
    v5 = build([(4, v5row, 14)], seed_bytes=[8])[0]
    check("a v5-instrument row is still selected, not set aside",
          v5["review_instrument"].lower() == "v5 instrument" and v5["row_id"] == "FIX-1")
    check("recorded draw kept in the key, not the material", p["recorded_byte"] == "146")

    # An unparseable assignment is reported, never guessed.
    blank = list(cells)
    blank[REVIEW] = "**DUAL (ordinal 900)**. R1 approved; R2 approved."
    unknown = build([(2, blank, 14)])[0]
    check("unparseable assignment is UNKNOWN, not a guess",
          unknown["a_model"] == "UNKNOWN" and unknown["assignment_source"] == "UNKNOWN")

    width = max(len(n) for n, _ in checks)
    failed = 0
    for name, ok in checks:
        print("  %s %s" % ("PASS" if ok else "FAIL", name.ljust(width)))
        failed += 0 if ok else 1
    print("\n%d checks, %d failed" % (len(checks), failed))
    return 1 if failed else 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--src", default=DEFAULT_SRC,
                    help="path to MODEL-AB-LOG.md (default: the in-tree copy)")
    ap.add_argument("--since", default=DEFAULT_SINCE, help="earliest row date (inclusive)")
    ap.add_argument("--until", default=None, help="latest row date (inclusive)")
    ap.add_argument("--out-dir", default=OUT_DIR)
    ap.add_argument("--audit-widths", action="store_true",
                    help="census the log's table schemas and exit")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()

    if args.selftest:
        return selftest()
    if args.audit_widths:
        audit_widths(args.src)
        return 0

    rows = parse_rows(args.src)
    duals, ambiguous = select(rows, args.since, args.until)
    if not duals:
        print("no dual rows in [%s, %s]" % (args.since, args.until or "today"))
        return 1
    pairs = build(duals)

    keys_dir = os.path.join(args.out_dir, "keys")
    labels_dir = os.path.join(args.out_dir, "labels")
    os.makedirs(keys_dir, exist_ok=True)
    os.makedirs(labels_dir, exist_ok=True)
    stamp = datetime.datetime.now().strftime("%Y%m%dT%H%M%S")
    material = os.path.join(args.out_dir, "blinded-reviews.md")
    key = os.path.join(keys_dir, "blind-key-%s.csv" % stamp)

    write_material(pairs, material)
    write_key(pairs, key)
    forms = write_forms(labels_dir, pairs)

    print("pairs blinded : %d  (%s .. %s)" % (
        len(pairs), pairs[0]["date"], pairs[-1]["date"]))
    eras = {}
    for p in pairs:
        eras[p["era"]] = eras.get(p["era"], 0) + 1
    print("era split     : %s" % ", ".join("%s %d" % kv for kv in sorted(eras.items())))
    print("material      : %s" % material)
    print("key (WITHHELD): %s" % key)
    for f in forms:
        print("blank form    : %s" % f)

    unknown = [p["row_id"] for p in pairs if p["a_model"] == "UNKNOWN"]
    if unknown:
        print("\nMANUAL ENTRY REQUIRED — the reviewer assignment did not parse for "
              "%d pair(s):" % len(unknown))
        for row_id in unknown:
            print("  %s" % row_id)
        print("Fill a_model/b_model in the key by hand from the row, then re-run "
              "unblind_adjudication.py.")

    straddled = [p for p in pairs if p["era_straddled"]]
    if straddled:
        print("\nDATED ACROSS THE 5.1 BOUNDARY — the era split cannot place these "
              "(%d):" % len(straddled))
        for p in straddled:
            print("  %-12s %s" % (p["row_id"], p["date_raw"]))
        print("They are filed under their start date; the instrument note says a "
              "readout spanning 2026-09-01 reports the eras separately, so say "
              "which side these were worked on before reading one.")

    anomalous = [p for p in pairs
                 if p["table_width"] is not None and p["width"] != p["table_width"]]
    if anomalous:
        print("\nWIDTH ANOMALY — selected pairs whose row is not its table's width "
              "(%d):" % len(anomalous))
        for p in anomalous:
            print("  %-12s row has %d cells, table declares %d"
                  % (p["row_id"], p["width"], p["table_width"]))
        print("Their review prose is blinded as usual (the first six columns are "
              "common to every table), but anything read from a later column of "
              "these rows is shifted or missing.")

    if ambiguous:
        print("\nAMBIGUOUS — reviewer labels but no DUAL marker, not selected (%d):"
              % len(ambiguous))
        for line_no, cells, _ in ambiguous:
            print("  line %-5d %-14s %s" % (line_no, cells[ID], cells[DATE]))

    text = open(material).read()
    leaks = scan_leaks(text)
    print("\nresidual leaks in the material: %d" % len(leaks))
    if leaks:
        seen = {}
        for label, hit in leaks:
            seen.setdefault(label, []).append(hit)
        for label, hits in seen.items():
            print("  %s: %d (%s)" % (label, len(hits), ", ".join(sorted(set(hits))[:6])))
        print("MATERIAL IS NOT BLIND — do not hand it to a coder.")
        return 1
    return 3 if unknown else 0


if __name__ == "__main__":
    # These print to stdout and get piped to head; die quietly, not with a
    # BrokenPipeError traceback.
    if hasattr(signal, "SIGPIPE"):
        signal.signal(signal.SIGPIPE, signal.SIG_DFL)

    sys.exit(main())
