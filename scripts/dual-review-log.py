#!/usr/bin/env python3
"""The dual-review log's one tool. `docs/DUAL-REVIEW-PROTOCOL.md` is the
protocol; this script holds only its bookkeeping (rule 8 and the foot).

    dual-review-log.py lint       the row files, and the index against its render
    dual-review-log.py render     the index, to docs/DUAL-REVIEW-LOG.md
    dual-review-log.py assign     number every unnumbered row in main's merge order
    dual-review-log.py --selftest

ONE FILE PER ROW, SO NO TWO PRs SHARE A FILE. A row lives at
`docs/dual-review-log/rows/PR-<n>.md`, named by the unit's PR number, which
is unique before the PR merges. It carries no DR number on its branch.
After it merges, the `dual-review-log` workflow runs `assign`, which numbers
it in main's merge order (the first-parent commit that added the file), and
`render`, and commits both from the Actions token, the way `work.py render`
writes `work/STATUS.md`. A PR therefore never touches a file another PR
touches, and never picks a number.

THE INDEX IS A RENDER. `docs/DUAL-REVIEW-LOG.md` is the table, the foot and
the later escapes, generated from the row files. `lint` re-renders it and
fails on any difference, naming the likely cause: a row appended to the
index by hand (a branch from before the split), a `dr:` set by hand, or a
hand edit. Rows without `dr:` are left out of the render, so a branch
adding a row leaves the index exactly as main's.

THE FOOT IS COMPUTED from each row's front matter, never typed. The fields
are exactly the ones the foot reads; everything else is the row's prose:

    dr:        the DR number (absent on a branch; `assign` writes it)
    arm:       concurrent | holdout | sequential (protocol rule 1)
    letter:    H | M | L | S, the difficulty letter logged at spec time
    tallied:   findings that entered the tally (rule 6)
    major:     true when any review of the unit raised a MAJOR
    excluded:  empty for a fair pair; otherwise why the pair is out of the
               tally and the pair count (rules 4 and 6(e))
    miss:      no | holdout | escape, and if an M-tier miss, which of
               rule 9's two kinds

The full readouts that close a counting window are in
`docs/dual-review-log/readouts.md`.

STDLIB ONLY; the front-matter reader is `scripts/work.py`'s, so the two
trackers accept one header grammar.
"""

from __future__ import annotations

import argparse
import importlib.util
import os
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field

_HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("work", os.path.join(_HERE, "work.py"))
assert _spec is not None and _spec.loader is not None
work = importlib.util.module_from_spec(_spec)
sys.modules.setdefault("work", work)
_spec.loader.exec_module(work)
Bail = work.Bail

INDEX = "docs/DUAL-REVIEW-LOG.md"
LOG_DIR = "docs/dual-review-log"
ROWS_DIR = f"{LOG_DIR}/rows"
READOUTS = f"{LOG_DIR}/readouts.md"
RULINGS = f"{LOG_DIR}/rulings.md"
ROW_NAME = re.compile(r"^PR-([1-9]\d*)\.md$")
ARMS = ("concurrent", "holdout", "sequential")
LETTERS = ("H", "M", "L", "S")
MISSES = ("no", "holdout", "escape")
REQUIRED = ("arm", "letter", "tallied", "major", "excluded", "miss")
ORDER = ("dr", *REQUIRED)
OLD_ROW = re.compile(r"^\|\s*(DR-[^|]*?)\s*\|")


@dataclass
class Row:
    name: str               # PR-4618.md
    pr: int
    dr: int | None
    arm: str
    letter: str
    tallied: int
    major: bool
    excluded: str
    miss: str
    line: str               # the table row without its number cell
    notes: list[str] = field(default_factory=list)

    @property
    def pair(self) -> bool:
        return self.arm in ("concurrent", "holdout")

    @property
    def m_tier(self) -> bool:
        return self.arm in ("holdout", "sequential")

    def table_line(self) -> str:
        return f"| DR-{self.dr} {self.line}"


@dataclass
class Readout:
    number: int
    trigger: int            # the DR number of the row that triggered it
    resets: set[str]        # which counts restart after it: units, misses


# --------------------------------------------------------------------------
# loading
# --------------------------------------------------------------------------

