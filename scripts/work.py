#!/usr/bin/env python3
"""The tracker's one tool. `work/README.md` is the contract this script
enforces; this header says only what the contract does not.

    work.py lint                         every rule in work/README.md
    work.py status [--program P]         the render, to stdout
    work.py render [--out PATH]          the render, to work/STATUS.md
    work.py new ID --kind K --title T [--program P] [--set key=value ...]
    work.py set ID key=value [key=value ...]
    work.py territory (--base REF | --files LIST) [--branch NAME] [--strict]
    work.py incoming [--base REF] [--program P]   what the base carries into work/P/ that HEAD lacks
    work.py --selftest

STDLIB ONLY, AND ITS OWN FRONT-MATTER READER. The header format is a
subset of YAML small enough to state in one sentence (`key: scalar` and
`key: [a, b]`, nothing nested), so a parser that accepts exactly that
subset is a lint rule, not a limitation: anything a real YAML library
would silently accept beyond it — anchors, block lists, multi-line
strings — is refused here by name. The runner image is not asked for a
library, matching every other cheap tripwire under scripts/.

WHAT LINT CANNOT SEE, stated because a disclosed blind spot is a work
order. Ints in `blocked_on`, `refs` and `pr` are PR or issue numbers
on GitHub and are not resolved AGAINST GITHUB — the tracker is version
control and does not call out. One resolution is in the tree and is
made: a migrated issue carries its number on the item that replaced it
(`github:`), so a number in `blocked_on` that matches exactly one such
item reaches the fired-trigger rule through it, as a warning rather
than an error because the match is an inference (`work/README.md`).
A number matching none, or two, stays unchecked. A `[ev]` PR title is
likewise a GitHub fact; lint checks that `needs_ev` is a bare `true`,
not that an open PR carries the flag (the PR is one `git log` away from
the item). Territory globs are matched with
`fnmatch`, where `*` crosses `/` — `crates/mesh/*` is the whole crate,
which is what every territory in the tree means by it.
"""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import os
import re
import subprocess
import sys
import tempfile
from collections import defaultdict
from dataclasses import dataclass, field

WORK = "work"
STATUS_FILE = "work/STATUS.md"
ISSUES_DIR = "issues"
FREE_FILES = {"README.md", "STATUS.md"}       # unparsed: top of work/, and
                                              # README.md inside work/issues/
NARRATIVE = {"plan.md", "log.md", "process-observations.md"}   # inside a program, unparsed
LOG_EXEMPT = {"docs/MODEL-AB-LOG.md",        # the non-program logs in docs/
              "docs/DUAL-REVIEW-LOG.md",
              "docs/DESIGN-FORK-LOG.md"}
STALE_DAYS = 14

KINDS = ("program", "unit", "issue", "ruling")
ITEM_STATUS = ("open", "spec", "dispatched", "review", "closed", "parked", "deferred")
# The two not-now statuses, ordered last above so a slate lists them furthest
# from dispatchable: `parked` waits on a named trigger, `deferred` is a
# ratified not-now whose reason is prose in the body (see work/README.md).
RULING_STATUS = ("open", "closed")
# A PROGRAM's status is about the track, not about a row. Two facts tell the
# three apart: whether an orchestrator holds the track, and whether anything on
# it can be picked up (`DISPATCHABLE`, below). Only the second is in the tree,
# so it is the only one lint checks; `active` is the program's own word.
# `blocked` never has an orchestrator BY CONSTRUCTION: an orchestrator that has
# run out of non-blocked units cuts the blocked rows into a new program and
# closes what it finishes, rather than holding a whole track hostage to its
# slowest blocker (Ev, in chat, 2026-09-21; work/README.md).
# There is no `closed`: a program that closes is DELETED in the sweep that
# closes it, so a closed program is an ABSENT one and no status has to say so.
PROGRAM_STATUS = ("ready", "active", "blocked")
AREAS = ("kernel", "api", "gui", "infra")

# The priority bands (work/README.md, "Priority"). P0 is most urgent. A band
# says WHAT to do, never WHEN: dispatch order is priority together with cost
# and with whether design work is open, which is the orchestrator's judgement
# and is deliberately not a field.
PRIORITIES = ("P0", "P1", "P2", "P3", "P4")

# The cost class, as the 2026-09-03 cut defined it (docs/WORK-TRACKS-2026-09.md):
# E the fix is written in the item, D a design question is open, H the intent is
# clear and getting it right is technically hard. The weights are what make one
# budget say "about 6 hard rows or about 30 easy ones" in a single number.
COSTS = ("E", "D", "H")
COST_WEIGHT = {"E": 1.0, "D": 2.5, "H": 5.0}
DEFAULT_BUDGET = 30            # points; 30 E, 12 D, or 6 H
UNPRICED_WEIGHT = COST_WEIGHT["D"]   # an unscored row is priced mid-range

# A row counts against its track's budget only while it is DISPATCHABLE. A row
# in flight, parked, deferred or closed is not a claim on the next sitting's
# attention, which is what the budget measures.
DISPATCHABLE = ("open", "spec")

# key -> (type, kinds that may carry it). Types: str, int, date, ref,
# reflist (ids or ints), strlist, enum:<name>.
SCHEMA: dict[str, tuple[str, tuple[str, ...]]] = {
    "id": ("str", KINDS),
    "kind": ("enum:kind", KINDS),
    "title": ("str", KINDS),
    "status": ("enum:status", KINDS),
    "opened": ("date", KINDS),
    "closed": ("date", KINDS),
    "refs": ("reflist", KINDS),
    "parent": ("ref", ("unit", "issue", "ruling")),
    "blocked_on": ("reflist", ("unit", "issue", "ruling")),
    "rides_with": ("ref", ("unit", "issue", "ruling")),
    "pr": ("int", ("unit", "issue", "ruling")),
    "branch": ("str", ("unit", "issue", "ruling")),
    "needs_ev": ("flag", ("unit", "issue", "ruling", "program")),
    "track": ("str", ("unit", "issue", "ruling")),
    "priority": ("enum:priority", KINDS),
    "cost": ("enum:cost", ("unit", "issue", "ruling")),
    "github": ("int", ("unit", "issue", "ruling")),
    "area": ("enum:area", ("program",)),
    "prefix": ("str", ("program",)),
    "tag": ("str", ("program",)),
    "ab_band": ("str", ("program",)),
    "paths": ("strlist", ("program",)),
    "keep_out": ("strlist", ("program",)),
    "blocks": ("strlist", ("program",)),
    "budget": ("int", ("program",)),
}
REQUIRED = ("id", "kind", "title", "status", "opened")
LIST_TYPES = ("reflist", "strlist")   # the fields `set` accepts a bare scalar for

ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
INT_RE = re.compile(r"^-?\d+$")
KEY_RE = re.compile(r"^([a-z_]+):(?:\s+(.*))?$")


class Bail(Exception):
    """Input this tool refuses. Never a pass."""


@dataclass
class Item:
    path: str                       # repo-relative
    program: str | None             # directory name, None under issues/
    fields: dict[str, object]
    body: str
    order: list[str] = field(default_factory=list)

    @property
    def id(self) -> str:
        return str(self.fields.get("id", ""))

    @property
    def kind(self) -> str:
        return str(self.fields.get("kind", ""))

    @property
    def status(self) -> str:
        return str(self.fields.get("status", ""))

    def get(self, key: str, default: object = None) -> object:
        return self.fields.get(key, default)


# --------------------------------------------------------------------------
# front matter
# --------------------------------------------------------------------------

def _scalar(text: str) -> object:
    text = text.strip()
    if text == "":
        return None
    if len(text) >= 2 and text[0] == text[-1] and text[0] in "'\"":
        return text[1:-1]
    if INT_RE.match(text):
        return int(text)
    if text in ("true", "false"):
        return text == "true"
    return text


def parse_front_matter(raw: str, where: str) -> tuple[dict[str, object], list[str], str]:
    """(fields, key order, body). Refuses everything outside the subset."""
    lines = raw.split("\n")
    if not lines or lines[0] != "---":
        raise Bail(f"{where}: no front matter (file must open with a `---` line)")
    fields: dict[str, object] = {}
    order: list[str] = []
    for n, line in enumerate(lines[1:], start=2):
        if line == "---":
            body = "\n".join(lines[n:])
            return fields, order, body
        if line.strip() == "" or line.lstrip().startswith("#"):
            continue
        if line[0] in " \t":
            raise Bail(f"{where}:{n}: indented line in front matter — nesting and block lists "
                       f"are not in the subset (use `key: [a, b]`)")
        m = KEY_RE.match(line)
        if not m:
            raise Bail(f"{where}:{n}: not a `key: value` line: {line!r}")
        key, value = m.group(1), (m.group(2) or "")
        if key in fields:
            raise Bail(f"{where}:{n}: duplicate key `{key}`")
        value = value.strip()
        if value.startswith("["):
            if not value.endswith("]"):
                raise Bail(f"{where}:{n}: list for `{key}` must close on the same line")
            inner = value[1:-1].strip()
            fields[key] = [] if inner == "" else [_scalar(p) for p in inner.split(",")]
        elif value.startswith(("&", "*", "|", ">", "{")):
            raise Bail(f"{where}:{n}: `{value[0]}` starts a YAML feature outside the subset")
        else:
            fields[key] = _scalar(value)
        order.append(key)
    raise Bail(f"{where}: front matter never closed (no second `---` line)")


