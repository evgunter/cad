#!/usr/bin/env python3
"""The two halves of CI run the same checks, the gates that must fire on
docs-tier inputs are SITED where they can, and the prune exception stays one job.

WHY THIS IS PYTHON AND NOT A `scripts/gates/*.sh` MEMBER. Its subject is the
STRUCTURE of `.github/workflows/*.yml` — which job a step belongs to, whether
that job carries an `if:`, whether a prune step precedes a step that reads the
pruned tree. The first version of this check was bash, and read that structure
with `awk '/^  [a-z0-9_-]+:$/'`. A reviewer planted a job named `buildXtra:`
with a checkout and a read of `local-scripts/`: the matcher did not see an
uppercase letter as a job start, folded the steps into the PREVIOUS job, and the
gate exited 0. The claim was stated as total and was approximately total, which
is the failure mode this whole track is about. `scripts/gates/`'s own roster
argument (see `gate-roster.sh`) is that the directory means `lib.sh`'s two-mode
bash contract, and that a python check either reimplements it or meets none of
it — so this lives in `scripts/` beside its sibling
`scripts/check-interval-cfg-additive.py`, is named by hand in BOTH halves like
every other check out here, and is covered by its own claim 1.

Stdlib only, no YAML library: the runner image is not asked for one, matching
the posture of every other cheap tripwire here. What replaces a parser is a
RECOGNISER THAT ENUMERATES, and the claim it supports is narrow enough to be
worth stating exactly, because it has twice been written wider than it held:

  Inside a workflow's `jobs:` block, every line must match one of the forms
  listed at `JOB_KEYS` / `STEP_KEYS` and the shapes around them; anything else
  raises `Bail`, which fails the check.

Four things that claim does NOT cover, none of them hidden:
  * Content OUTSIDE `jobs:` is not read at all. A file with no top-level
    `jobs:` key Bails rather than being half-read.
  * The body of a block scalar, of `with:` and of `env:` is opaque text. It is
    scanned for invocations; it is not parsed, and nothing in it can Bail.
  * A recognised key's VALUE is only interpreted where a claim needs it —
    `if:`, `needs:`, `continue-on-error:`, `uses:`. Every other value is text.
  * This is not YAML. Anchors, merge keys, flow mappings and multi-document
    files are all refused, not supported. Valid YAML this repo does not use
    reds CI with *I do not understand this file*, and the fix is to teach the
    recogniser in a diff someone reviews.

WHAT IT CANNOT SEE, stated because a disclosed blind spot is a work order.
Claims 1-4 are about PATHS, so a hosted row that runs work inline — `cargo` in a
`run:` block, with no `scripts/` or `demos/` path to match — is outside them.
Claim 9 is what covers that population: it is about JOBS rather than paths, and
it is deliberately coarse — one citation per hosted job, not per row inside it,
because a hosted job's steps and a local row's function body are not the same
partition of the work and pretending they are would be a checker of a
coincidence. What it proves is that no hosted JOB is unaccounted for locally.
It does NOT prove the two run the same commands, and it does not prove the
local row a citation sits above still exists — a marker is a comment, and
deleting the row under it leaves the citation resolving perfectly well against
the hosted step it names. Claim 6 reads
`.github/workflows/*.yml` and nothing else that can trigger a checkout, so a
composite action under `.github/actions/` is outside it. And, as everywhere,
wiring is not execution: a step disabled by an `if:` on the STEP still satisfies
claims 7 and 8 — only job-level `if:` is read.

  check-ci-mirror-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile

HOSTED_HALF = ".github/workflows/ci.yml"
LOCAL_HALF = "local-scripts/ci-local.sh"
WORKFLOW_DIR = ".github/workflows"

# The one job allowed to keep `local-scripts/` after checkout, because the
# agreement between the halves is what it checks.
SITING_JOB = "mirror"

# THE RULE THIS FILE ENFORCES (Evan, 2026-08-20, on S61): *a gate must be sited
# where it can fire on its own inputs.* Each entry is an invocation whose inputs
# are prose, documentation or `local-scripts/` — file classes that make a change
# set TIER=docs, on which every `if: run_build` job is skipped. Each must be
# invoked from a ci.yml job carrying NO `if:`, and from the local half BEFORE
# its docs-tier early exit.
#
# Without this claim the rule is prose in three script headers: hollowing the
# `mirror` job and moving its three steps back into `discipline` restores the
# exact state S61 recorded, and nothing fires.
TIER_BLIND = (
    "scripts/gates/gate-roster.sh",
    "scripts/gates/probe-suite-census.sh --citations",
    "scripts/check-ci-mirror-parity.py",
    # Not because its inputs are prose — because a change WIDENING the
    # filter's docs branch classifies itself as docs, so the tier that would
    # skip this self-test is the tier it is about.
    "scripts/ci-filter.py --selftest",
)

# Declared asymmetries in claim 1. `path: (half, reason)`. An entry is a
# confession, not a disposition: it says a check runs in one half only.
MIRROR_EXEMPT = {
    "demos/render-uv.sh": (
        "local",
        "the committed UV sheet drift row. Its hosted mirror was retired "
        "2026-08-17 when render.yml's uv lane started re-baselining itself; "
        "ci-local.sh says so at the row",
    ),
}

# Declared asymmetries in claim 2 (gate MODES). Empty, and that is the point:
# every flagged gate invocation is spelled in both halves today.
GATE_MODE_EXEMPT: dict[str, tuple[str, str]] = {}

# Claim 8's floor. A hand-maintained number inside a gate whose thesis is that
# hand-maintained rosters drift — kept deliberately, for the reason
# `probe-suite-census.sh`'s CENSUS_FLOOR is kept: what a floor pins is that the
# population cannot silently SHRINK, and there is nothing to derive it from. A
# marker is a sentence someone chose to write; no file lists which sentences
# ought to exist. Lowering it is a decision, and reads as one in a diff.
MIRROR_MARKER_FLOOR = 32

SCRIPT_RE = re.compile(r"(?:^|[^A-Za-z0-9_/.-])((?:scripts|demos)/[A-Za-z0-9_/.-]+\.(?:sh|py))")
COMMENT_RE = re.compile(r"^\s*#")
MARKER_RE = re.compile(r"#\s*HOSTED MIRROR:\s*(.*?)\s*$")
# Claim 9's confession, written in ci.yml AT THE JOB it excuses. Not a list in
# this file: a central exception table is a place to put things, and nobody
# reading the job would see that it is on it.
NO_MIRROR_RE = re.compile(r"#\s*NO LOCAL MIRROR:\s*(.*?)\s*$")


class Bail(Exception):
    """Structure this reader does not RECOGNISE. Never a pass."""

# WHERE TO GO WHEN THIS REFUSES YOUR FILE. A `Bail` that only says *no* is what
# makes the next person reach for an exemption entry instead of a fix, so every
# one of them names the file, the line, the text that was not recognised, and
# the symbol to extend. Growing the recogniser is the intended response and is
# a small diff someone reviews; that is the whole trade this strictness rests
# on.
SELF = "scripts/check-ci-mirror-parity.py"

# Some refusals have no shape to learn — a tab, a non-UTF-8 byte, a file that is
# not there. They still owe the reader an action, and they say this so the
# self-test can tell "no guidance needed" from "guidance forgotten".
NO_TEACH = "There is nothing to extend here"
# The same sentence as a suffix, for a message that ends in a full stop.
NO_TEACH_TAIL = " " + NO_TEACH + "."


def teach(where: str) -> str:
    return (f" To accept this shape, extend {where} in {SELF} — that is the intended fix here, and it "
            "is a small reviewable diff. Do NOT route around it with an exemption entry: exemptions in "
            "this file are for asymmetries that exist, not for input the reader cannot read.")



# THE RECOGNISER ENUMERATES; THE READER NEVER SKIPS.
#
# Two versions of this check have now failed the same way. The first was
# `awk '/^  [a-z0-9_-]+:$/'` and let an uppercase job name through, because an
# unmatched line meant "not a job". The second was a hand-rolled python reader
# and let a FLUSH-STYLE step sequence through — `    - uses: …` at the same
# indent as `steps:` is ordinary YAML, its first token parsed as a key named
# `- uses`, and an unrecognised key meant "nothing here". Same default, more
# structure: *I did not recognise this* and *there is nothing here* were one
# value, which is the exact defect this track's F6 row spent two instruments on
# one directory over.
#
# So the default is inverted. Every line inside `jobs:` must match one of the
# forms enumerated BELOW, and anything else raises `Bail`, which is a hard
# failure. The cost is real and is the point: a workflow written in a shape
# this repo has not used before reds CI with *I do not understand this file*,
# and the fix is to teach the recogniser, deliberately, in a diff someone
# reviews. The alternative is a checker that keeps saying OK about YAML it has
# never seen.
#
# It is NOT a YAML parser and does not try to be. It recognises the subset
# these two workflows are written in, refuses everything else, and reads the
# structural facts three claims need: which job a step belongs to, whether that
# job can be skipped, and whether a prune step precedes a step that reads the
# pruned tree.
JOB_KEYS = frozenset({
    "name", "runs-on", "needs", "if", "uses", "with", "secrets", "env",
    "permissions", "strategy", "outputs", "steps", "timeout-minutes",
    "continue-on-error", "defaults", "concurrency", "services", "container",
})
STEP_KEYS = frozenset({
    "name", "id", "uses", "with", "run", "env", "if", "shell",
    "working-directory", "continue-on-error", "timeout-minutes",
})
BLOCK_SCALAR_RE = re.compile(r"^[|>][+-]?\d*$")
JOB_NAME_RE = re.compile(r"([A-Za-z0-9_-]+):$")
KEY_RE = re.compile(r"([A-Za-z][A-Za-z0-9_-]*):(?:\s+(.*))?$")


class Step:
    def __init__(self) -> None:
        self.name: str | None = None
        self.lines: list[str] = []


class Job:
    def __init__(self, name: str, line: int) -> None:
        self.name = name
        self.line = line
        self.has_if = False
        self.continue_on_error = False
        self.needs: list[str] = []
        self.uses: str | None = None
        self.steps: list[Step] = []


def _significant(path: str) -> list[tuple[int, int, str]]:
    """`(lineno, indent, text)` for every line that carries structure.

    Tabs are refused outright rather than measured: YAML forbids them for
    indentation, and a tab used to make this reader silently lose the rest of
    the file.
    """
    try:
        with open(path, encoding="utf-8") as fh:
            raw = fh.read()
    except UnicodeDecodeError as exc:
        raise Bail(f"{path} is not UTF-8 ({exc}) — this reader will not guess at the bytes. "
                   f"Re-encode the workflow as UTF-8. {NO_TEACH}: the bytes are the problem, not the "
                   "shape") from exc
    if "\t" in raw:
        n = raw[: raw.index("\t")].count("\n") + 1
        raise Bail(f"{path}:{n}: a TAB character. YAML forbids tab indentation and this reader will "
                   "not guess what it was meant to be — re-indent that line with spaces. "
                   f"{NO_TEACH}; GitHub rejects a tab-indented workflow too")
    out = []
    for n, line in enumerate(raw.splitlines(), 1):
        if not line.strip() or COMMENT_RE.match(line):
            continue
        out.append((n, len(line) - len(line.lstrip(" ")), line))
    return out


def read_workflow(path: str) -> list[Job]:
    items = _significant(path)
    start = next((i for i, (_, ind, t) in enumerate(items) if ind == 0 and t.rstrip() == "jobs:"), None)
    if start is None:
        raise Bail(f"{path}: no top-level `jobs:` key at column 0. This reader recognises GitHub "
                   "workflow files and refuses to guess about anything else under "
                   f"{WORKFLOW_DIR}/. If this file belongs here and spells `jobs:` another way,"
                   + teach("`read_workflow`'s search for the `jobs:` key"))
    end = next((i for i in range(start + 1, len(items)) if items[i][1] == 0), len(items))
    block = items[start + 1:end]
    if not block:
        raise Bail(f"{path}:{items[start][0]}: `jobs:` has no body — every line after it is at column "
                   "0. A workflow with no jobs is not something this check can say anything about, and "
                   "reporting OK about it would be the silence this file exists to remove. Give the "
                   f"file jobs, or take it out of {WORKFLOW_DIR}/. {NO_TEACH}: the shape was understood, "
                   "there was simply nothing in it")

    job_indent = block[0][1]
    jobs: list[Job] = []
    i = 0
    while i < len(block):
        n, ind, text = block[i]
        if ind != job_indent:
            raise Bail(f"{path}:{n}: expected a job name at indent {job_indent} (the indent the first "
                       f"job in this file uses), found indent {ind}: {text.strip()!r}. Re-indent it to "
                       f"match, or if mixed job indents are meant to be legal here,"
                       + teach("`read_workflow`'s `job_indent` rule"))
        m = JOB_NAME_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a job name at job indent: {text.strip()!r}. A job key must be "
                       f"a bare `name:` on its own line (`JOB_NAME_RE`)."
                       + teach("`JOB_NAME_RE`"))
        job = Job(m.group(1), n)
        jobs.append(job)
        j = i + 1
        while j < len(block) and block[j][1] > job_indent:
            j += 1
        _read_job(path, job, block[i + 1:j])
        i = j
    if not jobs:
        raise Bail(f"{path}: `jobs:` parsed to no jobs at all — the reader scanned nothing, which is "
                   f"not a pass." + teach("`read_workflow`"))
    return jobs


def _read_job(path: str, job: Job, body: list[tuple[int, int, str]]) -> None:
    if not body:
        raise Bail(f"{path}:{job.line}: job `{job.name}` has an empty body. A job with no keys cannot "
                   f"be checked for a prune step or an `if:`." + teach("`_read_job`"))
    key_indent = body[0][1]
    i = 0
    while i < len(body):
        n, ind, text = body[i]
        if ind != key_indent:
            raise Bail(f"{path}:{n}: expected a key of job `{job.name}` at indent {key_indent} (the "
                       f"indent this job's first key uses), found indent {ind}: {text.strip()!r}. "
                       f"Re-indent it to match, or" + teach("`_read_job`'s `key_indent` rule"))
        m = KEY_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a `key:` line in job `{job.name}`: {text.strip()!r}. This is "
                       "where a YAML merge key (`<<: *anchor`) lands: anchors are refused, not "
                       f"supported." + teach("`KEY_RE` and `_read_job`"))
        key, value = m.group(1), (m.group(2) or "").strip()
        if key not in JOB_KEYS:
            raise Bail(f"{path}:{n}: job `{job.name}` carries the key `{key}`, which this recogniser "
                       f"does not know. Add `{key}` to `JOB_KEYS` — and, if a claim here depends on what "
                       "it means (the way `if:`, `needs:` and `continue-on-error:` decide whether a job "
                       f"can be skipped), read its value in `_read_job` at the same time."
                       + teach("`JOB_KEYS`"))
        # The key's own nested block: everything more indented, plus — for a
        # sequence — the flush-style `- ` items at the key's own indent.
        j = i + 1
        while j < len(body) and (body[j][1] > key_indent
                                 or (body[j][1] == key_indent and body[j][2].lstrip().startswith("- "))):
            j += 1
        nested = body[i + 1:j]
        if key == "if":
            job.has_if = True
        elif key == "continue-on-error":
            job.continue_on_error = value.lower() in ("true", "'true'", '"true"')
        elif key == "uses":
            job.uses = value
        elif key == "needs":
            job.needs = _read_needs(path, n, value, nested)
        elif key == "steps":
            _read_steps(path, job, key_indent, nested)
        i = j


def _read_needs(path: str, n: int, value: str, nested: list[tuple[int, int, str]]) -> list[str]:
    """`needs: a`, `needs: [a, b]`, and the block-sequence spelling. A job whose
    dependency is skipped is skipped, so claim 7 has to read this."""
    if value.startswith("["):
        if not value.endswith("]"):
            raise Bail(f"{path}:{n}: `needs:` flow sequence is not closed on one line: {value!r}. "
                       f"Claim 7 has to know what this job waits on and will not guess."
                       + teach("`_read_needs`"))
        return [p.strip().strip("'\"") for p in value[1:-1].split(",") if p.strip()]
    if value:
        return [value.strip("'\"")]
    out = []
    for ln, _, text in nested:
        t = text.strip()
        if not t.startswith("- "):
            raise Bail(f"{path}:{ln}: expected a `- job` item under `needs:`, found {t!r}."
                       + teach("`_read_needs`"))
        out.append(t[2:].strip().strip("'\""))
    return out


def _read_steps(path: str, job: Job, key_indent: int, nested: list[tuple[int, int, str]]) -> None:
    """Both indent styles for the sequence: items flush with `steps:` and items
    indented under it. Flush style is ordinary YAML, and reading it as a key
    named `- uses` is how a checked-out job that read `local-scripts/` came back
    OK from the version this replaced."""
    if not nested:
        raise Bail(f"{path}:{job.line}: job `{job.name}` has `steps:` with no steps under it."
                   + teach("`_read_steps`"))
    item_indent = nested[0][1]
    if item_indent not in (key_indent, key_indent + 2):
        raise Bail(f"{path}:{nested[0][0]}: `steps:` items of job `{job.name}` sit at indent "
                   f"{item_indent}, which is neither flush with `steps:` ({key_indent}) nor the usual "
                   f"one in ({key_indent + 2}). Both of those are recognised;"
                   + teach("`_read_steps`'s `item_indent` rule"))
    i = 0
    while i < len(nested):
        ln, ind, text = nested[i]
        if ind != item_indent or not text.lstrip().startswith("- "):
            raise Bail(f"{path}:{ln}: expected a `- ` step item at indent {item_indent} in job "
                       f"`{job.name}`, found {text.strip()!r}." + teach("`_read_steps`"))
        j = i + 1
        while j < len(nested) and nested[j][1] > item_indent:
            j += 1
        step = Step()
        job.steps.append(step)
        _read_step(path, job, step, item_indent, nested[i:j])
        i = j


def _read_step(path: str, job: Job, step: Step, item_indent: int,
               item: list[tuple[int, int, str]]) -> None:
    inner = item_indent + 2
    # The `- ` line carries the item's first mapping key at column `inner`.
    first_n, _, first_text = item[0]
    rows = [(first_n, inner, " " * inner + first_text.lstrip()[2:])] + list(item[1:])
    k = 0
    while k < len(rows):
        n, ind, text = rows[k]
        step.lines.append(text)
        if ind != inner:
            raise Bail(f"{path}:{n}: expected a key of a step in job `{job.name}` at indent {inner} "
                       f"(the column just after `- `), found indent {ind}: {text.strip()!r}."
                       + teach("`_read_step`"))
        m = KEY_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a `key:` line inside a step of job `{job.name}`: "
                       f"{text.strip()!r}." + teach("`KEY_RE` and `_read_step`"))
        key, value = m.group(1), (m.group(2) or "").strip()
        if key not in STEP_KEYS:
            raise Bail(f"{path}:{n}: a step of job `{job.name}` carries the key `{key}`, which this "
                       f"recogniser does not know. Add `{key}` to `STEP_KEYS`, and if its body is a "
                       "nested block rather than a scalar, list it beside `with:` and `env:` in "
                       f"`_read_step`." + teach("`STEP_KEYS`"))
        if key == "name":
            step.name = value.strip("'\"")
        # A block scalar's body is opaque text, not keys — `run: |` is where
        # every invocation this file reads actually lives.
        j = k + 1
        while j < len(rows) and rows[j][1] > inner:
            if BLOCK_SCALAR_RE.fullmatch(value) or key in ("with", "env"):
                step.lines.append(rows[j][2])
                j += 1
                continue
            raise Bail(f"{path}:{rows[j][0]}: content nested under `{key}:` in a step of job "
                       f"`{job.name}`. Nested content is only expected under a block scalar (`|`, `>`), "
                       f"`with:` or `env:`; the nested text is scanned for invocations and never parsed."
                       + teach("`_read_step`'s list of keys that may carry a nested block"))
        k = j


def non_comment(path: str) -> list[str]:
    with open(path, encoding="utf-8") as fh:
        return [l for l in fh.read().splitlines() if not COMMENT_RE.match(l)]


def invocations(lines: list[str]) -> set[str]:
    """Every scripts/** or demos/** path named. The leading boundary in
    SCRIPT_RE keeps `local-scripts/ci-local.sh` from being read as an
    invocation of `scripts/ci-local.sh`."""
    return {m for l in lines for m in SCRIPT_RE.findall(l)}


def gate_modes(lines: list[str]) -> set[str]:
    """`scripts/gates/X.sh --flag` pairs, ignoring `--selftest` (every half runs
    that for every gate) and `--root` (a fixture argument, never a mode)."""
    out = set()
    for l in lines:
        for m in re.finditer(r"(scripts/gates/[A-Za-z0-9_-]+\.sh)\s+(--[a-z-]+)", l):
            if m.group(2) not in ("--selftest", "--root"):
                out.add(f"{m.group(1)} {m.group(2)}")
    return out


def reachable(root: str, seeds: set[str]) -> set[str]:
    """Transitive closure of scripts/** and demos/** paths named from a seed
    set, following file contents. This is what turns claim 4 from a one-hop
    guess into a total statement: `scripts/step_import_check.py` is named by no
    half and is not an orphan — `scripts/check_step.sh` runs it."""
    seen = set(seeds)
    stack = list(seeds)
    while stack:
        cur = stack.pop()
        full = os.path.join(root, cur)
        if not os.path.isfile(full):
            continue
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                body = [l for l in fh.read().splitlines() if not COMMENT_RE.match(l)]
        except OSError:
            continue
        # A script names its siblings by BASENAME as often as by path
        # (`demos/render.sh` runs `strip_png_stamps.py` from its own directory),
        # so both spellings count — resolved against the NAMING script's own
        # directory, which is what the shell does.
        #
        # THE FAILURE DIRECTION, because a heuristic without one is a guess.
        # This resolves a bare `foo.sh` to `<dir of the naming script>/foo.sh`
        # whenever that file exists, and a bare name can be a coincidence: a
        # comment, a string, or a sibling with the same basename as some other
        # directory's script. So the closure can only be TOO LARGE — it can
        # decide a script is owned when nothing really runs it. It can never
        # shrink: every path-shaped reference is still matched exactly.
        #
        # An over-large closure UNDER-reports claim 4 (an orphan reads as
        # owned) and affects nothing else — no other claim consumes it. That is
        # the safe direction here, and deliberately so: claim 4's job is to
        # catch a check nobody runs, and a false positive there would demand a
        # `MIRROR_EXEMPT` entry for a script that is in fact perfectly wired,
        # which teaches the next reader that the exemption list is where
        # inconvenient results go. Under-reporting costs a missed orphan;
        # over-reporting costs the credibility of every other claim in the
        # file. The claim's own wording is written to match: it says a check
        # that neither half names *and no named script reaches*.
        here = os.path.dirname(cur)
        named = invocations(body)
        for l in body:
            # `name.sh`, and `./name.sh` — the spelling a script uses to source
            # a sibling (`. ./hosted-render-guard.sh`), which a path-shaped
            # matcher does not see.
            for m in (re.findall(r"(?:^|[^A-Za-z0-9_/.-])([A-Za-z0-9_.-]+\.(?:sh|py))", l)
                      + re.findall(r"\./([A-Za-z0-9_.-]+\.(?:sh|py))", l)):
                cand = os.path.join(here, m)
                if os.path.isfile(os.path.join(root, cand)):
                    named.add(cand)
        for p in named:
            if p not in seen:
                seen.add(p)
                stack.append(p)
    return seen


def local_docs_exit_line(lines: list[str]) -> int:
    """The line index of the local half's docs-tier early exit. Claim 7 needs
    it: a row placed after it does not run on the tier it is about."""
    for i, l in enumerate(lines):
        if re.search(r'"\$TIER"\s*=\s*docs', l):
            for j in range(i, min(i + 12, len(lines))):
                if re.match(r'\s*exit (0|"?\$[A-Za-z_]\w*"?)\s*$', lines[j]):
                    return j
            raise Bail(f"{LOCAL_HALF}:{i + 1}: the docs-tier branch opens here and no `exit` follows "
                       f"it within 12 lines: {l.strip()!r}. Claim 7's whole question is which rows run "
                       "BEFORE that exit, so this reader will not guess where the branch ends. If the "
                       f"branch now ends another way," + teach("`local_docs_exit_line`"))
    raise Bail(f"{LOCAL_HALF}: no docs-tier branch found — nothing matches `\"$TIER\" = docs`. Claim 7 "
               "measures every tier-blind row against that branch, so without it the claim has no "
               f"reference point and would pass vacuously." + teach("`local_docs_exit_line`"))


# THE SHELL HALF'S RECOGNISER, on the same inverted default as the YAML one.
# `^name() {` was the only spelling read, so `tier_blind_rows () {` and
# `function tier_blind_rows {` — both ordinary bash — parsed as "not a function",
# every body line resolved as a top-level call site, and a definition left above
# `ci-local.sh`'s docs exit with its single call moved below it came back OK.
# That is the MAJOR this resolution was added to close, re-opened by one space.
# So the forms are enumerated, and a line that LOOKS like a definition and is
# not one of them Bails.
FUNC_HINT_RE = re.compile(r"^(?:function\s+[A-Za-z_]|[A-Za-z_][A-Za-z0-9_]*\s*\(\s*\))")
FUNC_OPEN_RE = re.compile(r"^(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\s*\))?\s*\{\s*$")
FUNC_ONELINE_RE = re.compile(r"^(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\s*\))?\s*\{.*\}\s*$")


def shell_functions(lines: list[str]) -> dict[str, tuple[int, int]]:
    """`name -> (first, last)` body line indices, inclusive; a one-liner is its
    own body. Bails on a definition-shaped line in a spelling not listed."""
    funcs: dict[str, tuple[int, int]] = {}
    open_at: tuple[str, int] | None = None
    for i, l in enumerate(lines):
        if open_at is not None:
            if re.fullmatch(r"\}\s*", l):
                funcs[open_at[0]] = (open_at[1], i)
                open_at = None
            continue
        if not FUNC_HINT_RE.match(l):
            continue
        m = FUNC_ONELINE_RE.fullmatch(l)
        if m:
            funcs[m.group(1)] = (i, i)
            continue
        m = FUNC_OPEN_RE.fullmatch(l)
        if not m:
            raise Bail(f"{LOCAL_HALF}:{i + 1}: looks like a shell function definition and is not a "
                       f"spelling this recogniser knows: {l.strip()!r}. Recognised today: `name() {{`, "
                       "`name () {`, `function name {`, and the one-line `name() { …; }`. Claim 7 "
                       "has to know where a row is RUN, not where it is written, and it will not guess."
                       + teach("`FUNC_OPEN_RE` / `FUNC_ONELINE_RE`, beside `FUNC_HINT_RE`"))
        open_at = (m.group(1), i)
    if open_at is not None:
        raise Bail(f"{LOCAL_HALF}:{open_at[1] + 1}: function `{open_at[0]}` is opened here and never "
                   "closed by a `}` at column 0, so this reader cannot tell which lines are its body "
                   f"and which run at top level." + teach("`shell_functions`'s close rule"))
    return funcs


def local_call_sites(lines: list[str], want: str) -> list[int]:
    """Line indices at which the local half actually RUNS `want`.

    Not "mentions": a row wrapped in a shell function is run where the FUNCTION
    is called, and a definition placed above the docs-tier exit says nothing
    about when it executes. So an occurrence inside a function body resolves to
    that function's call sites outside it. Without this the claim is satisfied
    by moving a definition, which is not the property.
    """
    funcs = shell_functions(lines)

    def enclosing(i: int) -> str | None:
        for name, (a, b) in funcs.items():
            if (a == b and i == a) or a < i < b:
                return name
        return None

    sites: list[int] = []
    for i, l in enumerate(lines):
        if want not in l:
            continue
        host = enclosing(i)
        if host is None:
            sites.append(i)
            continue
        a, b = funcs[host]
        sites += [j for j, m in enumerate(lines)
                  if re.search(rf"(^|[^A-Za-z0-9_-]){re.escape(host)}([^A-Za-z0-9_-]|$)", m)
                  and not (a <= j <= b) and enclosing(j) is None]
    return sorted(set(sites))


def unconditional(jobs: dict[str, "Job"], name: str, seen: frozenset[str] = frozenset()) -> str | None:
    """None if job `name` runs on every tier; otherwise the reason it may not.

    `if:` is not the only door. **GitHub skips a job whose `needs:` dependency
    skipped**, so a job with no condition of its own is still conditional if
    anything it waits on is — and adding a `needs:` for an output or for
    ordering is a far likelier accident than deleting a step. Claim 7 read only
    `has_if` until a verification pass restored S61's exact state by giving the
    siting job `needs: discipline`.
    """
    if name in seen:
        return f"`{name}` is part of a `needs:` cycle, which this reader will not reason about"
    job = jobs.get(name)
    if job is None:
        return f"`{name}` is named in a `needs:` and is not a job in this workflow"
    if job.has_if:
        return f"`{name}` carries an `if:`"
    if job.continue_on_error:
        return f"`{name}` carries `continue-on-error: true`, so it runs but cannot redden the PR"
    for dep in job.needs:
        why = unconditional(jobs, dep, seen | {name})
        if why is not None:
            return f"`{name}` waits on {why} — GitHub skips a job whose dependency skipped"
    return None


def check(root: str) -> list[str]:
    errs: list[str] = []

    def err(msg: str) -> None:
        errs.append(msg)

    os.chdir(root)
    for required in (HOSTED_HALF, LOCAL_HALF):
        if not os.path.isfile(required):
            raise Bail(
                f"{required} does not exist under {root} — half of this check's subject is missing. "
                "This runs in the one job that does NOT prune local-scripts/, so if a runner reports "
                "this, the prune has spread to that job: remove it from that job rather than making "
                f"this check tolerate the absence. {NO_TEACH}"
            )

    hosted_lines = non_comment(HOSTED_HALF)
    local_lines = non_comment(LOCAL_HALF)
    hosted = invocations(hosted_lines)
    local = invocations(local_lines)
    if not hosted or not local:
        raise Bail(f"{HOSTED_HALF if not hosted else LOCAL_HALF} names no scripts/ or demos/ path at "
                   "all. Every claim below is a comparison between two populations, and a comparison "
                   f"against an empty one passes for the wrong reason." + teach("`SCRIPT_RE`"))

    # CLAIM 1 — invocation parity outside scripts/gates/, both directions.
    for path in sorted((hosted | local) - {p for p in hosted | local if p.startswith("scripts/gates/")}):
        in_h, in_l = path in hosted, path in local
        if in_h and in_l:
            # AN EXEMPTION IS A CONFESSION WITH AN EXPIRY. Nothing used to
            # check that one was still needed, so an entry whose row came back
            # would have sat here forever, reading as a live asymmetry.
            if path in MIRROR_EXEMPT:
                want, reason = MIRROR_EXEMPT[path]
                err(f"{path} is declared {want}-only in MIRROR_EXEMPT and BOTH halves now name it. The "
                    f'reason ("{reason}") has expired — delete the entry, so the list stays a record of '
                    "asymmetries that exist rather than of ones that once did")
            continue
        side = "hosted" if in_h else "local"
        if path in MIRROR_EXEMPT:
            want, reason = MIRROR_EXEMPT[path]
            if want != side:
                err(f"{path} is declared {want}-only in MIRROR_EXEMPT but is invoked by the {side} half. "
                    f'The exemption now describes the opposite of the tree — re-read the reason ("{reason}") '
                    "and fix whichever side moved")
            continue
        other = LOCAL_HALF if in_h else HOSTED_HALF
        why = ("A check that runs only on a developer's machine gates nothing on merge"
               if side == "local" else
               "a row added to one side is invisible on the other until someone reads for it")
        err(f"{HOSTED_HALF if in_h else LOCAL_HALF} invokes {path} and {other} does not. Every check "
            f"outside scripts/gates/ is named by hand in both halves, so {why} — mirror it, or declare it "
            "in MIRROR_EXEMPT with the reason it is one-sided")

    for path, (want, reason) in sorted(MIRROR_EXEMPT.items()):
        if path not in hosted and path not in local:
            err(f"{path} is declared {want}-only in MIRROR_EXEMPT and NEITHER half names it. Either the "
                f'row is gone and the entry should go with it, or ("{reason}") is describing a check '
                "that stopped running anywhere")

    # CLAIM 2 — gate MODE parity. Claim 1 excludes scripts/gates/ because both
    # halves take that roster from the directory, but the directory says
    # nothing about the FLAGS a gate is run with, and a gate's flagged mode is
    # a different check. `--citations` was hosted-only when this claim was
    # written, and nothing could see it.
    h_modes, l_modes = gate_modes(hosted_lines), gate_modes(local_lines)
    for mode, (want, reason) in sorted(GATE_MODE_EXEMPT.items()):
        if mode not in (h_modes ^ l_modes):
            err(f"`{mode}` is declared {want}-only in GATE_MODE_EXEMPT and is no longer one-sided "
                f'("{reason}"). Delete the entry')
    for mode in sorted(h_modes ^ l_modes):
        side = "hosted" if mode in h_modes else "local"
        if mode in GATE_MODE_EXEMPT:
            want, reason = GATE_MODE_EXEMPT[mode]
            if want != side:
                err(f"`{mode}` is declared {want}-only in GATE_MODE_EXEMPT but is invoked by the {side} "
                    f'half ("{reason}")')
            continue
        err(f"`{mode}` is invoked by the {side} half only. A gate's flagged mode is a separate check: the "
            "directory loop in the local half runs every gate in DEFAULT mode and sees no flag, so a mode "
            "wired on one side only runs on one side only — mirror it, or declare it in GATE_MODE_EXEMPT")

    # CLAIM 3 — a mirrored path exists on disk. `gate-roster.sh` makes exactly
    # this check for its own directory; without it a typo present in both
    # halves is perfect parity over a file that is not there.
    for path in sorted(hosted | local):
        if not os.path.isfile(path):
            err(f"both halves name {path} and no such file exists — a renamed or deleted check leaves "
                "rows invoking a stale path in perfect agreement with each other")

    # CLAIM 4 — no orphan executables under scripts/ or demos/. The seed is
    # BOTH halves plus every workflow file: `render.yml` is reached through a
    # `uses:` job and runs the render entry points, so a script owned only by
    # it is owned, not orphaned.
    wf_seeds: set[str] = set()
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if wf.endswith((".yml", ".yaml")):
            wf_seeds |= invocations(non_comment(f"{WORKFLOW_DIR}/{wf}"))
    owned = reachable(root, hosted | local | wf_seeds)
    for d in ("scripts", "demos"):
        for name in sorted(os.listdir(d)):
            p = f"{d}/{name}"
            if not os.path.isfile(p) or not name.endswith((".sh", ".py")):
                continue
            if p.startswith("scripts/gates/") or p in owned:
                continue
            err(f"{p} is an executable check under {d}/ that NEITHER half names and no named script "
                "reaches. Outside scripts/gates/ there is no roster property at all, so a check can be "
                "written, committed and never run by anything — wire it into both halves, or move it")

    # CLAIM 5 — the `tools/` crates. They are checked by `cd tools/X && cargo …`
    # rows, which carry no scripts/ or demos/ path for claim 1 to match; this is
    # the smallest total statement about that directory.
    for name in sorted(os.listdir("tools")):
        if not os.path.isdir(f"tools/{name}"):
            continue
        in_h = any(f"tools/{name}" in l for l in hosted_lines)
        in_l = any(f"tools/{name}" in l for l in local_lines)
        if not (in_h and in_l):
            side = "the hosted half only" if in_h else ("the local half only" if in_l else "neither half")
            err(f"tools/{name} is a workspace-excluded crate named by {side}. Nothing else "
                "gates that tree — scripts/doc-gate.sh is `cargo doc --workspace` and cannot see it — so "
                "a tool checked on one side only is checked on one side only")

    # CLAIM 6 — the prune exception is EXACTLY ONE JOB, across every workflow
    # file. `render.yml` runs four checked-out lanes of its own through a
    # `uses:` job, and a hole there is the same hole. Both trees are checked,
    # and the ORDER is checked: a job that reads local-scripts/ and then
    # deletes it has read it.
    siting_job_seen = False
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if not wf.endswith((".yml", ".yaml")):
            continue
        path = f"{WORKFLOW_DIR}/{wf}"
        for job in read_workflow(path):
            steps = job.steps
            if job.uses is not None:
                # A reusable-workflow call has no steps of its own. It is not
                # exempt from this claim — the workflow it calls is read in
                # this same loop, which is how `render.yml`'s four checked-out
                # lanes are covered — but it must actually point in here.
                called = job.uses.removeprefix("./")
                if not called.startswith(f"{WORKFLOW_DIR}/") or not os.path.isfile(called):
                    err(f"{path} job `{job.name}` calls `{job.uses}`, which is not a workflow file in "
                        f"{WORKFLOW_DIR}/. This check reads that directory and nothing else, so a job "
                        "calling out of it runs steps nobody here has looked at")
                continue
            checkout = next((i for i, st in enumerate(steps)
                             if any("actions/checkout" in l for l in st.lines)), None)
            if checkout is None:
                continue
            reads = next((i for i, st in enumerate(steps)
                          if any(("local-scripts" in l or ".claude" in l) and "rm -rf" not in l
                                 for l in st.lines)), None)
            pruned = {t for i, st in enumerate(steps) for l in st.lines
                      if "rm -rf" in l and i > checkout
                      for t in ("local-scripts", ".claude") if t in l}
            first_prune = next((i for i, st in enumerate(steps)
                                if any("rm -rf" in l and "local-scripts" in l for l in st.lines)), None)
            if path == HOSTED_HALF and job.name == SITING_JOB:
                siting_job_seen = True
                if pruned:
                    err(f"job `{SITING_JOB}` prunes {' and '.join(sorted(pruned))}, but it is the one job "
                        "whose subject is the agreement between the halves — with the tree deleted it can "
                        "only check the hosted side, and the gates sited there pass for the wrong reason")
                continue
            missing = {"local-scripts", ".claude"} - pruned
            if missing:
                err(f"{path} job `{job.name}` checks the repo out and does not delete "
                    f"{' and '.join(sorted(missing))}. That deletion is what makes `scripts/ci-filter.py` "
                    "right to classify a change under either tree as non-triggering for the build rows; a "
                    "job that keeps them can couple the hosted gate to a developer's machine or an agent's "
                    "container without anything saying so")
            elif reads is not None and first_prune is not None and reads < first_prune:
                err(f"{path} job `{job.name}` reads local-only tooling at step {reads + 1} and prunes it at "
                    f"step {first_prune + 1}. The prune is only structural while it comes FIRST")
    if not siting_job_seen:
        err(f"{HOSTED_HALF} has no job `{SITING_JOB}` that checks the repo out. That job is where the "
            "checks whose inputs are docs, prose and local-scripts/ are sited; without it they are back in "
            "a job that skips on exactly the change class they are about")

    # CLAIM 7 — THE SITING RULE ITSELF. See TIER_BLIND's comment.
    ci_jobs = {j.name: j for j in read_workflow(HOSTED_HALF)}
    local_exit = local_docs_exit_line(non_comment(LOCAL_HALF))
    for want in TIER_BLIND:
        hosts = [j for j in ci_jobs.values() if any(want in l for st in j.steps for l in st.lines)]
        if not hosts:
            err(f"no ci.yml job runs `{want}`, which is one of the checks whose INPUTS are the docs tier. "
                "A check nobody runs cannot fire anywhere")
            continue
        reasons = {j.name: unconditional(ci_jobs, j.name) for j in hosts}
        if all(r is not None for r in reasons.values()):
            detail = "; ".join(f"{n}: {r}" for n, r in sorted(reasons.items()))
            err(f"`{want}` is run only by job(s) {', '.join(sorted(reasons))}, and not one of them runs "
                f"on every tier — {detail}. Its inputs are prose, documentation or local-scripts/, file "
                "classes that make a change set TIER=docs, on which every `if: run_build` job is skipped, "
                "so it cannot fire on the only change class that breaks it. That is the exact state S61 "
                "recorded. Site it in a job that no condition and no `needs:` chain can skip")
        # The local half is half the pair, and the same rule binds it.
        at = local_call_sites(non_comment(LOCAL_HALF), want)
        if not at:
            err(f"{LOCAL_HALF} never runs `{want}`. The rule is not hosted-side-only: a check the local "
                "half cannot run is a check a developer cannot run before pushing")
        elif min(at) > local_exit:
            err(f"{LOCAL_HALF} RUNS `{want}` only after its docs-tier `exit 0` — a definition above the "
                "exit is not a run — so on a docs-only or "
                "local-scripts-only change the local half runs it not at all — S61's defect, in the other "
                "half of the pair. Move the row above the early exit")

    # CLAIM 8 — mirror citations resolve, job AND step.
    steps = {(j.name, st.name) for j in ci_jobs.values() for st in j.steps if st.name}
    # Markers ARE comments, so they are read from the raw file.
    with open(LOCAL_HALF, encoding="utf-8") as fh:
        markers = sorted({m.group(1) for l in fh.read().splitlines()
                          for m in [MARKER_RE.search(l)] if m})
    for marker in markers:
        if " / " not in marker:
            err(f"{LOCAL_HALF} has a HOSTED MIRROR marker `{marker}` that is not `<job> / <step name>`. The "
                "job half is the part that drifted last time — the prose named the wrong job for a step "
                "that existed")
            continue
        job_name, step_name = marker.split(" / ", 1)
        if (job_name, step_name) not in steps:
            err(f"{LOCAL_HALF} cites a hosted mirror `{marker}`, and ci.yml has no step named "
                f"`{step_name}` in job `{job_name}`. Renaming a step or moving it between jobs leaves "
                "every sentence citing it quietly false")
    if len(markers) < MIRROR_MARKER_FLOOR:
        err(f"{LOCAL_HALF} carries {len(markers)} HOSTED MIRROR marker(s), below the "
            f"{MIRROR_MARKER_FLOOR} it had when this floor was set. Deleting a marker is how a citation "
            "check becomes vacuous; if a row genuinely lost its hosted mirror, lower the floor deliberately")

    # CLAIM 9 — JOB PARITY. Every job in ci.yml is either MIRRORED (some
    # HOSTED MIRROR marker in the local half names it, and claim 8 above has
    # already proved that citation resolves to a real step) or CONFESSED, by a
    # `# NO LOCAL MIRROR: <reason>` comment written at the job itself.
    #
    # WHY THE HOSTED SIDE IS ENUMERATED AND THE LOCAL SIDE DECLARES. The two
    # halves do not share a partition of the work — one hosted job can be
    # three local rows and one local row can span two hosted jobs — so any
    # rule that DERIVED the local list by a pattern and matched it against job
    # names would be checking a coincidence, which is the mistake both this
    # file's header and `gate-roster.sh`'s record having made. So the side
    # that can be enumerated exactly is enumerated (the recogniser above), and
    # the side that cannot is required to say, in writing, which hosted job
    # each of its rows answers to.
    #
    # WHY THE REASON LIVES IN ci.yml AND NOT IN A TABLE HERE. A list of
    # exceptions in this file is a place to put things, invisible from the job
    # it excuses. At the job, it is in the diff of anyone adding a job and in
    # front of anyone reading one. A confession with no reason is not a
    # confession, so an empty one is an error; and a job that is both cited
    # and excused has an expired confession, exactly as MIRROR_EXEMPT does.
    excused = _no_mirror_reasons(HOSTED_HALF, ci_jobs)
    cited_jobs = {m.split(" / ", 1)[0] for m in markers if " / " in m}
    for name in sorted(ci_jobs):
        reason = excused.get(name)
        if name in cited_jobs and reason is not None:
            err(f"ci.yml job `{name}` carries `NO LOCAL MIRROR` and {LOCAL_HALF} cites it anyway "
                f'("{reason}"). The confession has expired — delete it, so the comment stays a record '
                "of jobs that have no local half rather than of ones that once did")
            continue
        if name in cited_jobs:
            continue
        if reason is None:
            err(f"ci.yml job `{name}` has no local mirror: no `# HOSTED MIRROR: {name} / <step>` marker "
                f"in {LOCAL_HALF} names it, and the job carries no `# NO LOCAL MIRROR:` line. A hosted "
                "job with no local half is invisible to `local-scripts/gate.sh`, which documents itself "
                "as the merge gate when hosted Actions is unavailable — that is how `oracle-certify` "
                "went unmirrored. Mirror it and cite it, or write the reason it cannot be at the job")
        elif not reason:
            err(f"ci.yml job `{name}` carries a `# NO LOCAL MIRROR:` line with no reason after it. An "
                "exception with no reason is where the next one goes: say why this job has no local "
                "half, in the same line")

    return errs


def _no_mirror_reasons(path: str, jobs: dict[str, "Job"]) -> dict[str, str]:
    """`# NO LOCAL MIRROR: <reason>` lines in a workflow, by the job each
    excuses.

    ONE PLACEMENT RULE, AND IT IS STRICT: the line must sit in the contiguous
    comment block that runs down to a job's key. Anywhere else raises `Bail`.
    The looser reading — walk up from the comment to whichever job came before
    it — silently attributes a reason written BETWEEN two jobs to the one
    ABOVE, so a confession meant for the job you are looking at excuses a
    different job entirely and both look fine. A rule that can only be
    satisfied one way is a rule a reader can check by eye.
    """
    with open(path, encoding="utf-8") as fh:
        raw = fh.read().splitlines()
    at_line = {j.line: j.name for j in jobs.values()}
    out: dict[str, str] = {}
    for i, line in enumerate(raw, 1):
        m = NO_MIRROR_RE.search(line)
        if not m:
            continue
        j = i + 1
        while j <= len(raw) and raw[j - 1].strip().startswith("#"):
            j += 1
        if j not in at_line:
            raise Bail(f"{path}:{i}: a `NO LOCAL MIRROR` line that is not against a job key — the "
                       "first non-comment line below it is not the start of a job. Write it in the "
                       "comment block that runs down to the key of the job it excuses, with no blank "
                       "line between, so the job it names is the job it is read against."
                       + NO_TEACH_TAIL)
        out[at_line[j]] = m.group(1)
    return out


# ---------------------------------------------------------------- self-test
#
# THE FIXTURE IS A MINIATURE REPO. Every case runs the checker AS A SUBPROCESS,
# not as an in-process call: the bash gates' shared harness runs its subject
# inside `if out=$(…)`, where bash suppresses errexit, and that is exactly the
# condition under which a `set -e` script dies before printing its own error.
# A self-test that cannot reproduce the real invocation cannot see that.
def plant_clean(t: str) -> None:
    for d in ("scripts/gates", "demos", "local-scripts", ".github/workflows", "tools/toolcrate"):
        os.makedirs(os.path.join(t, d), exist_ok=True)
    open(os.path.join(t, "tools/toolcrate/Cargo.toml"), "w").close()
    names = [f"check-{i}.sh" for i in range(MIRROR_MARKER_FLOOR)]
    for n in names:
        open(os.path.join(t, "scripts", n), "w").close()
    # Derived from TIER_BLIND, not listed again: a second spelling of that
    # tuple is a second roster, and this file's whole subject is rosters that
    # drift from what they describe.
    for want in TIER_BLIND:
        open(os.path.join(t, want.split()[0]), "w").close()
    with open(os.path.join(t, HOSTED_HALF), "w") as fh:
        fh.write("jobs:\n")
        fh.write(f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n")
        for want in TIER_BLIND:
            fh.write(f"      - name: sited {want}\n        run: {want}\n")
        # A step the local half can cite without naming a TIER_BLIND path:
        # the citation marker must survive the plants that delete those rows.
        fh.write("      - name: sited rows\n        run: echo sited\n")
        fh.write("  discipline:\n    if: needs.filter.outputs.run_build == 'true'\n    steps:\n")
        fh.write("      - uses: actions/checkout@v4\n")
        fh.write("      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n")
        for i, n in enumerate(names):
            fh.write(f"      - name: mirrored step {i}\n        run: scripts/{n}\n")
        fh.write("      - name: tools\n        run: cd tools/toolcrate && cargo test\n")
        # A job with no local half, confessing at its own key — claim 9's
        # other branch, exercised by the CLEAN fixture so the passing shape is
        # covered as well as the failing ones.
        fh.write("  # NO LOCAL MIRROR: nothing is archived on one machine\n")
        fh.write("  archive:\n    steps:\n      - uses: actions/checkout@v4\n")
        fh.write("      - name: prune\n        run: rm -rf local-scripts .claude\n")
        fh.write("      - name: archived\n        run: echo hi\n")
    with open(os.path.join(t, LOCAL_HALF), "w") as fh:
        fh.write("#!/usr/bin/env bash\n")
        fh.write(f"# HOSTED MIRROR: {SITING_JOB} / sited rows\n")
        for want in TIER_BLIND:
            fh.write(f"{want}\n")
        fh.write('if [ "$TIER" = docs ]; then\n  exit 0\nfi\n')
        for i, n in enumerate(names):
            fh.write(f"# HOSTED MIRROR: discipline / mirrored step {i}\nscripts/{n}\n")
        fh.write("demos/render-uv.sh\ncd tools/toolcrate && cargo test\n")
    open(os.path.join(t, "demos/render-uv.sh"), "w").close()


def _run(root: str) -> tuple[int, str]:
    r = subprocess.run([sys.executable, os.path.abspath(__file__), "--root", root],
                       capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


def _case(want: str, plant) -> None:
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        plant(t)
        rc, out = _run(t)
        if rc == 0:
            raise SystemExit(f"SELFTEST FAILED: passed a planted violation ({plant.__name__})\n{out}")
        if want not in out:
            raise SystemExit(f"SELFTEST FAILED ({plant.__name__}): unexpected message\n{out}")


def _append(path: str, text: str):
    def go(t: str) -> None:
        with open(os.path.join(t, path), "a") as fh:
            fh.write(text)
    return go


def selftest_bail_messages() -> None:
    """EVERY `Bail` TELLS THE READER WHAT TO DO. A refusal that only says *no*
    is what sends the next person to the exemption list instead of to a
    two-minute fix, so each one either names the symbol to extend (`teach`) or
    says plainly that there is nothing to extend (`NO_TEACH`). Checked here
    rather than asserted in the header, because a convention about messages is
    exactly the kind that rots quietly."""
    with open(os.path.abspath(__file__), encoding="utf-8") as fh:
        src = fh.read()
    bad = []
    for i, chunk in enumerate(src.split("raise Bail(")[1:], 1):
        head = chunk[:900]
        stop = head.find("\n\n")
        body = head[:stop] if stop > 0 else head
        if "teach(" not in body and "NO_TEACH" not in body:
            bad.append(f"#{i}: {body.splitlines()[0].strip()[:90]}")
    if bad:
        raise SystemExit("SELFTEST FAILED: these Bail messages tell the reader nothing to do — add a "
                         "`teach(...)` pointer naming the symbol to extend, or `NO_TEACH` if there is "
                         "genuinely no shape to learn:\n  " + "\n  ".join(bad))


def selftest() -> None:
    selftest_bail_messages()
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        rc, out = _run(t)
        if rc != 0:
            raise SystemExit(f"SELFTEST FAILED: the checker FAILED on a clean fixture\n{out}")

    def hosted_only(t):        _append(HOSTED_HALF, "      - name: new\n        run: scripts/check-new.sh\n")(t); open(os.path.join(t, "scripts/check-new.sh"), "w").close()
    def local_only(t):         _append(LOCAL_HALF, "scripts/check-local.sh\n")(t); open(os.path.join(t, "scripts/check-local.sh"), "w").close()
    def gate_mode_one_side(t): _append(HOSTED_HALF, "      - name: cit\n        run: scripts/gates/probe-suite-census.sh --crates\n")(t)
    def ghost_path(t):         _append(HOSTED_HALF, "      - name: g\n        run: scripts/gone.sh\n")(t); _append(LOCAL_HALF, "scripts/gone.sh\n")(t)
    def orphan(t):             open(os.path.join(t, "scripts/orphan.sh"), "w").close()
    def tools_one_side(t):     os.makedirs(os.path.join(t, "tools/lonely"))
    def unpruned_job(t):       _append(HOSTED_HALF, "  extra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")(t)
    def uppercase_job(t):      _append(HOSTED_HALF, "  buildXtra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n")(t)
    def second_workflow(t):
        with open(os.path.join(t, ".github/workflows/render.yml"), "w") as fh:
            fh.write("jobs:\n  tour:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")
    def prune_after_read(t):   _append(HOSTED_HALF, "  late:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n      - run: rm -rf local-scripts .claude\n")(t)
    def unparseable(t):        _append(HOSTED_HALF, "  not a job name\n")(t)
    # F1: the flush-style step sequence, ordinary YAML, that the previous
    # reader parsed as a key named `- uses` and dropped — the `buildXtra`
    # defect moved one line down.
    def flush_style_steps(t): _append(HOSTED_HALF, "  sneaky:\n    steps:\n    - uses: actions/checkout@v4\n    - run: cat local-scripts/ci-local.sh\n")(t)
    # F4c: a three-space job body. Valid YAML, and the reader used to see no
    # steps at all in it.
    def three_space_body(t): _append(HOSTED_HALF, "  spaced:\n   steps:\n     - uses: actions/checkout@v4\n     - run: echo hi\n")(t)
    # F2: the siting job waits on a job that skips. Its own `if:` is absent,
    # which is all claim 7 used to read.
    def siting_job_needs(t): _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:", f"  {SITING_JOB}:\n    needs: discipline\n    steps:")
    def siting_job_soft(t):  _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:", f"  {SITING_JOB}:\n    continue-on-error: true\n    steps:")
    def merge_key(t):        _append(HOSTED_HALF, "  merged:\n    <<: *anchor\n    steps:\n      - run: echo hi\n")(t)
    def unknown_job_key(t):  _append(HOSTED_HALF, "  odd:\n    stepz:\n      - run: echo hi\n")(t)
    def unknown_step_key(t): _append(HOSTED_HALF, "  odd:\n    steps:\n      - runn: echo hi\n")(t)
    def tabbed(t):           _append(HOSTED_HALF, "  tabbed:\n\tsteps:\n")(t)
    def bad_func_spelling(t): _append(LOCAL_HALF, "weird() (\n  echo hi\n)\n")(t)
    def exemption_expired(t):
        _append(HOSTED_HALF, "      - name: uv\n        run: demos/render-uv.sh\n")(t)
    def exemption_orphaned(t):
        _sub(t, LOCAL_HALF, "demos/render-uv.sh\n", "")
        os.remove(os.path.join(t, "demos/render-uv.sh"))
    def marker_wrong_job(t):   _sub(t, LOCAL_HALF, "# HOSTED MIRROR: discipline / mirrored step 0", "# HOSTED MIRROR: k-lint / mirrored step 0")
    def marker_step_renamed(t): _sub(t, HOSTED_HALF, "- name: mirrored step 0", "- name: mirrored step zero")
    def markers_deleted(t):    _sub(t, LOCAL_HALF, "# HOSTED MIRROR: ", "# was: ")
    def job_unmirrored(t):     _append(HOSTED_HALF, "  lonely:\n    steps:\n      - uses: actions/checkout@v4\n      - name: prune\n        run: rm -rf local-scripts .claude\n      - name: work\n        run: cargo test\n")(t)
    def job_reason_empty(t):   _append(HOSTED_HALF, "  # NO LOCAL MIRROR:\n  lonely:\n    steps:\n      - uses: actions/checkout@v4\n      - name: prune\n        run: rm -rf local-scripts .claude\n      - name: work\n        run: cargo test\n")(t)
    def confession_expired(t): _append(LOCAL_HALF, "# HOSTED MIRROR: archive / archived\n")(t)
    def reason_detached(t):    _sub(t, HOSTED_HALF, "  # NO LOCAL MIRROR: nothing is archived on one machine\n", "  # NO LOCAL MIRROR: detached\n\n  # NO LOCAL MIRROR: nothing is archived on one machine\n")
    def exemption_inverted(t): _sub(t, LOCAL_HALF, "demos/render-uv.sh\n", ""); _append(HOSTED_HALF, "      - name: uv\n        run: demos/render-uv.sh\n")(t)

    _case("and local-scripts/ci-local.sh does not", hosted_only)
    _case("and .github/workflows/ci.yml does not", local_only)
    _case("is invoked by the hosted half only", gate_mode_one_side)
    _case("and no such file exists", ghost_path)
    _case("NEITHER half names", orphan)
    _case("named by neither half", tools_one_side)
    _case("checks the repo out and does not delete", unpruned_job)
    _case("job `buildXtra` checks the repo out", uppercase_job)
    _case("render.yml job `tour` checks the repo out", second_workflow)
    _case("prunes it at step", prune_after_read)
    _case("but it is the one job", _resite_prune)
    _case("not a job name at job indent", unparseable)
    _case("job `sneaky` checks the repo out", flush_style_steps)
    _case("job `spaced` checks the repo out", three_space_body)
    _case("waits on `discipline` carries an `if:`", siting_job_needs)
    _case("cannot redden the PR", siting_job_soft)
    _case("not a `key:` line in job `merged`", merge_key)
    _case("carries the key `stepz`", unknown_job_key)
    _case("carries the key `runn`", unknown_step_key)
    _case("a TAB character", tabbed)
    _case("looks like a shell function definition", bad_func_spelling)
    _case("BOTH halves now name it", exemption_expired)
    _case("NEITHER half names it", exemption_orphaned)
    _case("has no step named", marker_wrong_job)
    _case("has no step named", marker_step_renamed)
    _case("below the", markers_deleted)
    _case("declared local-only in MIRROR_EXEMPT", exemption_inverted)
    # THE SITING RULE, both halves. These are the cases the reviewer's
    # experiment plants: hollow `mirror`, move the steps back into a job that
    # skips on docs tier; and, locally, move the row below the docs exit.
    _case("has no local mirror", job_unmirrored)
    _case("with no reason after it", job_reason_empty)
    _case("The confession has expired", confession_expired)
    _case("not against a job key", reason_detached)
    _case("carries an `if:`", _hollow_siting_job)
    _case("a definition above the exit is not a run", _local_row_below_exit)
    _case("never runs", _local_row_deleted)
    print("check-ci-mirror-parity selftest OK: every Bail names the symbol to extend or says there is "
          "none; passes a clean fixture; fires on a one-sided row, a "
          "one-sided gate MODE, a path both halves name that does not exist, an orphan script, a "
          "one-sided tools/ crate, a checked-out job that keeps either tree, an UPPERCASE job name doing "
          "the same, a second workflow file growing one, a prune that comes after the read, the siting job "
          "pruning, an unparseable workflow, a flush-style or three-space step block hiding a checked-out job, the siting job given a `needs:` onto a skipping job or `continue-on-error`, a merge key, an unknown job or step key, a tab, an unrecognised shell function spelling, an exemption that expired or was orphaned, a marker naming the wrong job or a renamed step, the markers "
          "deleted, an inverted exemption, the sited steps moved back into an `if:` job, the local "
          "half's rows wrapped in a function called below its docs-tier exit, or deleted, a hosted JOB "
          "with neither a citation nor a reason, a reason with nothing after it, a reason on a job the "
          "local half cites anyway, and a reason separated from its job key by a blank line")


def _sub(t: str, path: str, a: str, b: str) -> None:
    full = os.path.join(t, path)
    with open(full, encoding="utf-8") as fh:
        s = fh.read()
    with open(full, "w", encoding="utf-8") as fh:
        fh.write(s.replace(a, b))


def _resite_prune(t: str) -> None:
    _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n",
         f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n      - run: rm -rf local-scripts .claude\n")


def _hollow_siting_job(t: str) -> None:
    """The reviewer's experiment: hollow `mirror`, move its steps into the
    `if:`-guarded job."""
    moved = "".join(f"      - name: sited {w}\n        run: {w}\n" for w in TIER_BLIND)
    _sub(t, HOSTED_HALF, moved, "      - run: echo hollow\n")
    _sub(t, HOSTED_HALF, "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n",
         "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n" + moved)


def _local_row_below_exit(t: str) -> None:
    """The rows wrapped in a function DEFINED above the docs exit and CALLED
    below it — the shape that satisfies a "mentioned before the exit" check
    while running nothing on the tier the rows are about."""
    for w in TIER_BLIND:
        _sub(t, LOCAL_HALF, f"{w}\n", "")
    body = "".join(f"  {w}\n" for w in TIER_BLIND)
    _sub(t, LOCAL_HALF, 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n',
         f"tier_blind_rows() {{\n{body}}}\n" + 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n'
         "tier_blind_rows\n")


def _local_row_deleted(t: str) -> None:
    _sub(t, LOCAL_HALF, f"{TIER_BLIND[0]}\n", "")


def main() -> int:
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    args = sys.argv[1:]
    if "--selftest" in args:
        selftest()
        return 0
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    try:
        errs = check(root)
    except Bail as exc:
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {exc}")
        print(f"ERROR: check-ci-mirror-parity: {exc}", file=sys.stderr)
        return 1
    for e in errs:
        # BOTH FORMS. `::error::` is what puts the message on the failing step
        # in the Actions UI; the plain line on stderr is what a `gh run view
        # --log` and a piped local run carry. Neither subsumes the other, and
        # the cost of printing both is one line.
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {e}")
        print(f"ERROR: check-ci-mirror-parity: {e}", file=sys.stderr)
    if errs:
        return 1
    print("check-ci-mirror-parity OK: both halves name the same checks and the same gate modes, no orphan "
          "or missing check under scripts/ or demos/, both tools/ crates' halves agree, every checked-out "
          f"job in {WORKFLOW_DIR}/ but `{SITING_JOB}` prunes local-only tooling before reading it, every "
          "tier-blind check is sited in a job that no `if:`, `needs:` chain or `continue-on-error` can "
          "skip and above the local half's docs exit, all hosted-mirror citations resolve, and every "
          "ci.yml job is either cited by the local half or says at its own key why it has no local half")
    return 0


if __name__ == "__main__":
    sys.exit(main())