def parse_row(raw: str, name: str, where: str) -> Row:
    """One row file. Raises Bail with every problem in the file."""
    m = ROW_NAME.match(name)
    if not m:
        raise Bail(f"{where}: a row file is named PR-<the unit's PR number>.md")
    fields, _order, body = work.parse_front_matter(raw, where)
    errs = []
    unknown = [k for k in fields if k not in ORDER]
    if unknown:
        errs.append(f"unknown field(s) {', '.join(unknown)} (the fields are {', '.join(ORDER)})")
    missing = [k for k in REQUIRED if k not in fields]
    if missing:
        errs.append(f"missing field(s) {', '.join(missing)}")
    dr = fields.get("dr")
    if dr is not None and (not isinstance(dr, int) or isinstance(dr, bool) or dr < 1):
        errs.append(f"`dr` must be a positive integer, not {dr!r}")
    arm, letter, miss = fields.get("arm"), fields.get("letter"), fields.get("miss")
    tallied, major = fields.get("tallied"), fields.get("major")
    excluded = fields.get("excluded")
    if "arm" in fields and arm not in ARMS:
        errs.append(f"`arm` is one of {', '.join(ARMS)}, not {arm!r}")
    if "letter" in fields and letter not in LETTERS:
        errs.append(f"`letter` is one of {', '.join(LETTERS)}, not {letter!r}")
    if "miss" in fields and miss not in MISSES:
        errs.append(f"`miss` is one of {', '.join(MISSES)}, not {miss!r}")
    if "tallied" in fields and (not isinstance(tallied, int) or isinstance(tallied, bool) or tallied < 0):
        errs.append(f"`tallied` is a count, not {tallied!r}")
    if "major" in fields and not isinstance(major, bool):
        errs.append(f"`major` is true or false, not {major!r}")
    if excluded is not None and not isinstance(excluded, str):
        errs.append(f"`excluded` is empty or a reason, not {excluded!r}")
    if not errs:
        if arm == "sequential":
            if tallied:
                errs.append("a sequential unit is not a pair, so nothing of it is tallied (rule 1)")
            if excluded:
                errs.append("a sequential unit is not a pair, so it is never excluded from the pair count; "
                            "leave `excluded` empty")
        if arm == "holdout" and letter not in ("M", "S"):
            errs.append("only an M- or S-letter unit is drawn into the holdout arm (rule 1)")
        if arm == "sequential" and letter not in ("M", "S"):
            errs.append("only an M- or S-letter unit is drawn into the sequential arm (rule 1)")
        if miss == "holdout" and (arm != "holdout" or not tallied):
            errs.append("a holdout miss is a tallied finding in a holdout pair (rule 9)")
        if tallied and not major:
            errs.append("a tallied finding was raised as MAJOR (rule 6(a)), so `major` is true")
    lines = [ln for ln in body.split("\n") if ln.strip()]
    line = lines[0] if lines else ""
    if not line.startswith("| "):
        errs.append("the body opens with the row's table line, `| date | protocol | ... |`")
    elif OLD_ROW.match(line):
        errs.append("the table line starts at the date: the `DR-` cell is rendered from `dr`, never written")
    notes = lines[1:]
    for n in notes:
        if not n.startswith("- "):
            errs.append(f"after the table line, each line is a `- ` note (a later escape, rule 11): {n[:60]!r}")
    if errs:
        raise Bail(f"{where}: " + f"\n{where}: ".join(errs))
    assert isinstance(arm, str) and isinstance(letter, str) and isinstance(miss, str)
    assert isinstance(tallied, int) and isinstance(major, bool)
    return Row(name=name, pr=int(m.group(1)), dr=dr if isinstance(dr, int) else None, arm=arm, letter=letter,
               tallied=tallied, major=major, excluded=excluded or "", miss=miss, line=line, notes=notes)


def load_rows(root: str) -> tuple[list[Row], list[str]]:
    rows, errors = [], []
    d = os.path.join(root, ROWS_DIR)
    if not os.path.isdir(d):
        return [], [f"{ROWS_DIR}/: missing — the dual-review rows live there, one file per row"]
    for name in sorted(os.listdir(d)):
        rel = f"{ROWS_DIR}/{name}"
        try:
            with open(os.path.join(d, name), encoding="utf-8") as f:
                rows.append(parse_row(f.read(), name, rel))
        except Bail as e:
            errors.append(str(e))
    return rows, errors