def format_front_matter(fields: dict[str, object], order: list[str]) -> str:
    keys = [k for k in order if k in fields] + [k for k in fields if k not in order]
    out = ["---"]
    for k in keys:
        v = fields[k]
        if isinstance(v, list):
            out.append(f"{k}: [{', '.join(_fmt_scalar(x, in_list=True) for x in v)}]")
        elif v is None:
            out.append(f"{k}:")
        else:
            out.append(f"{k}: {_fmt_scalar(v)}")
    out.append("---")
    return "\n".join(out) + "\n"


def _fmt_scalar(v: object, in_list: bool = False) -> str:
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    s = str(v)
    if in_list and "," in s:
        raise Bail(f"a list element may not contain a comma: {s!r}")
    looks_typed = bool(INT_RE.match(s)) or s in ("true", "false")
    # QUOTE ONLY WHAT THE READER WOULD MISREAD, and let `_scalar` be the judge
    # of that rather than a second list of characters. The two disagreed: the
    # reader strips a pair of quotes only when the value opens AND closes with
    # one, so a title that merely BEGINS with a quoted phrase round-trips bare
    # — while the writer quoted on the first character alone and then refused
    # the value for containing a quote. The result was a header `work.py` could
    # read and could not write, so `set` failed on the whole item for the shape
    # of one field it was not touching (`S58`, 2026-09-20).
    needs_quoting = (s == "" or s != s.strip() or s[0] in "[&*|>{"
                     or looks_typed or _scalar(s) != s)
    if needs_quoting:
        if '"' in s:
            raise Bail(f"cannot quote a value that contains a double quote: {s!r}")
        return '"' + s + '"'
    return s


# --------------------------------------------------------------------------
# loading
# --------------------------------------------------------------------------

def tracked_files(root: str) -> list[str]:
    r = subprocess.run(["git", "-C", root, "ls-files", "-z"], capture_output=True, check=False)
    if r.returncode != 0:
        raise Bail(f"git ls-files failed under {root}: {r.stderr.decode(errors='replace').strip()}")
    return [p for p in r.stdout.decode().split("\0") if p]


def load_tree(root: str) -> tuple[list[Item], list[str]]:
    """Every parsed file under work/, plus structural errors."""
    errors: list[str] = []
    items: list[Item] = []
    wdir = os.path.join(root, WORK)
    if not os.path.isdir(wdir):
        raise Bail(f"{WORK}/ does not exist under {root}")
    for entry in sorted(os.listdir(wdir)):
        full = os.path.join(wdir, entry)
        rel = f"{WORK}/{entry}"
        if os.path.isfile(full):
            if entry not in FREE_FILES:
                errors.append(f"{rel}: a file at the top of {WORK}/ must be one of {sorted(FREE_FILES)}; "
                              f"items live in a program directory or {WORK}/{ISSUES_DIR}/")
            continue
        if entry == ISSUES_DIR:
            for name in sorted(os.listdir(full)):
                p = os.path.join(full, name)
                if os.path.isdir(p) or not name.endswith(".md"):
                    errors.append(f"{rel}/{name}: only `.md` items belong under {WORK}/{ISSUES_DIR}/")
                    continue
                # The directory's own signpost, not an item: it says what
                # belongs here (issues with no owner yet) so a lane reading
                # the directory does not have to infer the rule from the
                # files already in it. Unparsed for the same reason
                # `work/README.md` is.
                if name in FREE_FILES:
                    continue
                _load_item(p, f"{rel}/{name}", None, items, errors)
            continue
        if not ID_RE.match(entry) or entry != entry.lower():
            errors.append(f"{rel}/: a program directory is a lowercase id")
            continue
        names = sorted(os.listdir(full))
        if "program.md" not in names:
            errors.append(f"{rel}/: no program.md")
            continue
        for name in names:
            p = os.path.join(full, name)
            if os.path.isdir(p) or name in NARRATIVE or not name.endswith(".md"):
                continue          # narrative and free-form subtrees are unparsed
            _load_item(p, f"{rel}/{name}", entry, items, errors)
    return items, errors


def _load_item(path: str, rel: str, program: str | None, items: list[Item], errors: list[str]) -> None:
    with open(path, encoding="utf-8") as f:
        raw = f.read()
    try:
        fields, order, body = parse_front_matter(raw, rel)
    except Bail as e:
        errors.append(str(e))
        return
    items.append(Item(rel, program, fields, body, order))


# --------------------------------------------------------------------------
# lint
# --------------------------------------------------------------------------

def _check_type(item: Item, key: str, typ: str, value: object) -> list[str]:
    w = f"{item.path}: `{key}`"
    if value is None:
        return [] if key == "closed" else [f"{w} is empty"]
    if typ == "str":
        return [] if isinstance(value, str) and value != "" else [f"{w} must be a string"]
    if typ == "int":
        return [] if isinstance(value, int) and not isinstance(value, bool) else [f"{w} must be an int (a PR or issue number)"]
    if typ == "date":
        return [] if isinstance(value, str) and DATE_RE.match(value) else [f"{w} must be a YYYY-MM-DD date"]
    if typ == "flag":
        return [] if value is True else [f"{w} is either `true` or absent"]
    if typ == "ref":
        return [] if isinstance(value, str) and ID_RE.match(value) else [f"{w} must be an item id"]
    if typ == "reflist":
        if not isinstance(value, list):
            return [f"{w} must be a list"]
        bad = [x for x in value if not (isinstance(x, int) and not isinstance(x, bool)) and not (isinstance(x, str) and ID_RE.match(x))]
        return [f"{w}: {x!r} is neither an item id nor a number" for x in bad]
    if typ == "strlist":
        if not isinstance(value, list):
            return [f"{w} must be a list"]
        return [f"{w}: {x!r} is not a string" for x in value if not isinstance(x, str) or x == ""]
    if typ == "enum:kind":
        return [] if value in KINDS else [f"{w} must be one of {KINDS}"]
    if typ == "enum:area":
        return [] if value in AREAS else [f"{w} must be one of {AREAS}"]
    if typ == "enum:priority":
        return [] if value in PRIORITIES else [f"{w} must be one of {PRIORITIES}"]
    if typ == "enum:cost":
        return [] if value in COSTS else [f"{w} must be one of {COSTS}"]
    if typ == "enum:status":
        vocab = PROGRAM_STATUS if item.kind == "program" else RULING_STATUS if item.kind == "ruling" else ITEM_STATUS
        return [] if value in vocab else [f"{w} must be one of {vocab} for kind {item.kind}"]
    raise AssertionError(typ)


def _listed(item: Item, key: str) -> list[object]:
    """A list-typed field's members, or none when the header does not hold a
    list. `_check_type` has already reported the shape; every reader past it
    goes through here so a malformed field is DIAGNOSED rather than crashing
    the run that was about to say so."""
    v = item.get(key)
    return v if isinstance(v, list) else []


def _load(rows: list[Item]) -> float:
    """The weighted DISPATCHABLE load a slate is carrying, in budget points.
    An unpriced row is charged the middle weight rather than nothing, so a
    track cannot come in under budget by declining to price itself."""
    total = 0.0
    for r in rows:
        if r.status not in DISPATCHABLE:
            continue
        c = r.get("cost")
        total += COST_WEIGHT.get(str(c), UNPRICED_WEIGHT)
    return total


def _budget(p: Item) -> int:
    b = p.get("budget")
    return b if isinstance(b, int) and not isinstance(b, bool) else DEFAULT_BUDGET


def _fmt_load(rows: list[Item], p: Item) -> str:
    load, cap = _load(rows), _budget(p)
    n = f"{load:g}/{cap}"
    return f"**{n}**" if load > cap else n


def _prio_key(it: Item) -> int:
    v = it.get("priority")
    return PRIORITIES.index(str(v)) if str(v) in PRIORITIES else len(PRIORITIES)


def _say_fired(vs: list[object], resolves: dict[int, Item]) -> str:
    """Fired blockers, named. An id prints as itself; a number prints as the
    number AND the item it resolved to, so the reader can check the inference
    rather than take it."""
    return ", ".join(f"`{v}`" if isinstance(v, str) else f"#{v} (`{resolves[v].id}`)" for v in vs)


def _names(text: str, program_id: str) -> bool:
    """Does a `keep_out` text name this program? Whole word, case-insensitive,
    so the role spellings the clauses actually use — `S-BOOL`, `DOCM`, `view` —
    all resolve to the id, and `bool` does not match `boolean`.

    BLIND SPOT, stated: many ids are ordinary English (`view`, `shell`, `fix`,
    `trim`, `blend`, `meta`, `curved`), so a clause that happens to use the word
    reads as a record and SUPPRESSES the warning. It fails quiet, in the
    direction of not nagging, which is the right way round for an advisory —
    and it is a measured hazard, not a hypothetical: today `bool`'s clause
    contains "ops.rs's curved arm" as well as the real "are CURVED's", and only
    the second is a record. Checked at the landing: all three currently-passing
    pairs are genuine mutual records. The fix, if this ever passes a pair it
    should not, is a `keep_out` that names programs in a field rather than in
    prose — a schema change, filed with the error flip."""
    return re.search(rf"\b{re.escape(program_id)}\b", text, re.I) is not None


