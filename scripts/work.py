#!/usr/bin/env python3
"""The tracker's one tool. `work/README.md` is the contract this script
enforces; this header says only what the contract does not.

    work.py lint                         every rule in work/README.md
    work.py status [--program P]         the render, to stdout
    work.py render [--out PATH]          the render, to work/STATUS.md
    work.py new ID --kind K --title T [--program P] [--set key=value ...]
    work.py set ID key=value [key=value ...]
    work.py territory (--base REF | --files LIST) [--branch NAME] [--strict]
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
on GitHub and are not resolved — the tracker is version control and
does not call out. A `[ev]` PR title is likewise a GitHub fact; lint
checks that `needs_ev` is a bare `true`, not that an open PR carries
the flag (the PR is one `git log` away from the item). Territory globs are matched with
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
FREE_FILES = {"README.md", "STATUS.md"}       # top level of work/, unparsed
NARRATIVE = {"plan.md", "log.md", "process-observations.md"}   # inside a program, unparsed
LOG_EXEMPT = {"docs/MODEL-AB-LOG.md"}         # the one non-program log in docs/
STALE_DAYS = 14

KINDS = ("program", "unit", "issue", "ruling")
ITEM_STATUS = ("open", "spec", "dispatched", "review", "closed", "parked")
RULING_STATUS = ("open", "closed")
PROGRAM_STATUS = ("open", "closed")
AREAS = ("kernel", "api", "gui", "infra")

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
    "github": ("int", ("unit", "issue", "ruling")),
    "area": ("enum:area", ("program",)),
    "prefix": ("str", ("program",)),
    "tag": ("str", ("program",)),
    "ab_band": ("str", ("program",)),
    "paths": ("strlist", ("program",)),
    "keep_out": ("strlist", ("program",)),
    "blocks": ("strlist", ("program",)),
}
REQUIRED = ("id", "kind", "title", "status", "opened")

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
    if s == "" or s != s.strip() or s[0] in "[&*|>{'\"" or looks_typed:
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
    if typ == "enum:status":
        vocab = PROGRAM_STATUS if item.kind == "program" else RULING_STATUS if item.kind == "ruling" else ITEM_STATUS
        return [] if value in vocab else [f"{w} must be one of {vocab} for kind {item.kind}"]
    raise AssertionError(typ)


def lint(root: str) -> list[str]:
    items, errors = load_tree(root)
    by_id: dict[str, Item] = {}
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
    programs = {it.id: it for it in items if it.kind == "program" and it.id}
    tracked: list[str] | None = None
    for it in items:
        # status-coupled fields
        if it.status == "closed" and it.get("closed") is None:
            errors.append(f"{it.path}: status closed needs a `closed:` date")
        if it.status != "closed" and it.get("closed") is not None:
            errors.append(f"{it.path}: `closed:` is set but status is {it.status}")
        if it.status == "parked" and not it.get("blocked_on"):
            errors.append(f"{it.path}: parked needs a non-empty `blocked_on`")
        # references
        for key in ("parent", "rides_with"):
            v = it.get(key)
            if isinstance(v, str) and v not in by_id:
                errors.append(f"{it.path}: `{key}` names `{v}`, which is no item")
        for key in ("blocked_on", "refs"):
            for v in it.get(key) or []:
                if isinstance(v, str) and v not in by_id:
                    errors.append(f"{it.path}: `{key}` names `{v}`, which is no item")
        carrier = it.get("rides_with")
        if isinstance(carrier, str) and carrier in by_id and by_id[carrier].status == "closed" and it.status != "closed":
            errors.append(f"{it.path}: rides with `{carrier}`, which is closed — re-home it (a struck row may not delete its passengers)")
        if it.kind == "program":
            if it.status == "closed":
                live = [o.id for o in items if o.program == it.id and o.kind != "program" and o.status != "closed"]
                if live:
                    errors.append(f"{it.path}: program is closed but {live} are not")
            prefix = it.get("prefix")
            if isinstance(prefix, str) and not prefix.endswith("/"):
                errors.append(f"{it.path}: `prefix` must end in `/`")
            for g in it.get("paths") or []:
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
    out.append("| area | program | status | open | spec | dispatched | review | parked | closed | on Ev |")
    out.append("|---|---|---|---|---|---|---|---|---|---|")
    for p in programs:
        rows = by_program.get(p.id, [])
        c = {s: sum(1 for r in rows if r.status == s) for s in ITEM_STATUS}
        ev = sum(1 for r in rows if r.status != "closed" and r.get("needs_ev") is not None)
        out.append(f"| {p.get('area') or '—'} | `{p.id}` | {p.status} | {c['open']} | {c['spec']} | "
                   f"{c['dispatched']} | {c['review']} | {c['parked']} | {c['closed']} | {ev or ''} |")
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
        if p.status == "closed":
            out.append("Closed; every item is closed.")
            out.append("")
            continue
        if not rows:
            out.append("No open items.")
            out.append("")
            continue
        out.append("| item | kind | status | title | blocked on | PR |")
        out.append("|---|---|---|---|---|---|")
        for r in sorted(rows, key=lambda i: (ITEM_STATUS.index(i.status) if i.status in ITEM_STATUS else 9, i.id)):
            blocked = ", ".join(_fmt_ref(b) for b in (r.get("blocked_on") or []))
            pr = f"#{r.get('pr')}" if r.get("pr") is not None else ""
            ev = " **[ev]**" if r.get("needs_ev") is not None else ""
            out.append(f"| `{r.id}` | {r.kind} | {r.status}{ev} | {r.get('title')} | {blocked} | {pr} |")
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
                       f"{', '.join(_fmt_ref(b) for b in it.get('blocked_on') or [])} |")
    else:
        out.append("Nothing.")
    out.append("")

    # stale
    stale = []
    for it in live:
        if it.status == "parked":
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
    with open(os.path.join(root, it.path), "w", encoding="utf-8") as f:
        f.write(format_front_matter(it.fields, it.order))
        f.write(it.body)
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
        r = subprocess.run(["git", "-C", root, "rev-parse", "--abbrev-ref", "HEAD"], capture_output=True, check=False)
        branch = r.stdout.decode().strip() if r.returncode == 0 else ""
    mine = None
    best = -1
    for p in programs:
        pre = p.get("prefix")
        if isinstance(pre, str) and branch.startswith(pre) and len(pre) > best:
            mine, best = p.id, len(pre)
    if files is None:
        if base is None:
            raise Bail("territory needs --base or --files")
        r = subprocess.run(["git", "-C", root, "diff", "--name-only", f"{base}...HEAD"], capture_output=True, check=False)
        if r.returncode != 0:
            raise Bail(f"git diff against {base} failed: {r.stderr.decode(errors='replace').strip()}")
        files = r.stdout.decode().split("\n")
    lines = []
    for path in sorted(p.strip() for p in files if p.strip()):
        owners = sorted(p.id for p in programs if p.status != "closed"
                        and any(fnmatch.fnmatchcase(path, g) for g in p.get("paths") or [] if isinstance(g, str)))
        if owners and mine not in owners:
            lines.append(f"{path}: owned by {', '.join(owners)}" + (f"; this branch is {mine}'s" if mine else "; this branch has no program prefix"))
    return lines, mine


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
           "---\nid: mesh\nkind: program\ntitle: S-MESH\nstatus: open\nopened: 2026-08-31\n"
           "area: kernel\nprefix: mesh/\nab_band: 1200-1299\npaths: [crates/mesh/*]\n---\n")
    _write(root, "work/mesh/plan.md", "plan\n")
    _write(root, "work/mesh/log.md", "log\n")
    _write(root, "work/mesh/MESH-1.md",
           "---\nid: MESH-1\nkind: unit\ntitle: first\nstatus: review\nopened: 2026-09-01\npr: 1605\n"
           "blocked_on: [MESH-2, 1601]\n---\n\nbody\n")
    _write(root, "work/mesh/MESH-2.md",
           "---\nid: MESH-2\nkind: issue\ntitle: second\nstatus: open\nopened: 2026-09-01\nneeds_ev: true\n---\n")
    _write(root, "work/topo/program.md",
           "---\nid: topo\nkind: program\ntitle: closed one\nstatus: closed\nopened: 2026-08-01\n"
           "closed: 2026-08-20\narea: kernel\nprefix: topo/\npaths: [crates/topo/*]\n---\n")
    _write(root, "work/topo/T-1.md",
           "---\nid: T-1\nkind: unit\ntitle: done\nstatus: closed\nopened: 2026-08-01\nclosed: 2026-08-19\n---\n")
    _write(root, "work/issues/stray-thing.md",
           "---\nid: stray-thing\nkind: issue\ntitle: unowned\nstatus: open\nopened: 2026-09-02\n---\n")
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

    with tempfile.TemporaryDirectory() as root:
        _fixture(root)
        expect("clean fixture", lint(root))
        text = render(root, today=dt.date(2026, 9, 20))
        for needle in ("## Waiting on Ev", "`MESH-2`", "`stray-thing`", "## Blocked", "MESH-2, #1601", "`topo`"):
            if needle not in text:
                failures.append(f"render lacks {needle!r}")
        if "MESH-1" not in text.split("## Untouched")[1]:
            failures.append("render: fixture items committed 'today' should still be listed stale at +19 days")
        p = render(root, only_program="mesh", today=dt.date(2026, 9, 20))
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

        # one mutation per rule, each restored
        cases: list[tuple[str, str, str, str]] = [
            ("unknown key", "work/mesh/MESH-2.md", "needs_ev: true", "colour: red"),
            ("needs_ev is a bare true", "work/mesh/MESH-2.md", "needs_ev: true", "needs_ev: 1700"),
            ("dangling ref", "work/mesh/MESH-1.md", "blocked_on: [MESH-2, 1601]", "blocked_on: [MESH-9]"),
            ("id vs file name", "work/mesh/MESH-1.md", "id: MESH-1", "id: MESH-7"),
            ("bad status", "work/mesh/MESH-1.md", "status: review", "status: landed"),
            ("closed needs date", "work/mesh/MESH-1.md", "status: review", "status: closed"),
            ("parked needs blocker", "work/mesh/MESH-2.md", "status: open", "status: parked"),
            ("closed program with live item", "work/topo/T-1.md", "status: closed\nopened: 2026-08-01\nclosed: 2026-08-19", "status: open\nopened: 2026-08-01"),
            ("glob matches nothing", "work/mesh/program.md", "paths: [crates/mesh/*]", "paths: [crates/nope/*]"),
            ("prefix shape", "work/mesh/program.md", "prefix: mesh/", "prefix: mesh"),
            ("program field on a unit", "work/mesh/MESH-1.md", "pr: 1605", "prefix: x/"),
            ("nested yaml refused", "work/mesh/MESH-1.md", "pr: 1605", "pr:\n  - 1605"),
            ("unit outside a program", "work/issues/stray-thing.md", "kind: issue", "kind: unit"),
        ]
        expectations = ["unknown key", "either `true` or absent", "no item", "must equal the file name", "must be one of",
                        "needs a `closed:` date", "non-empty `blocked_on`", "program is closed but",
                        "matches no tracked path", "must end in `/`", "not a field of kind unit",
                        "indented line", "only kind issue lives under"]
        for (name, rel, old, new), needle in zip(cases, expectations, strict=True):
            p = os.path.join(root, rel)
            with open(p, encoding="utf-8") as f:
                original = f.read()
            if old not in original:
                failures.append(f"{name}: fixture lacks {old!r}")
                continue
            _write(root, rel, original.replace(old, new))
            expect(name, lint(root), needle)
            _write(root, rel, original)
        expect("restored fixture", lint(root))

        # rides-along on a closed carrier
        _write(root, "work/topo/T-2.md",
               "---\nid: T-2\nkind: issue\ntitle: passenger\nstatus: open\nopened: 2026-08-02\nrides_with: T-1\n---\n")
        expect("passenger on struck row", lint(root), "re-home it", "program is closed but")
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
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "mesh/y")
        if mine != "mesh" or lines:
            failures.append(f"territory (own program): {mine} {lines}")
        lines, mine = territory(root, "master" if _branch_exists(root, "master") else "main", "verbs/z")
        if mine is not None or len(lines) != 1 or "crates/mesh/src/other.rs" not in lines[0]:
            failures.append(f"territory (foreign branch, closed program ignored): {mine} {lines}")

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
    args = ap.parse_args(argv)

    try:
        if args.selftest:
            return selftest()
        if not args.cmd:
            ap.print_help()
            return 2
        root = args.root or _repo_root()
        if args.cmd == "lint":
            errors = lint(root)
            for e in errors:
                print(e)
            print(f"work.py lint: {'FAIL' if errors else 'ok'} ({len(errors)} problem{'s' if len(errors) != 1 else ''})")
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
        raise AssertionError(args.cmd)
    except Bail as e:
        print(f"work.py: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