def load_readouts(root: str) -> list[Readout]:
    with open(os.path.join(root, READOUTS), encoding="utf-8") as f:
        fields, _order, _body = work.parse_front_matter(f.read(), READOUTS)
    first, triggers = fields.get("first"), fields.get("triggers")
    units, misses = fields.get("units_restart"), fields.get("misses_restart")
    ints = lambda v: isinstance(v, list) and all(isinstance(x, int) for x in v)  # noqa: E731
    if not isinstance(first, int) or not ints(triggers) or not ints(units) or not ints(misses):
        raise Bail(f"{READOUTS}: needs `first: <n>` and integer lists `triggers`, `units_restart`, "
                   f"`misses_restart`")
    assert isinstance(triggers, list) and isinstance(units, list) and isinstance(misses, list)
    if triggers != sorted(set(triggers)) or not set(units) | set(misses) <= set(triggers):
        raise Bail(f"{READOUTS}: `triggers` ascend, and every restart is one of them")
    return [Readout(first + i, t, ({"units"} if t in units else set()) | ({"misses"} if t in misses else set()))
            for i, t in enumerate(triggers)]


def numbered(rows: list[Row]) -> list[Row]:
    return sorted((r for r in rows if r.dr is not None), key=lambda r: (r.dr, r.pr))


# --------------------------------------------------------------------------
# the foot
# --------------------------------------------------------------------------

def _label(r: Row, dup: set[int]) -> str:
    return f"DR-{r.dr} (#{r.pr})" if r.dr in dup else f"DR-{r.dr}"


def _list(rs: list[Row], dup: set[int]) -> str:
    return ", ".join(_label(r, dup) for r in rs) if rs else "none"


@dataclass
class Window:
    after: Readout | None          # opens after this readout's trigger
    through: Readout | None        # closes at this readout's trigger (None: open)
    rows: list[Row]


def windows(rows: list[Row], readouts: list[Readout], kind: str) -> list[Window]:
    """Split `rows` at each readout that restarts the `kind` count."""
    cuts = [r for r in readouts if kind in r.resets]
    out, prev = [], None
    for cut in [*cuts, None]:
        lo = prev.trigger if prev else 0
        hi = cut.trigger if cut else None
        out.append(Window(prev, cut, [r for r in rows if r.dr is not None and r.dr > lo
                                      and (hi is None or r.dr <= hi)]))
        prev = cut
    return out


def _span(w: Window) -> str:
    if w.through is not None:
        return f"through readout {w.through.number}'s trigger (DR-{w.through.trigger})"
    if w.after is not None:
        return f"after readout {w.after.number}'s trigger (DR-{w.after.trigger})"
    return "so far"


@dataclass
class Foot:
    tally: int
    fair_major: list[Row]
    excluded: list[Row]
    units: list[Window]
    misses: list[Window]
    duplicates: dict[int, list[Row]]


def compute_foot(rows: list[Row], readouts: list[Readout]) -> Foot:
    rs = numbered(rows)
    fair = [r for r in rs if r.pair and not r.excluded]
    by_dr: dict[int, list[Row]] = {}
    for r in rs:
        assert r.dr is not None
        by_dr.setdefault(r.dr, []).append(r)
    return Foot(
        tally=sum(r.tallied for r in fair),
        fair_major=[r for r in fair if r.major],
        excluded=[r for r in rs if r.pair and r.excluded],
        units=windows([r for r in rs if r.m_tier], readouts, "units"),
        misses=windows([r for r in rs if r.miss != "no"], readouts, "misses"),
        duplicates={k: v for k, v in by_dr.items() if len(v) > 1},
    )