def _double_claims(root: str, programs: list[Item]) -> list[str]:
    """Pairs of OPEN programs whose `paths` globs match a common tracked path,
    less those both `keep_out`s name — the territory map at rest.

    **Shared ground between programs is legitimate and expected**, and an
    overlap neither side wrote down is not a conflict (Ev, in chat,
    2026-09-20: *"it's ok if units have shared ground, they should just be
    aware of each other if working at the same time"*). What two programs owe
    each other is awareness while a lane is LIVE on a shared file, and that is
    a per-branch question `territory` already answers — it reads a branch's
    diff and names every path another program claims. This census answers the
    at-rest question instead: which programs share ground at all.

    So it is a REPORT, printed by `work.py territory --overlaps`, and not a
    lint warning. It used to fire on every `lint` run, which put 25 warnings in
    front of every reader of every program for a condition none of them was
    expected to fix — a warning nobody can act on trains people to skip
    warnings, which costs more than the census is worth. The rule it used to
    enforce is gone from `work/README.md` with it; the open row that carried
    the error flip, `double-claim-lint-rule-waits-on-the-tests-seam`, records
    the ruling that retired the question."""
    if len(programs) < 2:
        return []
    tracked = tracked_files(root)
    owners: dict[str, list[str]] = {}
    for p in programs:
        globs = [g for g in _listed(p, "paths") if isinstance(g, str)]
        if not globs:
            continue
        for path in tracked:
            if any(fnmatch.fnmatchcase(path, g) for g in globs):
                owners.setdefault(path, []).append(p.id)
    shared: dict[tuple[str, str], int] = {}
    for ids in owners.values():
        if len(ids) > 1:
            for i in range(len(ids)):
                for j in range(i + 1, len(ids)):
                    key = (ids[i], ids[j]) if ids[i] < ids[j] else (ids[j], ids[i])
                    shared[key] = shared.get(key, 0) + 1
    keep_out = {p.id: " ".join(str(x) for x in _listed(p, "keep_out")) for p in programs}
    path_of = {p.id: p.path for p in programs}
    out = []
    for (a, b), n in sorted(shared.items(), key=lambda kv: (-kv[1], kv[0])):
        ab, ba = _names(keep_out[a], b), _names(keep_out[b], a)
        if ab and ba:
            continue
        if ab or ba:
            first, second = (a, b) if ab else (b, a)
            why = (f"recorded by `{first}` only, so it is not visible from `{second}`'s "
                   f"side — worth a line there if a lane is likely to be live on both")
        else:
            why = "recorded by neither, so a live lane on one is invisible to the other"
        out.append(f"{path_of[a]}: shares {n} tracked path{'s' if n != 1 else ''} with `{b}`; {why}")
    return out


def lint(root: str, warnings: list[str] | None = None) -> list[str]:
    """Errors, sorted and deduped. `warnings` (an out-parameter) collects the
    advisories: a lint error blocks a merge, a warning names a row and leaves
    the fix to whoever owns it."""
    items, errors = load_tree(root)
    warns: list[str] = []
    by_id: dict[str, Item] = {}
    # The migrated-issue map: `github: 1202` says this item is what GitHub
    # issue 1202 became, which is the only thing in the tree that can turn a
    # number in `blocked_on` back into a row. A number two rows both claim
    # resolves to NEITHER — an ambiguous mapping is not a mapping — and is
    # reported instead.
    by_github: dict[int, list[Item]] = {}
    for it in items:
        # keys and types
        for key in it.fields:
            if key not in SCHEMA:
                errors.append(f"{it.path}: unknown key `{key}` (add it to SCHEMA in scripts/work.py in the PR that first uses it)")
        for key in REQUIRED:
            if key not in it.fields:
                errors.append(f"{it.path}: missing `{key}`")
        for key, value in it.fields.items():
            if key not in SCHEMA:
                continue
            typ, kinds = SCHEMA[key]
            if it.kind in KINDS and it.kind not in kinds:
                errors.append(f"{it.path}: `{key}` is not a field of kind {it.kind}")
                continue
            errors.extend(_check_type(it, key, typ, value))
        # placement and naming
        stem = os.path.basename(it.path)[:-3]
        if it.kind == "program":
            if stem != "program":
                errors.append(f"{it.path}: kind program belongs only in program.md")
            elif it.id != it.program:
                errors.append(f"{it.path}: id `{it.id}` must equal the directory name `{it.program}`")
        else:
            if stem == "program":
                errors.append(f"{it.path}: program.md must have kind program")
            elif it.id and it.id != stem:
                errors.append(f"{it.path}: id `{it.id}` must equal the file name")
            if it.program is None and it.kind not in ("issue", ""):
                errors.append(f"{it.path}: only kind issue lives under {WORK}/{ISSUES_DIR}/")
        if it.id:
            if it.id in by_id:
                errors.append(f"{it.path}: id `{it.id}` is already {by_id[it.id].path}")
            else:
                by_id[it.id] = it
        gh = it.get("github")
        if isinstance(gh, int) and not isinstance(gh, bool):
            by_github.setdefault(gh, []).append(it)
    for gh, claimants in sorted(by_github.items()):
        if len(claimants) > 1:
            warns.append(f"{claimants[0].path}: `github` {gh} is claimed by "
                         f"{', '.join(f'`{c.id}`' for c in claimants)} — one GitHub issue became one item, "
                         f"and a number two rows both claim resolves to neither, so a `blocked_on: [{gh}]` "
                         f"goes unchecked")
    resolves = {gh: c[0] for gh, c in by_github.items() if len(c) == 1}
    programs = {it.id: it for it in items if it.kind == "program" and it.id}
    owned: dict[str, list[Item]] = defaultdict(list)
    for it in items:
        if it.kind != "program" and it.program:
            owned[it.program].append(it)
    tracked: list[str] | None = None
    for it in items:
        # status-coupled fields
        if it.status == "closed" and it.get("closed") is None:
            errors.append(f"{it.path}: status closed needs a `closed:` date")
        if it.status != "closed" and it.get("closed") is not None:
            errors.append(f"{it.path}: `closed:` is set but status is {it.status}")
        if it.status == "parked" and not it.get("blocked_on"):
            errors.append(f"{it.path}: parked needs a non-empty `blocked_on`")
        if it.status == "deferred" and it.get("blocked_on"):
            errors.append(f"{it.path}: deferred is a ratified not-now, not a wait on a named trigger — "
                          f"a row with `blocked_on` is parked; cite the ratification in the body instead")
        if it.status == "parked":
            # A trigger that has fired. `blocked_on` still resolves, so nothing
            # else here objects and the row reads blocked forever.
            named = [v for v in _listed(it, "blocked_on")
                     if isinstance(v, str) and v in by_id and by_id[v].status == "closed"]
            # An int reaches the rule only INFERRED, through `github:`: the
            # author wrote a number, not a reference, and the tracker matched it
            # to the item that number became. A true inference is still an
            # inference, so it WARNS and names the row where a naming errors —
            # `work/README.md` gives the reason. A number matching no `github:`,
            # or two, stays unchecked, as every int did before. (GitHub numbers
            # PRs and issues from one sequence per repo, so the number a row is
            # parked on cannot be a PR number AND some other row's issue.)
            inferred = [v for v in _listed(it, "blocked_on")
                        if isinstance(v, int) and not isinstance(v, bool)
                        and v in resolves and resolves[v].status == "closed"]
            fired = named + inferred
            if fired:
                rest = [_fmt_ref(v) for v in _listed(it, "blocked_on") if v not in fired]
                if named and not rest:
                    errors.append(f"{it.path}: parked on {_say_fired(named, resolves)}, which is closed — "
                                  f"that trigger has fired and nothing else gates this row; "
                                  f"re-park it on what does, open it, or defer it")
                elif named:
                    # Still blocked, so the status is true and only the entry is stale.
                    warns.append(f"{it.path}: parked on {_say_fired(named, resolves)}, which is closed — "
                                 f"that trigger has fired; it still waits on {', '.join(rest)}, "
                                 f"so prune the fired entry")
                if inferred:
                    gate = "nothing else gates this row" if not rest else f"it still waits on {', '.join(rest)}"
                    warns.append(f"{it.path}: parked on {_say_fired(inferred, resolves)}, which is closed — "
                                 f"the number resolves through that item's `github:` and its trigger has fired; "
                                 f"{gate}. Name the item instead of the number, then the rule reads it directly")
        # references
        for key in ("parent", "rides_with"):
            v = it.get(key)
            if isinstance(v, str) and v not in by_id:
                errors.append(f"{it.path}: `{key}` names `{v}`, which is no item")
        for key in ("blocked_on", "refs"):
            for v in _listed(it, key):
                if isinstance(v, str) and v not in by_id:
                    errors.append(f"{it.path}: `{key}` names `{v}`, which is no item")
        carrier = it.get("rides_with")
        if isinstance(carrier, str) and carrier in by_id and by_id[carrier].status == "closed" and it.status != "closed":
            errors.append(f"{it.path}: rides with `{carrier}`, which is closed — re-home it (a struck row may not delete its passengers)")
        if it.kind == "program":
            live = [o for o in owned.get(it.id, []) if o.status != "closed"]
            disp = [o.id for o in live if o.status in DISPATCHABLE]
            if it.status == "blocked":
                # What `blocked` claims about an orchestrator is unfalsifiable
                # here; what it claims about the slate is not.
                if disp:
                    errors.append(f"{it.path}: program is blocked but {disp} are dispatchable — a track with "
                                  f"a row to pick up is `ready`, or `active` if an orchestrator holds it")
                elif not live:
                    errors.append(f"{it.path}: program is blocked but holds no live row — a track waiting on "
                                  f"nothing is finished, not blocked; close it")
            elif it.status == "ready" and live and not disp:
                errors.append(f"{it.path}: program is ready but none of its {len(live)} live rows is "
                              f"dispatchable — a track nobody can pick up is `blocked`, or `active` if an "
                              f"orchestrator holds it")
            prefix = it.get("prefix")
            if isinstance(prefix, str) and not prefix.endswith("/"):
                errors.append(f"{it.path}: `prefix` must end in `/`")
            for g in _listed(it, "paths"):
                if not isinstance(g, str):
                    continue
                if tracked is None:
                    tracked = tracked_files(root)
                if not any(fnmatch.fnmatchcase(p, g) for p in tracked):
                    errors.append(f"{it.path}: territory glob `{g}` matches no tracked path")
    # nothing of a program's lives in docs/ any more
    docs = os.path.join(root, "docs")
    if os.path.isdir(docs):
        for name in sorted(os.listdir(docs)):
            rel = f"docs/{name}"
            if (name.endswith("-PLAN.md") or name.endswith("-LOG.md")) and rel not in LOG_EXEMPT:
                errors.append(f"{rel}: plans and logs live in {WORK}/<program>/ (plan.md, log.md), not in docs/")
    if not programs and not errors:
        errors.append(f"{WORK}/ holds no program")
    if warnings is not None:
        warnings.extend(sorted(set(warns)))
    return sorted(set(errors))