def render_foot(foot: Foot) -> str:
    dup = set(foot.duplicates)
    parts = [f"**Tally: {foot.tally}. Fair pairs that found any MAJOR: {len(foot.fair_major)} "
             f"({_list(foot.fair_major, dup)}).**"]
    for r in foot.excluded:
        parts.append(f"({_label(r, dup)} excluded: {r.excluded}.)")
    unit_bits = []
    for w in foot.units:
        if w.through is None:
            unit_bits.append(f"since the last full readout, {_span(w)}: {len(w.rows)} ({_list(w.rows, dup)})")
        else:
            unit_bits.append(f"{_span(w)}: {len(w.rows)} ({_list(w.rows, dup)})")
    parts.append("**M-tier units under rule 1's arms, " + "; ".join(unit_bits) + "**")
    miss_bits = []
    for w in foot.misses:
        listed = ", ".join(f"{_label(r, dup)} ({r.miss})" for r in w.rows) or "none"
        if w.through is None:
            miss_bits.append(f"since the last full readout, {_span(w)}: {len(w.rows)} ({listed})")
        else:
            miss_bits.append(f"{_span(w)}: {listed}")
    parts.append("**M-tier misses, " + "; ".join(miss_bits) + "**")
    for n, rs in sorted(foot.duplicates.items()):
        parts.append(f"(DR-{n} is held by {len(rs)} rows, "
                     f"{' and '.join(f'#{r.pr}' for r in rs)}: numbered by hand before the rows "
                     f"were split into files, kept as main had them.)")
    return " ".join(parts)


# --------------------------------------------------------------------------
# the render
# --------------------------------------------------------------------------

HEADER = """\
# Dual Opus review — reviewer concordance log

<!-- GENERATED by scripts/dual-review-log.py from docs/dual-review-log/rows/;
never hand-edited. A new row is a new file there (see that script's header). -->

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-23), opened the day the
model A/B protocol was suspended (`docs/MODEL-AB-LOG.md`, the
suspension entry). **The protocol in force lives in
`docs/DUAL-REVIEW-PROTOCOL.md`.** Each row names the protocol it was
recorded under by commit hash, so `git show
<hash>:docs/DUAL-REVIEW-PROTOCOL.md` reads that version. Ev's rulings
on the readouts are in `docs/dual-review-log/rulings.md`.

**This file is generated.** Each row is one file,
`docs/dual-review-log/rows/PR-<n>.md`; its DR number is assigned on main
in merge order, and the foot is computed from the rows' front matter
(`scripts/dual-review-log.py`).

## Rows

Columns: **protocol** (commit hash of the protocol in force at merge — the last
commit on main that touched `docs/DUAL-REVIEW-PROTOCOL.md`); **unit** (program, PR); **class** (difficulty letter and task
class, logged at spec time; the arm, and for an M- or S-letter unit
the draw byte, per protocol rule 1) and the **triage reason**; **head** (the
frozen commit); **R1** and **R2** (verdict, MAJ/MIN/NOTE, one line of
prose per MAJOR, tokens, wall-clock); **correspondence** (bilateral
headline; unilateral findings R1→ and R2→; severity divergence);
**tally** (candidates, with the 6(a)–(e) disposition of each);
**fair** (yes, or the relaxation / interruption that excludes it);
**fix pass** (size, and who executed it).

| # | date | protocol | unit | class · triage reason | head | R1 | R2 | correspondence | tally | fair | fix pass |
|---|------|----------|------|-----------------------|------|----|----|----------------|-------|------|----------|"""


def render(root: str) -> str:
    rows, errors = load_rows(root)
    if errors:
        raise Bail("cannot render over invalid rows:\n" + "\n".join(errors))
    rs = numbered(rows)
    out = [HEADER]
    out.extend(r.table_line() for r in rs)
    out.append("")
    out.append(render_foot(compute_foot(rows, load_readouts(root))))
    escapes = [n for r in rs for n in r.notes]
    if escapes:
        out.append("")
        out.append("## Later escapes")
        out.append("")
        out.extend(escapes)
    return "\n".join(out) + "\n"


# --------------------------------------------------------------------------
# numbering, on main
# --------------------------------------------------------------------------

def merge_order(root: str) -> list[str]:
    """Row file names in the order main's first-parent history added them."""
    r = subprocess.run(["git", "-C", root, "log", "--first-parent", "--diff-merges=first-parent",
                        "--diff-filter=A", "--reverse", "--format=%x00", "--name-only", "HEAD",
                        "--", ROWS_DIR], capture_output=True, check=False)
    if r.returncode != 0:
        raise Bail(f"git log failed: {r.stderr.decode(errors='replace').strip()}")
    seen: list[str] = []
    for commit in r.stdout.decode().split("\0"):
        for p in sorted(p for p in commit.split("\n") if p.startswith(ROWS_DIR + "/")):
            name = p[len(ROWS_DIR) + 1:]
            if name not in seen:
                seen.append(name)
    return seen


def assign(root: str) -> list[tuple[str, int]]:
    rows, errors = load_rows(root)
    if errors:
        raise Bail("cannot number invalid rows:\n" + "\n".join(errors))
    todo = {r.name for r in rows if r.dr is None}
    if not todo:
        return []
    order = merge_order(root)
    lost = sorted(todo - set(order))
    if lost:
        raise Bail(f"no commit on HEAD's first-parent history adds {', '.join(lost)} — "
                   f"`assign` runs on main with full history (fetch-depth: 0)")
    nxt = max((r.dr for r in rows if r.dr is not None), default=0) + 1
    done = []
    for name in (n for n in order if n in todo):
        path = os.path.join(root, ROWS_DIR, name)
        with open(path, encoding="utf-8") as f:
            raw = f.read()
        fields, _order, body = work.parse_front_matter(raw, name)
        fields["dr"] = nxt
        with open(path, "w", encoding="utf-8") as f:
            f.write(work.format_front_matter(fields, list(ORDER)) + body)
        done.append((name, nxt))
        nxt += 1
    return done


# --------------------------------------------------------------------------
# lint
# --------------------------------------------------------------------------

MOVE_HINT = (f"rows no longer go in {INDEX}: each is its own file, "
             f"{ROWS_DIR}/PR-<the unit's PR number>.md — front matter (arm, letter, tallied, major, "
             f"excluded, miss; NO `dr`, main numbers it at merge) then the table line WITHOUT the `| DR-n` "
             f"cell; leave {INDEX} as main has it (a branch from before the split: take main's copy and "
             f"move your row into the file). The foot is computed; do not edit it. "
             f"See scripts/dual-review-log.py's header.")


def lint(root: str) -> list[str]:
    rows, errors = load_rows(root)
    if errors:
        return errors
    for name in ("readouts.md", "rulings.md"):
        if not os.path.isfile(os.path.join(root, LOG_DIR, name)):
            errors.append(f"{LOG_DIR}/{name}: missing")
    if errors:
        return errors
    try:
        expect = render(root)
    except Bail as e:
        return [str(e)]
    path = os.path.join(root, INDEX)
    have = open(path, encoding="utf-8").read() if os.path.isfile(path) else ""
    if have == expect:
        return []
    want_rows = {ln for ln in expect.split("\n") if OLD_ROW.match(ln)}
    have_rows = [ln for ln in have.split("\n") if OLD_ROW.match(ln)]
    stray = [ln for ln in have_rows if ln not in want_rows]
    for ln in stray:
        m = OLD_ROW.match(ln)
        assert m
        errors.append(f"{INDEX}: a row `{m.group(1)}` that no row file renders. {MOVE_HINT}")
    if not stray:
        by_line = {r.table_line(): r for r in rows if r.dr is not None}
        for ln in sorted(want_rows - set(have_rows)):
            r = by_line[ln]
            errors.append(f"{ROWS_DIR}/{r.name}: carries `dr: {r.dr}` but {INDEX} does not list it — "
                          f"a DR number is assigned on main at merge by the dual-review-log workflow, "
                          f"never by hand: delete the `dr:` line")
    if not errors:
        errors.append(f"{INDEX}: differs from its render — it is generated, never hand-edited. "
                      f"Run `python3 scripts/dual-review-log.py render` (or take main's copy); "
                      f"a change belongs in a row file under {ROWS_DIR}/ or in {READOUTS}")
    return errors


# --------------------------------------------------------------------------
# self-test
# --------------------------------------------------------------------------

def _row_text(dr: int | None = None, arm: str = "concurrent", letter: str = "H", tallied: int = 0,
              major: bool = False, excluded: str = "", miss: str = "no", line: str = "| 2026-10-11 | x |",
              notes: tuple[str, ...] = ()) -> str:
    head = ["---"] + ([f"dr: {dr}"] if dr is not None else [])
    head += [f"arm: {arm}", f"letter: {letter}", f"tallied: {tallied}",
             f"major: {'true' if major else 'false'}", f"excluded: {excluded}".rstrip(), f"miss: {miss}", "---"]
    return "\n".join([*head, line, *notes]) + "\n"