# --------------------------------------------------------------------------
# render
# --------------------------------------------------------------------------

def _last_touched(root: str) -> dict[str, str]:
    """path -> date of the last commit touching it, from one git call."""
    r = subprocess.run(["git", "-C", root, "log", "--format=%x00%cs", "--name-only", "--", WORK],
                       capture_output=True, check=False)
    out: dict[str, str] = {}
    if r.returncode != 0:
        return out
    date = ""
    for line in r.stdout.decode(errors="replace").split("\n"):
        if line.startswith("\0"):
            date = line[1:].strip()
        elif line.strip() and line.strip() not in out:
            out[line.strip()] = date
    return out


def _fmt_ref(v: object) -> str:
    return f"#{v}" if isinstance(v, int) else str(v)


def render(root: str, only_program: str | None = None, today: dt.date | None = None) -> str:
    items, errors = load_tree(root)
    if errors:
        raise Bail("cannot render an invalid tree; run `work.py lint`:\n  " + "\n  ".join(errors))
    today = today or dt.date.today()
    touched = _last_touched(root)
    programs = sorted((it for it in items if it.kind == "program"), key=lambda p: (str(p.get("area")), p.id))
    by_program: dict[str | None, list[Item]] = defaultdict(list)
    for it in items:
        if it.kind != "program":
            by_program[it.program].append(it)
    live = [it for it in items if it.kind != "program" and it.status != "closed"]
    if only_program:
        programs = [p for p in programs if p.id == only_program]
        if not programs:
            raise Bail(f"no program `{only_program}`")

    out: list[str] = []
    out.append("# Work status")
    out.append("")
    out.append(f"Generated by `scripts/work.py render` on {today.isoformat()} — do not edit; "
               "`work/README.md` says how this file is produced and what each section means.")
    out.append("")

    # Ev's queue
    if not only_program:
        queue = sorted((it for it in live if it.get("needs_ev") is not None), key=lambda i: (str(i.get("opened")), i.id))
        out.append("## Waiting on Ev")
        out.append("")
        if queue:
            out.append("| since | item | program | status | title |")
            out.append("|---|---|---|---|---|")
            for it in queue:
                out.append(f"| {it.get('opened')} | `{it.id}` | {it.program or '—'} | {it.status} | {it.get('title')} |")
        else:
            out.append("Nothing.")
        out.append("")

    # board
    out.append("## Programs")
    out.append("")
    bands = " | ".join(PRIORITIES)
    out.append(f"| area | program | pri | {bands} | load | status | open | spec | dispatched | review | parked | deferred | closed | on Ev |")
    out.append("|---|---|---|" + "---|" * len(PRIORITIES) + "---|---|---|---|---|---|---|---|---|---|")
    totals = {b: 0 for b in PRIORITIES}
    for p in sorted(programs, key=lambda q: (_prio_key(q), str(q.get("area") or ""), q.id)):
        rows = by_program.get(p.id, [])
        c = {s: sum(1 for r in rows if r.status == s) for s in ITEM_STATUS}
        ev = sum(1 for r in rows if r.status != "closed" and r.get("needs_ev") is not None)
        # Band counts are over LIVE rows, whatever their status: a parked P0 is
        # still P0 work this track holds, and the status columns beside them
        # already say which rows are dispatchable.
        live_rows = [r for r in rows if r.status != "closed"]
        band = {b: sum(1 for r in live_rows if str(r.get("priority")) == b) for b in PRIORITIES}
        for b in PRIORITIES:
            totals[b] += band[b]
        cells = " | ".join(str(band[b] or "") for b in PRIORITIES)
        out.append(f"| {p.get('area') or '—'} | `{p.id}` | {p.get('priority') or '—'} | {cells} | "
                   f"{_fmt_load(rows, p)} | {p.status} | {c['open']} | {c['spec']} | "
                   f"{c['dispatched']} | {c['review']} | {c['parked']} | {c['deferred']} | {c['closed']} | "
                   f"{ev or ''} |")
    out.append("| | **all programs** | | " + " | ".join(f"**{totals[b]}**" for b in PRIORITIES)
               + " | | | | | | | | | | |")
    out.append("")
    out.append("`P0`–`P4` count this program's LIVE rows in each band "
               "(`work/README.md`, Priority); a row is counted whatever its "
               "status, and the status columns say which of them are "
               "dispatchable. `status` is the TRACK's own state: `ready` "
               "(no orchestrator, and a row to pick up), `active` (an "
               "orchestrator holds it), `blocked` (no orchestrator, and "
               "nothing dispatchable). `load` is the DISPATCHABLE weight against the "
               "track's budget (Track size); bold is over. `pri` is the band "
               "of the track's spine, never a ceiling on its rows.")
    out.append("")

    # per-program slates
    for p in programs:
        rows = [r for r in by_program.get(p.id, []) if r.status != "closed"]
        out.append(f"## `{p.id}` — {p.get('title')}")
        out.append("")
        meta = []
        for key in ("area", "prefix", "tag", "ab_band"):
            if p.get(key) is not None:
                meta.append(f"{key} `{p.get(key)}`")
        if meta:
            out.append("; ".join(meta) + ".")
            out.append("")
        if p.status == "blocked":
            out.append("Blocked: no orchestrator, and no row that can be picked up.")
            out.append("")
        if not rows:
            out.append("No open items.")
            out.append("")
            continue
        out.append("| pri | item | kind | cost | status | title | blocked on | PR |")
        out.append("|---|---|---|---|---|---|---|---|")
        for r in sorted(rows, key=lambda i: (_prio_key(i), ITEM_STATUS.index(i.status) if i.status in ITEM_STATUS else 9, i.id)):
            blocked = ", ".join(_fmt_ref(b) for b in _listed(r, "blocked_on"))
            pr = f"#{r.get('pr')}" if r.get("pr") is not None else ""
            ev = " **[ev]**" if r.get("needs_ev") is not None else ""
            out.append(f"| {r.get('priority') or '—'} | `{r.id}` | {r.kind} | {r.get('cost') or '—'} | "
                       f"{r.status}{ev} | {r.get('title')} | {blocked} | {pr} |")
        out.append("")

    if only_program:
        return "\n".join(out)

    # unowned issues
    unowned = [it for it in by_program.get(None, []) if it.status != "closed"]
    out.append("## Issues no program owns")
    out.append("")
    if unowned:
        out.append("| item | opened | title |")
        out.append("|---|---|---|")
        for it in sorted(unowned, key=lambda i: (str(i.get("opened")), i.id)):
            out.append(f"| `{it.id}` | {it.get('opened')} | {it.get('title')} |")
    else:
        out.append("None.")
    out.append("")

    # blocked
    blocked = [it for it in live if it.get("blocked_on")]
    out.append("## Blocked")
    out.append("")
    if blocked:
        out.append("| item | program | status | blocked on |")
        out.append("|---|---|---|---|")
        for it in sorted(blocked, key=lambda i: (i.program or "", i.id)):
            out.append(f"| `{it.id}` | {it.program or '—'} | {it.status} | "
                       f"{', '.join(_fmt_ref(b) for b in _listed(it, 'blocked_on'))} |")
    else:
        out.append("Nothing.")
    out.append("")

    # stale
    stale = []
    for it in live:
        if it.status in ("parked", "deferred"):
            continue
        d = touched.get(it.path)
        if d:
            age = (today - dt.date.fromisoformat(d)).days
            if age >= STALE_DAYS:
                stale.append((age, it))
    out.append(f"## Untouched for {STALE_DAYS}+ days")
    out.append("")
    if stale:
        out.append("| days | item | program | status | title |")
        out.append("|---|---|---|---|---|")
        for age, it in sorted(stale, key=lambda t: (-t[0], t[1].id)):
            out.append(f"| {age} | `{it.id}` | {it.program or '—'} | {it.status} | {it.get('title')} |")
    else:
        out.append("Nothing.")
    out.append("")
    return "\n".join(out)


# --------------------------------------------------------------------------
# new / set
# --------------------------------------------------------------------------

def _parse_assignment(text: str) -> tuple[str, object]:
    if "=" not in text:
        raise Bail(f"expected key=value, got {text!r}")
    key, value = text.split("=", 1)
    key = key.strip()
    if key not in SCHEMA:
        raise Bail(f"unknown key `{key}`")
    value = value.strip()
    if value == "":
        return key, None
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        return key, [] if inner == "" else [_scalar(p) for p in inner.split(",")]
    # A scalar written into a list-typed field is what a caller means, not a
    # malformed header: `blocked_on=2171` is one blocker. Without this the bare
    # spelling wrote `blocked_on: 2171` and the next lint died reading it, and
    # nothing told the caller `[2171]` was the working spelling. lint diagnoses
    # the shape too (a header may be hand-written), but the tool no longer
    # produces it.
    if SCHEMA[key][0] in LIST_TYPES:
        return key, [_scalar(value)]
    return key, _scalar(value)


def _find(root: str, item_id: str) -> Item:
    items, _ = load_tree(root)
    for it in items:
        if it.id == item_id:
            return it
    raise Bail(f"no item `{item_id}`")


def cmd_new(root: str, item_id: str, kind: str, title: str, program: str | None, extra: list[str]) -> str:
    if not ID_RE.match(item_id):
        raise Bail(f"`{item_id}` is not an id")
    if kind == "program":
        raise Bail("a program is created by hand: work/<id>/program.md, plan.md, log.md")
    if program:
        d = os.path.join(root, WORK, program)
        if not os.path.isfile(os.path.join(d, "program.md")):
            raise Bail(f"no program `{program}`")
    else:
        if kind != "issue":
            raise Bail(f"a {kind} needs --program; only issues live under {WORK}/{ISSUES_DIR}/")
        d = os.path.join(root, WORK, ISSUES_DIR)
        os.makedirs(d, exist_ok=True)
    path = os.path.join(d, item_id + ".md")
    if os.path.exists(path):
        raise Bail(f"{os.path.relpath(path, root)} exists")
    fields: dict[str, object] = {"id": item_id, "kind": kind, "title": title, "status": "open",
                                 "opened": dt.date.today().isoformat()}
    order = list(fields)
    for a in extra:
        k, v = _parse_assignment(a)
        fields[k] = v
        if k not in order:
            order.append(k)
    with open(path, "w", encoding="utf-8") as f:
        f.write(format_front_matter(fields, order))
        f.write("\n")
    return os.path.relpath(path, root)


def cmd_set(root: str, item_id: str, assignments: list[str]) -> str:
    it = _find(root, item_id)
    for a in assignments:
        k, v = _parse_assignment(a)
        if v is None:
            it.fields.pop(k, None)
        else:
            it.fields[k] = v
            if k not in it.order:
                it.order.append(k)
    # RENDER BEFORE OPENING. `format_front_matter` can refuse (a value the
    # subset cannot spell), and `open(..., "w")` truncates on the way in — so
    # rendering inside the `with` left the item EMPTY whenever `set` refused,
    # which is a refusal destroying the thing it declined to change. Build the
    # whole text first; the file is only touched once it is certain to be
    # written whole (found 2026-09-20, on `S58`).
    text = format_front_matter(it.fields, it.order) + it.body
    with open(os.path.join(root, it.path), "w", encoding="utf-8") as f:
        f.write(text)
    return it.path


# --------------------------------------------------------------------------
# territory
# --------------------------------------------------------------------------

def territory(root: str, base: str | None, branch: str | None, files: list[str] | None = None) -> tuple[list[str], str | None]:
    """(collision lines, the program the branch belongs to). `files` replaces the
    diff against `base` — hosted CI hands in the PR's own path list."""
    items, errors = load_tree(root)
    if errors:
        raise Bail("invalid tree; run `work.py lint`")
    programs = [it for it in items if it.kind == "program"]
    if branch is None:
        branch = _current_branch(root)
    mine = _branch_program(programs, branch)
    if files is None:
        if base is None:
            raise Bail("territory needs --base or --files")
        r = subprocess.run(["git", "-C", root, "diff", "--name-only", f"{base}...HEAD"], capture_output=True, check=False)
        if r.returncode != 0:
            raise Bail(f"git diff against {base} failed: {r.stderr.decode(errors='replace').strip()}")
        files = r.stdout.decode().split("\n")
    lines = []
    for path in sorted(p.strip() for p in files if p.strip()):
        owners = sorted(p.id for p in programs
                        if any(fnmatch.fnmatchcase(path, g) for g in _listed(p, "paths") if isinstance(g, str)))
        others = [o for o in owners if o != mine]
        if not others:
            continue
        if mine in owners:
            # The case this check was blind to until 2026-09-11: a path the
            # branch's own program claims TOO. Reported with different wording,
            # because it is a different fact — not "you are in someone else's
            # territory" but "two programs claim this and one of them may not
            # know". Silence here is what let FIX and SHELL both hold
            # crates/topo/src/transform.rs for a day.
            lines.append(f"{path}: also claimed by {', '.join(others)}; "
                         f"{mine} claims it too — a double claim, not a crossing")
        else:
            lines.append(f"{path}: owned by {', '.join(others)}"
                         + (f"; this branch is {mine}'s" if mine else "; this branch has no program prefix"))
    return lines, mine


def _current_branch(root: str) -> str:
    r = subprocess.run(["git", "-C", root, "rev-parse", "--abbrev-ref", "HEAD"], capture_output=True, check=False)
    return r.stdout.decode().strip() if r.returncode == 0 else ""


def _branch_program(programs: list, branch: str) -> str | None:
    """The program whose `prefix` is the longest one `branch` starts with."""
    mine = None
    best = -1
    for p in programs:
        pre = p.get("prefix")
        if isinstance(pre, str) and branch.startswith(pre) and len(pre) > best:
            mine, best = p.id, len(pre)
    return mine


# --------------------------------------------------------------------------
# incoming
# --------------------------------------------------------------------------

def incoming(root: str, base: str, program: str | None, branch: str | None) -> tuple[str, int, str]:
    """(program, commit count, `git log -p`) for every commit on `base` that
    touches `work/<program>/` and is not yet in HEAD.

    WHY IT IS KEYED TO BASE, NOT TO A MERGE. A program's `log.md` merges with
    git's union driver (`.gitattributes`), so another program's notice lands
    in it with no conflict to point at it. The conflict never was a reliable
    alert — it fired only when the owner had appended too, buried among its
    own lanes' collisions — so the read is made explicit instead. `HEAD..base`
    lists a commit until it reaches this branch by any route (a base merge, a
    reset onto the base), so no merge direction can skip one; the owner's own
    lane entries show too, once their units land, and are recognisable as its
    own."""
    items, errors = load_tree(root)
    if errors:
        raise Bail("invalid tree; run `work.py lint`")
    programs = [it for it in items if it.kind == "program"]
    if program is None:
        branch = branch if branch is not None else _current_branch(root)
        program = _branch_program(programs, branch)
        if program is None:
            raise Bail(f"branch {branch!r} carries no program prefix; pass --program")
    elif program not in {p.id for p in programs}:
        raise Bail(f"no program {program!r} under {WORK}/")
    rng, path = f"HEAD..{base}", f"{WORK}/{program}/"
    r = subprocess.run(["git", "-C", root, "rev-list", "--count", rng, "--", path], capture_output=True, check=False)
    if r.returncode != 0:
        raise Bail(f"git rev-list {rng} failed (fetch {base} first?): {r.stderr.decode(errors='replace').strip()}")
    count = int(r.stdout.decode().strip())
    r = subprocess.run(["git", "-C", root, "log", "-p", "--reverse", rng, "--", path], capture_output=True, check=False)
    if r.returncode != 0:
        raise Bail(f"git log {rng} failed: {r.stderr.decode(errors='replace').strip()}")
    return program, count, r.stdout.decode(errors="replace")


# --------------------------------------------------------------------------
# selftest
# --------------------------------------------------------------------------