def _plant(root: str, rows: dict[str, str],
           readouts: str = "---\nfirst: 4\ntriggers: [3]\nunits_restart: [3]\nmisses_restart: [3]\n---\n") -> None:
    os.makedirs(os.path.join(root, ROWS_DIR), exist_ok=True)
    for name, text in rows.items():
        with open(os.path.join(root, ROWS_DIR, name), "w", encoding="utf-8") as f:
            f.write(text)
    with open(os.path.join(root, READOUTS), "w", encoding="utf-8") as f:
        f.write(readouts)
    with open(os.path.join(root, RULINGS), "w", encoding="utf-8") as f:
        f.write("# rulings\n")


def _git(root: str, *args: str) -> None:
    subprocess.run(["git", "-C", root, *args], check=True, capture_output=True)


# Main's foot as it stood when the rows were split into files (origin/main
# 8585f14be), and how the computed foot differs from it. The differences are
# the main foot's own bookkeeping gaps, reported on the PR that split the log,
# not forced away: a computed foot is the point of the split.
MAIN_TALLY = 40
MAIN_FAIR_MAJOR = [5, 9, 11, 13, 15, 16, 17, 21, 24, 28, 29, 30, 32, 33, 34, 35, 37, 38, 39, 42, 43, 46, 48, 50,
                   52, 54, 56, 57, 58, 59, 62, 67, 70, 72, 74, 75, 76, 77, 78, 86, 88, 89, 90, 93, 95, 96, 97, 98,
                   102, 107, 111, 114, 115, 116, 117, 137, 138, 140, 141, 146, 147, 148, 152, 153, 160, 165]
MAIN_EXCLUDED = [2, 51, 101, 112, 119]
MAIN_UNITS_THROUGH_4 = [48, 53, 60, 65, 104, 105, 108, 110, 118, 122, 125, 128, 130, 131, 133, 134, 136, 137,
                        143, 144]
MAIN_UNITS_AFTER_4 = [145, 147, 149, 150, 151, 156, 157, 158, 162, 163, 165]
MAIN_MISSES = [104, 147, 165]
# The FLUX rows DR-149 (#4554) and DR-150 (#4557) were appended below the
# rulings without a foot edit: both are fair H pairs that found a MAJOR.
DELTA_FAIR_MAJOR = [("DR-149", 4554), ("DR-150", 4557)]
# DR-22's fair cell reads "no — 6(e): R2 truncated"; it found no MAJOR, so
# the main foot never listed it. DR-104 was "read apart from the concurrent
# tally" when recorded and left out of both counts; its row says so.
DELTA_EXCLUDED = [22, 104]
# Sequential M units no main foot listed; nothing in their rows excludes them.
DELTA_UNITS_THROUGH_4 = [123, 129, 132]
DELTA_UNITS_AFTER_4 = [161]


def _fixture_foot() -> Foot:
    """The foot over scripts/fixtures/dual-review-rows-2026-10-11.tsv, the
    front matter of every row as migrated."""
    rows = []
    with open(os.path.join(_HERE, "fixtures", "dual-review-rows-2026-10-11.tsv"), encoding="utf-8") as f:
        for ln in f:
            if ln.startswith("#") or not ln.strip():
                continue
            name, dr, arm, letter, tallied, major, miss, excluded = ln.rstrip("\n").split("\t")
            rows.append(Row(name=name, pr=int(name[3:-3]), dr=int(dr), arm=arm, letter=letter,
                            tallied=int(tallied), major=major == "true", excluded=excluded, miss=miss,
                            line="| x |"))
    readouts = [Readout(3, 104, {"misses"}), Readout(4, 144, {"units", "misses"})]
    return compute_foot(rows, readouts)