def _write(root: str, rel: str, text: str) -> None:
    p = os.path.join(root, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf-8") as f:
        f.write(text)


def _fixture(root: str) -> None:
    subprocess.run(["git", "-C", root, "init", "-q"], check=True)
    subprocess.run(["git", "-C", root, "config", "user.email", "x@example.invalid"], check=True)
    subprocess.run(["git", "-C", root, "config", "user.name", "fixture"], check=True)
    _write(root, "crates/mesh/src/lib.rs", "")
    _write(root, "crates/topo/src/lib.rs", "")
    _write(root, "work/README.md", "contract\n")
    _write(root, "work/mesh/program.md",
           "---\nid: mesh\nkind: program\ntitle: S-MESH\nstatus: active\nopened: 2026-08-31\n"
           "area: kernel\nprefix: mesh/\nab_band: 1200-1299\npaths: [crates/mesh/*]\n---\n")
    _write(root, "work/mesh/plan.md", "plan\n")
    _write(root, "work/mesh/log.md", "log\n")
    _write(root, "work/mesh/MESH-1.md",
           "---\nid: MESH-1\nkind: unit\ntitle: first\nstatus: review\nopened: 2026-09-01\npr: 1605\n"
           "blocked_on: [MESH-2, 1601]\n---\n\nbody\n")
    _write(root, "work/mesh/MESH-2.md",
           "---\nid: MESH-2\nkind: issue\ntitle: second\nstatus: open\nopened: 2026-09-01\nneeds_ev: true\n---\n")
    _write(root, "work/topo/program.md",
           "---\nid: topo\nkind: program\ntitle: swept clean\nstatus: ready\nopened: 2026-08-01\n"
           "area: kernel\nprefix: topo/\npaths: [crates/topo/*]\n---\n")
    _write(root, "work/topo/T-1.md",
           "---\nid: T-1\nkind: unit\ntitle: done\nstatus: closed\nopened: 2026-08-01\nclosed: 2026-08-19\n---\n")
    _write(root, "work/issues/stray-thing.md",
           "---\nid: stray-thing\nkind: issue\ntitle: unowned\nstatus: open\nopened: 2026-09-02\n---\n")
    # The directory's signpost, carrying no front matter. `clean fixture`
    # below is what proves it is skipped rather than parsed as an item.
    _write(root, "work/issues/README.md", "# issues with no owner yet\n")
    subprocess.run(["git", "-C", root, "add", "-A"], check=True)
    subprocess.run(["git", "-C", root, "commit", "-q", "-m", "fixture"], check=True)


def selftest() -> int:
    failures: list[str] = []

    def expect(name: str, errors: list[str], *needles: str) -> None:
        if not needles:
            if errors:
                failures.append(f"{name}: expected clean, got {errors}")
            return
        for n in needles:
            if not any(n in e for e in errors):
                failures.append(f"{name}: no error containing {n!r}; got {errors}")

    # The fixture is COMMITTED now, so "untouched" ages are measured from
    # the real today, not from the items' authored dates: a render date
    # fixed in the calendar stopped being STALE_DAYS past the commit the
    # day the calendar caught up with it (2026-09-07, every run red).
    far = dt.date.today() + dt.timedelta(days=STALE_DAYS + 5)

    with tempfile.TemporaryDirectory() as root:
        _fixture(root)
        warns: list[str] = []
        expect("clean fixture", lint(root, warns))
        expect("clean fixture (warnings)", warns)
        text = render(root, today=far)
        for needle in ("## Waiting on Ev", "`MESH-2`", "`stray-thing`", "## Blocked", "MESH-2, #1601", "`topo`"):
            if needle not in text:
                failures.append(f"render lacks {needle!r}")
        if "MESH-1" not in text.split("## Untouched")[1]:
            failures.append(f"render: fixture items committed 'today' should still be listed stale at +{STALE_DAYS + 5} days")
        p = render(root, only_program="mesh", today=far)
        if "## Waiting on Ev" in p or "`topo`" in p:
            failures.append("--program render leaked whole-board sections")

        # new / set round-trip, the CLI spelling included
        if main(["--root", root, "new", "MESH-4", "--kind", "issue", "--title", "fourth", "--program", "mesh", "--set", "track=R", "--set", "opened=2026-09-03"]) != 0:
            failures.append("CLI `new` with --set fields failed")
        elif _find(root, "MESH-4").get("track") != "R":
            failures.append("CLI `new` dropped its trailing assignments")
        os.remove(os.path.join(root, "work/mesh/MESH-4.md"))
        rel = cmd_new(root, "MESH-3", "unit", "third", "mesh", ["track=R"])
        expect("after new", lint(root))
        cmd_set(root, "MESH-3", ["status=parked", "blocked_on=[MESH-1, 42]", "track="])
        it = _find(root, "MESH-3")
        if it.get("blocked_on") != ["MESH-1", 42] or it.get("track") is not None or it.status != "parked":
            failures.append(f"set round-trip: {it.fields}")
        expect("after set", lint(root))
        os.remove(os.path.join(root, rel))

        # THE EXEMPTION IS README.md ALONE, not "prose under issues/". Without
        # this, a skip that widened to any un-parsed .md would let a malformed
        # item sit in the directory unread, which is the failure the whole
        # front-matter contract exists to prevent.
        _write(root, "work/issues/NOTES.md", "loose prose, no front matter\n")
        expect("a non-README .md in issues/ is still an item",
               lint(root), "front matter")
        os.remove(os.path.join(root, "work/issues/NOTES.md"))
        expect("issues/ clean again", lint(root))

        # one mutation per rule, each restored
        cases: list[tuple[str, str, str, str]] = [
            ("unknown key", "work/mesh/MESH-2.md", "needs_ev: true", "colour: red"),
            ("needs_ev is a bare true", "work/mesh/MESH-2.md", "needs_ev: true", "needs_ev: 1700"),
            ("dangling ref", "work/mesh/MESH-1.md", "blocked_on: [MESH-2, 1601]", "blocked_on: [MESH-9]"),
            ("id vs file name", "work/mesh/MESH-1.md", "id: MESH-1", "id: MESH-7"),
            ("bad status", "work/mesh/MESH-1.md", "status: review", "status: landed"),
            ("closed needs date", "work/mesh/MESH-1.md", "status: review", "status: closed"),
            ("parked needs blocker", "work/mesh/MESH-2.md", "status: open", "status: parked"),
            ("glob matches nothing", "work/mesh/program.md", "paths: [crates/mesh/*]", "paths: [crates/nope/*]"),
            ("prefix shape", "work/mesh/program.md", "prefix: mesh/", "prefix: mesh"),
            ("program field on a unit", "work/mesh/MESH-1.md", "pr: 1605", "prefix: x/"),
            ("nested yaml refused", "work/mesh/MESH-1.md", "pr: 1605", "pr:\n  - 1605"),
            ("unit outside a program", "work/issues/stray-thing.md", "kind: issue", "kind: unit"),
            ("parked on a fired trigger", "work/mesh/MESH-1.md",
             "status: review\nopened: 2026-09-01\npr: 1605\nblocked_on: [MESH-2, 1601]",
             "status: parked\nopened: 2026-09-01\npr: 1605\nblocked_on: [T-1]"),
            ("a fired trigger beside a live one only warns", "work/mesh/MESH-1.md",
             "status: review\nopened: 2026-09-01\npr: 1605\nblocked_on: [MESH-2, 1601]",
             "status: parked\nopened: 2026-09-01\npr: 1605\nblocked_on: [T-1, MESH-2]"),
            ("deferred may not name a blocker", "work/mesh/MESH-1.md",
             "status: review", "status: deferred"),
        ]
        expectations = ["unknown key", "either `true` or absent", "no item", "must equal the file name", "must be one of",
                        "needs a `closed:` date", "non-empty `blocked_on`",
                        "matches no tracked path", "must end in `/`", "not a field of kind unit",
                        "indented line", "only kind issue lives under",
                        "nothing else gates this row", "so prune the fired entry",
                        "cite the ratification in the body"]
        for (name, rel, old, new), needle in zip(cases, expectations, strict=True):
            p = os.path.join(root, rel)
            with open(p, encoding="utf-8") as f:
                original = f.read()
            if old not in original:
                failures.append(f"{name}: fixture lacks {old!r}")
                continue
            _write(root, rel, original.replace(old, new))
            warns = []
            expect(name, lint(root, warns) + warns, needle)
            _write(root, rel, original)
        expect("restored fixture", lint(root))

        # THE PROGRAM'S OWN STATUS. `active` is unfalsifiable from the tree,
        # so the two checkable halves are the ones tested: a blocked track with
        # a row to pick up, and a ready track with nothing to pick up.
        mp0 = open(os.path.join(root, "work/mesh/program.md"), encoding="utf-8").read()
        _write(root, "work/mesh/program.md", mp0.replace("status: active", "status: blocked"))
        expect("blocked with a dispatchable row", lint(root), "but ['MESH-2'] are dispatchable")
        _write(root, "work/mesh/program.md", mp0.replace("status: active", "status: ready"))
        expect("ready with a dispatchable row is clean", lint(root))
        o2 = open(os.path.join(root, "work/mesh/MESH-2.md"), encoding="utf-8").read()
        _write(root, "work/mesh/MESH-2.md", o2.replace("status: open", "status: deferred"))
        expect("ready with nothing dispatchable", lint(root), "none of its 2 live rows is dispatchable")
        _write(root, "work/mesh/program.md", mp0.replace("status: active", "status: blocked"))
        expect("blocked with nothing dispatchable is clean", lint(root))
        text = render(root, today=far)
        if "Blocked: no orchestrator" not in text:
            failures.append("render: a blocked track does not say so on its slate")
        _write(root, "work/mesh/MESH-2.md", o2)
        _write(root, "work/topo/program.md",
               open(os.path.join(root, "work/topo/program.md"), encoding="utf-8").read()
               .replace("status: ready", "status: blocked"))
        expect("blocked with no live row at all", lint(root), "waiting on nothing is finished")
        subprocess.run(["git", "-C", root, "checkout", "-q", "--", "work/topo/program.md"], check=True)
        _write(root, "work/mesh/program.md", mp0)
        expect("restored after the program-status cases", lint(root))

        # deferred: a ratified not-now, no blocker, its own render column, and
        # NOT the fired-trigger error's subject
        orig2 = open(os.path.join(root, "work/mesh/MESH-2.md"), encoding="utf-8").read()
        _write(root, "work/mesh/MESH-2.md", orig2.replace("status: open", "status: deferred"))
        warns = []
        expect("a deferred row needs no blocker", lint(root, warns))
        expect("a deferred row raises no warning", warns)
        text = render(root, today=far)
        if "| parked | deferred | closed |" not in text:
            failures.append("render: the programs table has no deferred column")
        if "`MESH-2` | issue | — | deferred" not in text:
            failures.append("render: a deferred row does not read deferred on its program's slate")
        if "MESH-2" in text.split("## Untouched")[1]:
            failures.append("render: a deferred row is listed stale for going untouched")
        _write(root, "work/mesh/MESH-2.md", orig2)

        # SCALAR ROUND-TRIP: every value the reader accepts, the writer must be
        # able to write back. The two used to disagree on a value that opens
        # with a quote and does not close with one, which `set` met as a refusal
        # to touch the item at all rather than as a note about one field.
        for _v in ('"A phrase in quotes" and then prose', "plain", "123", "true",
                   "", " leading", "'whole'", "[bracketed]", 'a "quoted" middle',
                   'trailing quote"'):
            try:
                if _scalar(_fmt_scalar(_v)) != _v:
                    failures.append(f"scalar round-trip: {_v!r} does not survive the writer")
            except Bail as _e:
                failures.append(f"scalar round-trip: {_v!r} refused by the writer ({_e})")

        # A REFUSED `set` LEAVES THE ITEM ALONE. The write used to truncate on
        # the way in and render inside the handle, so a field the subset cannot
        # spell emptied the file that `set` had declined to change — and the
        # offending field is usually one the caller never touched, since `set`
        # re-renders the WHOLE header. The value is planted by hand because
        # `_parse_assignment` normalises away every spelling that reaches it
        # through the CLI, which is why this only ever bit a header written
        # before the writer's rule was what it is.
        _bad = os.path.join(root, "work/mesh/MESH-1.md")
        _before = open(_bad, encoding="utf-8").read()
        _write(root, "work/mesh/MESH-1.md",
               re.sub(r"^title: .*$", "title: \"'a \"b\" c'\"", _before,
                      count=1, flags=re.M))
        _planted = open(_bad, encoding="utf-8").read()
        _refused = False
        try:
            cmd_set(root, "MESH-1", ["branch=mesh/atomicity"])
        except Bail:
            _refused = True
        if not _refused:
            failures.append("the atomicity fixture no longer refuses; the case is not being tested")
        elif open(_bad, encoding="utf-8").read() != _planted:
            failures.append("a refused `set` changed the item it refused")
        _write(root, "work/mesh/MESH-1.md", _before)

        # a fired trigger BLOCKS when nothing else gates the row, and only WARNS
        # when a live blocker remains — the two channels, told apart
        orig1 = open(os.path.join(root, "work/mesh/MESH-1.md"), encoding="utf-8").read()
        _write(root, "work/mesh/MESH-1.md",
               orig1.replace("status: review", "status: parked").replace("[MESH-2, 1601]", "[T-1]"))
        warns = []
        expect("a wholly fired trigger is an error", lint(root, warns), "nothing else gates this row")
        expect("a wholly fired trigger raises no warning", warns)
        _write(root, "work/mesh/MESH-1.md",
               orig1.replace("status: review", "status: parked").replace("[MESH-2, 1601]", "[T-1, MESH-2]"))
        warns = []
        expect("a fired trigger beside a live one is not an error", lint(root, warns))
        expect("a fired trigger beside a live one warns", warns, "prune the fired entry")
        _write(root, "work/mesh/MESH-1.md", orig1)
        expect("restored again", lint(root))

        # a scalar in a list-typed field: `set` no longer writes one, and lint
        # DIAGNOSES a hand-written one instead of dying reading it
        cmd_set(root, "MESH-2", ["refs=1601"])
        if _find(root, "MESH-2").get("refs") != [1601]:
            failures.append(f"set: a scalar into a reflist should become a one-element list, got {_find(root, 'MESH-2').get('refs')!r}")
        expect("after a scalar set", lint(root))
        cmd_set(root, "MESH-2", ["refs="])
        _write(root, "work/mesh/MESH-1.md", orig1.replace("blocked_on: [MESH-2, 1601]", "blocked_on: MESH-2"))
        warns = []
        expect("a hand-written scalar reflist is reported, not a crash", lint(root, warns), "must be a list")
        _write(root, "work/mesh/MESH-1.md", orig1)

        # the fired-trigger rule reaches an int through `github:` — and does it
        # as a warning, because the number is matched to the row, not naming it
        t1 = open(os.path.join(root, "work/topo/T-1.md"), encoding="utf-8").read()
        _write(root, "work/topo/T-1.md", t1.replace("kind: unit", "kind: unit\ngithub: 855"))
        _write(root, "work/mesh/MESH-1.md",
               orig1.replace("status: review", "status: parked").replace("[MESH-2, 1601]", "[855]"))
        warns = []
        expect("a fired trigger named by number does not block", lint(root, warns))
        expect("a fired trigger named by number warns", warns, "resolves through that item's `github:`")
        # ... and only when the mapping is unambiguous
        _write(root, "work/mesh/MESH-2.md", orig2.replace("needs_ev: true", "github: 855"))
        warns = []
        errs = lint(root, warns)
        expect("an ambiguous number is not an error", errs)
        expect("an ambiguous number resolves to neither", warns, "resolves to neither")
        if any("github:`" in w for w in warns):
            failures.append(f"an ambiguous number should not reach the fired-trigger rule: {warns}")
        _write(root, "work/mesh/MESH-2.md", orig2)
        _write(root, "work/topo/T-1.md", t1)
        _write(root, "work/mesh/MESH-1.md", orig1)
        expect("restored after the github cases", lint(root))

        # two open programs claiming one path: a warning naming both, and
        # silence once both keep_outs name the other
        mp = open(os.path.join(root, "work/mesh/program.md"), encoding="utf-8").read()
        _write(root, "work/verbs/program.md",
               "---\nid: verbs\nkind: program\ntitle: VERBS\nstatus: ready\nopened: 2026-09-01\n"
               "area: kernel\nprefix: verbs/\npaths: [crates/mesh/*]\n---\n")
        _write(root, "work/verbs/plan.md", "plan\n")
        _write(root, "work/verbs/log.md", "log\n")
        warns = []
        expect("a double claim is not an error", lint(root, warns))
        if any("shares" in w and "verbs" in w for w in warns):
            failures.append("shared ground must not warn on lint: it is legitimate (Ev, 2026-09-20)")
        progs = [it for it in load_tree(root)[0] if it.kind == "program"]
        rep = " ".join(_double_claims(root, progs))
        for want in ("shares", "verbs", "recorded by neither"):
            if want not in rep:
                failures.append(f"the at-rest overlap report should still name it: {want!r} not in {rep!r}")
        _write(root, "work/verbs/program.md",
               "---\nid: verbs\nkind: program\ntitle: VERBS\nstatus: ready\nopened: 2026-09-01\n"
               "area: kernel\nprefix: verbs/\npaths: [crates/mesh/*]\nkeep_out: [the mesh crate is S-MESH's until it cedes it]\n---\n")
        warns = []
        expect("a one-sided record is not an error", lint(root, warns))
        progs = [it for it in load_tree(root)[0] if it.kind == "program"]
        if "recorded by `verbs` only" not in " ".join(_double_claims(root, progs)):
            failures.append("the report should say which side recorded a one-sided overlap")
        _write(root, "work/mesh/program.md",
               mp.replace("paths: [crates/mesh/*]", "paths: [crates/mesh/*]\nkeep_out: [verbs holds the verb seat inside this crate]"))
        warns = []
        expect("an overlap both keep_outs name is silent", lint(root, warns))
        progs = [it for it in load_tree(root)[0] if it.kind == "program"]
        if any("verbs" in line for line in _double_claims(root, progs)):
            failures.append("an overlap both keep_outs name should leave the report too")
        _write(root, "work/mesh/program.md", mp)
        for name in ("program.md", "plan.md", "log.md"):
            os.remove(os.path.join(root, "work/verbs", name))
        os.rmdir(os.path.join(root, "work/verbs"))
        expect("restored after the double-claim cases", lint(root))

        # rides-along on a closed carrier
        _write(root, "work/topo/T-2.md",
               "---\nid: T-2\nkind: issue\ntitle: passenger\nstatus: open\nopened: 2026-08-02\nrides_with: T-1\n---\n")
        expect("passenger on struck row", lint(root), "re-home it")
        os.remove(os.path.join(root, "work/topo/T-2.md"))

        # a plan left in docs/
        _write(root, "docs/S-MESH-LOG.md", "old\n")
        expect("log in docs", lint(root), "plans and logs live in work/")
        os.remove(os.path.join(root, "docs/S-MESH-LOG.md"))
        _write(root, "docs/MODEL-AB-LOG.md", "experiment\n")
        expect("the exempt log", lint(root))

        # territory
        subprocess.run(["git", "-C", root, "checkout", "-q", "-b", "topo/x"], check=True)
        _write(root, "crates/mesh/src/other.rs", "")
        _write(root, "crates/topo/src/other.rs", "")
        subprocess.run(["git", "-C", root, "add", "-A"], check=True)
        subprocess.run(["git", "-C", root, "commit", "-q", "-m", "x"], check=True)
        crossing = "crates/topo/src/other.rs"
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "mesh/y")
        if mine != "mesh" or [ln for ln in lines if crossing not in ln] or len(lines) != 1:
            failures.append(f"territory (own program is silent, the crossing is not): {mine} {lines}")
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "verbs/z")
        if mine is not None or len(lines) != 2 or not all(
                any(f"crates/{c}/src/other.rs" in ln for ln in lines) for c in ("mesh", "topo")):
            failures.append(f"territory (foreign branch, every program's claim counts): {mine} {lines}")
        # THE BLIND SPOT: a path the branch's OWN program claims and another
        # program claims too was reported by nothing until 2026-09-11.
        mp2 = open(os.path.join(root, "work/mesh/program.md"), encoding="utf-8").read()
        _write(root, "work/verbs2/program.md",
               "---\nid: verbs2\nkind: program\ntitle: VERBS2\nstatus: ready\nopened: 2026-09-01\n"
               "area: kernel\nprefix: verbs2/\npaths: [crates/mesh/*]\n---\n")
        _write(root, "work/verbs2/plan.md", "plan\n")
        _write(root, "work/verbs2/log.md", "log\n")
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "mesh/y")
        if mine != "mesh" or not any("also claimed by verbs2" in ln and "a double claim" in ln for ln in lines):
            failures.append(f"territory (a path the branch's own program ALSO claims): {mine} {lines}")
        for name in ("program.md", "plan.md", "log.md"):
            os.remove(os.path.join(root, "work/verbs2", name))
        os.rmdir(os.path.join(root, "work/verbs2"))
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "mesh/y")
        if any("crates/mesh" in ln for ln in lines) or len(lines) != 1:
            failures.append(f"territory (own program, sole claimant): {lines}")
        _write(root, "work/mesh/program.md", mp2)

        # incoming: a note another program leaves in mesh's log reaches the
        # base; it is listed until this branch has it, however it arrives.
        base = "master" if _branch_exists(root, "master") else "main"
        subprocess.run(["git", "-C", root, "checkout", "-q", base], check=True)
        with open(os.path.join(root, "work/mesh/log.md"), "a", encoding="utf-8") as f:
            f.write("a note from topo\n")
        subprocess.run(["git", "-C", root, "commit", "-q", "-am", "topo: note on mesh's log"], check=True)
        subprocess.run(["git", "-C", root, "checkout", "-q", "topo/x"], check=True)
        prog, n, text = incoming(root, base, "mesh", None)
        if prog != "mesh" or n != 1 or "a note from topo" not in text:
            failures.append(f"incoming (a foreign note on the base is listed): {prog} {n} {text!r}")
        prog, n, _ = incoming(root, base, None, "topo/x")
        if prog != "topo" or n != 0:
            failures.append(f"incoming (the branch's program, untouched on the base): {prog} {n}")
        subprocess.run(["git", "-C", root, "merge", "-q", "--no-edit", base], check=True)
        if incoming(root, base, "mesh", None)[1] != 0:
            failures.append("incoming (a note already merged in is still listed)")

    if failures:
        for f in failures:
            print(f"SELFTEST FAIL: {f}", file=sys.stderr)
        return 1
    print("work.py selftest: ok")
    return 0


def _branch_exists(root: str, name: str) -> bool:
    r = subprocess.run(["git", "-C", root, "rev-parse", "--verify", "-q", name], capture_output=True, check=False)
    return r.returncode == 0


# --------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------

def _repo_root() -> str:
    r = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, check=False)
    if r.returncode != 0:
        raise Bail("not inside a git checkout")
    return r.stdout.decode().strip()


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(prog="work.py", description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--root", help="repo root (default: the enclosing checkout)")
    sub = ap.add_subparsers(dest="cmd")
    sub.add_parser("lint")
    s = sub.add_parser("status")
    s.add_argument("--program")
    s = sub.add_parser("render")
    s.add_argument("--out", default=STATUS_FILE)
    s = sub.add_parser("new")
    s.add_argument("id")
    s.add_argument("--kind", required=True, choices=[k for k in KINDS if k != "program"])
    s.add_argument("--title", required=True)
    s.add_argument("--program")
    s.add_argument("--set", dest="assignments", action="append", default=[], metavar="KEY=VALUE",
                   help="extra header fields, one per --set")
    s = sub.add_parser("set")
    s.add_argument("id")
    s.add_argument("assignments", nargs="+")
    s = sub.add_parser("territory")
    s.add_argument("--base", help="diff `<base>...HEAD` is the change set")
    s.add_argument("--files", help="a newline-separated path list, or `-` for stdin, instead of --base")
    s.add_argument("--branch")
    s.add_argument("--strict", action="store_true", help="exit 1 on a collision")
    s.add_argument("--overlaps", action="store_true",
                   help="instead of a diff: the at-rest map of which open programs share ground")
    s = sub.add_parser("incoming")
    s.add_argument("--base", default="origin/main", help="the branch notices arrive on (fetch it first)")
    s.add_argument("--program", help="default: the program whose prefix the current branch carries")
    args = ap.parse_args(argv)

    try:
        if args.selftest:
            return selftest()
        if not args.cmd:
            ap.print_help()
            return 2
        root = args.root or _repo_root()
        if args.cmd == "lint":
            warnings: list[str] = []
            errors = lint(root, warnings)
            for e in errors:
                print(e)
            for w in warnings:
                print(f"warning: {w}")
                if os.environ.get("GITHUB_ACTIONS"):
                    print(f"::warning file={w.split(':', 1)[0]}::{w}")
            print(f"work.py lint: {'FAIL' if errors else 'ok'} ({len(errors)} problem{'s' if len(errors) != 1 else ''}"
                  f", {len(warnings)} warning{'s' if len(warnings) != 1 else ''})")
            return 1 if errors else 0
        if args.cmd == "status":
            print(render(root, args.program))
            return 0
        if args.cmd == "render":
            text = render(root)
            with open(os.path.join(root, args.out), "w", encoding="utf-8") as f:
                f.write(text + "\n")
            print(f"wrote {args.out}")
            return 0
        if args.cmd == "new":
            print(cmd_new(root, args.id, args.kind, args.title, args.program, args.assignments))
            return 0
        if args.cmd == "set":
            print(cmd_set(root, args.id, args.assignments))
            return 0
        if args.cmd == "territory":
            if args.overlaps:
                progs = [it for it in load_tree(root)[0] if it.kind == "program"]
                lines = _double_claims(root, progs)
                for line in lines:
                    print(f"overlap: {line}")
                print(f"work.py territory --overlaps: {len(lines)} unrecorded pair(s) over {len(progs)} open programs")
                return 0
            files = None
            if args.files is not None:
                files = (sys.stdin.read() if args.files == "-" else open(args.files, encoding="utf-8").read()).split("\n")
            lines, mine = territory(root, args.base, args.branch, files)
            for line in lines:
                print(f"territory: {line}")
                if os.environ.get("GITHUB_ACTIONS"):
                    print(f"::warning file={line.split(':', 1)[0]}::{line}")
            print(f"work.py territory: branch program {mine or '(none)'}, {len(lines)} path(s) in another program's territory")
            return 1 if (lines and args.strict) else 0
        if args.cmd == "incoming":
            prog, n, text = incoming(root, args.base, args.program, None)
            if text:
                print(text, end="" if text.endswith("\n") else "\n")
            print(f"work.py incoming: {n} commit(s) on {args.base} touch {WORK}/{prog}/ that this branch lacks")
            return 0
        raise AssertionError(args.cmd)
    except Bail as e:
        print(f"work.py: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