def selftest() -> int:
    failures = []

    def check(name: str, ok: bool, detail: object = "") -> None:
        if not ok:
            failures.append(f"{name}: {detail}")

    # 1. Today's foot, reproduced from the migrated front matter.
    foot = _fixture_foot()
    drs = lambda rs: sorted(r.dr for r in rs)  # noqa: E731
    check("tally", foot.tally == MAIN_TALLY, foot.tally)
    want = sorted([*MAIN_FAIR_MAJOR, 149, 150])
    check("fair pairs with a MAJOR", drs(foot.fair_major) == want, drs(foot.fair_major))
    flux = [(f"DR-{r.dr}", r.pr) for r in foot.fair_major if r.dr in (149, 150)]
    check("the FLUX pairs", flux == DELTA_FAIR_MAJOR, flux)
    check("excluded", drs(foot.excluded) == sorted(MAIN_EXCLUDED + DELTA_EXCLUDED), drs(foot.excluded))
    u = foot.units
    check("unit windows", len(u) == 2 and u[0].through is not None and u[0].through.number == 4, u)
    check("units through readout 4", drs(u[0].rows) == sorted(MAIN_UNITS_THROUGH_4 + DELTA_UNITS_THROUGH_4),
          drs(u[0].rows))
    check("units after readout 4", drs(u[1].rows) == sorted(MAIN_UNITS_AFTER_4 + DELTA_UNITS_AFTER_4),
          drs(u[1].rows))
    m = foot.misses
    check("miss windows", [drs(w.rows) for w in m] == [[104], [], [147, 165]], [drs(w.rows) for w in m])
    check("miss kinds", {r.miss for w in m for r in w.rows} == {"holdout"})
    check("duplicates", {k: sorted(r.pr for r in v) for k, v in foot.duplicates.items()}
          == {149: [4554, 4583], 150: [4540, 4557]}, foot.duplicates)
    text = render_foot(foot)
    check("foot text: no threshold", "owed" not in text and "toward" not in text, text)
    check("foot text: open window", "since the last full readout, after readout 4's trigger (DR-144): 12 (" in text
          and "since the last full readout, after readout 4's trigger (DR-144): 2 (DR-147 (holdout), "
              "DR-165 (holdout))" in text, text)
    check("foot text: duplicate labels", "DR-149 (#4583)" in text and "DR-150 (#4540)" in text, text)

    # 2. Row validation.
    def refused(text: str, needle: str, name: str = "PR-1.md") -> None:
        try:
            parse_row(text, name, name)
        except Bail as e:
            check(f"refusal {needle!r}", needle in str(e), e)
        else:
            check(f"refusal {needle!r}", False, "accepted")
    parse_row(_row_text(), "PR-1.md", "ok")
    refused(_row_text(), "named PR-", "row-1.md")
    refused(_row_text().replace("arm: concurrent\n", ""), "missing field(s) arm")
    refused(_row_text(arm="pair"), "`arm` is one of")
    refused(_row_text(letter="X"), "`letter` is one of")
    refused(_row_text(arm="sequential", letter="M", tallied=1, major=True), "nothing of it is tallied")
    refused(_row_text(arm="holdout", letter="H"), "only an M- or S-letter")
    refused(_row_text(miss="holdout"), "a holdout miss")
    refused(_row_text(tallied=1), "`major` is true")
    refused(_row_text(dr=0), "positive integer")
    refused(_row_text(line="| DR-3 | 2026 |"), "rendered from `dr`")
    refused(_row_text(line="no table"), "opens with the row's table line")
    refused(_row_text(notes=("stray",)), "`- ` note")
    refused(_row_text().replace("miss: no", "miss: no\ncolour: red"), "unknown field")

    # 3. Render, lint and assign over a planted repo.
    with tempfile.TemporaryDirectory() as root:
        _git(root, "init", "-q", "-b", "main")
        _git(root, "config", "user.email", "t@example.invalid")
        _git(root, "config", "user.name", "t")
        _plant(root, {"PR-10.md": _row_text(1, tallied=1, major=True),
                      "PR-20.md": _row_text(2, arm="holdout", letter="M", tallied=1, major=True, miss="holdout",
                                            notes=("- **Later escape, DR-2:** one.",)),
                      "PR-30.md": _row_text(3, arm="sequential", letter="M")})
        with open(os.path.join(root, INDEX), "w", encoding="utf-8") as f:
            f.write(render(root))
        check("lint clean", lint(root) == [], lint(root))
        idx = open(os.path.join(root, INDEX), encoding="utf-8").read()
        check("render: rows", "| DR-1 | 2026-10-11 | x |" in idx and "| DR-3 |" in idx, idx)
        check("render: tally", "**Tally: 2. Fair pairs that found any MAJOR: 2 (DR-1, DR-2).**" in idx, idx)
        check("render: escapes", "## Later escapes\n\n- **Later escape, DR-2:** one." in idx, idx)
        check("render: units", "through readout 4's trigger (DR-3): 2 (DR-2, DR-3)" in idx, idx)
        _git(root, "add", "-A")
        _git(root, "commit", "-q", "-m", "base")

        # a branch from before the split appends its row to the index
        with open(os.path.join(root, INDEX), "a", encoding="utf-8") as f:
            f.write("| DR-4 | 2026-10-12 | y |\n")
        e = lint(root)
        check("lint: old-file row", len(e) == 1 and "rows no longer go in" in e[0] and "PR-<the unit" in e[0], e)
        with open(os.path.join(root, INDEX), "w", encoding="utf-8") as f:
            f.write(idx.replace("**Tally: 2.", "**Tally: 3."))
        e = lint(root)
        check("lint: hand-edited foot", len(e) == 1 and "never hand-edited" in e[0], e)
        with open(os.path.join(root, INDEX), "w", encoding="utf-8") as f:
            f.write(idx)

        # a new row on a branch: no dr, the index unchanged, lint clean
        _git(root, "checkout", "-q", "-b", "pr-50")
        _plant(root, {"PR-50.md": _row_text()})
        check("lint: new row, no dr", lint(root) == [], lint(root))
        _plant(root, {"PR-50.md": _row_text(4)})
        e = lint(root)
        check("lint: hand-set dr", len(e) == 1 and "never by hand" in e[0], e)
        _plant(root, {"PR-50.md": _row_text()})
        _git(root, "add", "-A")
        _git(root, "commit", "-q", "-m", "row 50")
        _git(root, "checkout", "-q", "main")
        _git(root, "checkout", "-q", "-b", "pr-40")
        _plant(root, {"PR-40.md": _row_text(arm="sequential", letter="S")})
        _git(root, "add", "-A")
        _git(root, "commit", "-q", "-m", "row 40")
        _git(root, "checkout", "-q", "main")
        # PR 50 merges first, then PR 40: main's merge order, not the PR number
        _git(root, "merge", "-q", "--no-ff", "-m", "Merge 50", "pr-50")
        _git(root, "merge", "-q", "--no-ff", "-m", "Merge 40", "pr-40")
        check("lint: merged, unnumbered", lint(root) == [], lint(root))
        done = assign(root)
        check("assign: merge order", done == [("PR-50.md", 4), ("PR-40.md", 5)], done)
        raw = open(os.path.join(root, ROWS_DIR, "PR-40.md"), encoding="utf-8").read()
        check("assign: dr written first", raw.startswith("---\ndr: 5\narm: sequential\n"), raw)
        check("assign: idempotent", assign(root) == [])
        e = lint(root)
        check("lint: numbered, not rendered", len(e) == 2 and all("never by hand" in x for x in e), e)
        with open(os.path.join(root, INDEX), "w", encoding="utf-8") as f:
            f.write(render(root))
        check("lint: rendered", lint(root) == [], lint(root))

    for f in failures:
        print(f"FAIL {f}")
    print(f"dual-review-log.py --selftest: {'FAIL' if failures else 'ok'} ({len(failures)} failure(s))")
    return 1 if failures else 0


# --------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------

def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(prog="dual-review-log.py", description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--root", help="repo root (default: the enclosing checkout)")
    sub = ap.add_subparsers(dest="cmd")
    sub.add_parser("lint")
    sub.add_parser("render")
    sub.add_parser("assign")
    args = ap.parse_args(argv)
    try:
        if args.selftest:
            return selftest()
        if not args.cmd:
            ap.print_help()
            return 2
        root = args.root or work._repo_root()
        if args.cmd == "lint":
            errors = lint(root)
            for e in errors:
                print(e)
            print(f"dual-review-log.py lint: {'FAIL' if errors else 'ok'} ({len(errors)} problem(s))")
            return 1 if errors else 0
        if args.cmd == "render":
            text = render(root)
            with open(os.path.join(root, INDEX), "w", encoding="utf-8") as f:
                f.write(text)
            print(f"wrote {INDEX}")
            return 0
        if args.cmd == "assign":
            for name, n in assign(root):
                print(f"{ROWS_DIR}/{name}: DR-{n}")
            return 0
        raise AssertionError(args.cmd)
    except Bail as e:
        print(f"dual-review-log.py: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
