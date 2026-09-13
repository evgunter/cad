#!/usr/bin/env python3
"""`local-scripts/render-hosted.sh` works from a lane roster, and `render.yml`
declares one. This is the thing that reds when they disagree.

WHY THIS EXISTS. The helper is the one-command front end for a hosted render:
push, dispatch, poll, download, install. Every lane fact in it — which lanes
exist, what `--lane all` expands to, what `--lane` accepts, each lane's artifact
name, each lane's committed directory, which jobs the poll waits for — is a
statement about `render.yml`, and nothing compared any of them to it. A count in
prose and a roster in bash go stale the same way: silently, at the next lane.

WHAT IT PROVES. Against `.github/workflows/render.yml`'s own declarations — the
`lanes` input's `options`, each lane's `rebaseline-lane` step (`lane:` + `dir:`),
each lane's `upload-artifact` step (`name:` + `path:`), the job each of those
sits in, and the `gh run download` roster in that file's header — it holds every
lane fact `render-hosted.sh --print-lane-table` answers with:

  claim 1  the lane SETS are equal;
  claim 2  each lane's ARTIFACT NAME is the one render.yml uploads for it;
  claim 3  each lane's COMMITTED DIRECTORY is the one render.yml re-baselines;
  claim 4  `RENDER_JOBS_RE`'s alternatives ARE the jobs that upload lane
           artifacts — every one of them, and nothing else;
  claim 5  the helper's two DERIVED lists agree with its table: `--lane all`
           expands to every lane, and `--lane` accepts every lane and `all`
           and nothing else;
  claim 6  render.yml's own header roster names exactly the lane artifacts.

Claim 4 is not bookkeeping. That regex decides which jobs the poll waits for, so
a lane job it does not match is a lane the poll will not wait for: the run reads
as settled while that lane is still drawing, and the download then reports a
missing artifact for a lane that was going to produce one. The GUI lane's job is
a third job and the regex named two — latent while the helper refused
`--lane gui` outright and `all` never yielded it, and live the moment the helper
learned the lane.

It is stated as a ROSTER rather than as a filter over a job population, and that
is the whole of why it is sound. The helper's DEFAULT path polls a `ci.yml` run,
where these jobs arrive prefixed (`render lanes / viewer gui montage`) and the
match is on the anchored suffix — so the population a filter would have to read
is every job of both workflows, and ci.yml's are largely `${{ matrix… }}`
templates that no static reader expands. Comparing the pattern's own
alternatives to the lane jobs needs no population at all, and refuses a regex
that reaches a foreign job of EITHER file for the same reason it refuses one
that misses a lane.

Claim 5 exists because the two lists it holds are the two that decide behaviour
and the two a table print does not reach. `--lane all` silently taking four of
six lanes was this file's worst state, worse than the refusal that was visible.

WHAT IT DOES NOT PROVE. Not that a lane RENDERS anything, or renders the right
thing — that is the lane's own gate. Not the ORDER of the roster, which is
cosmetic and deliberately unchecked. Not that `render.yml`'s per-lane `if:`
conditions select the lane the caller asked for. Not the helper's control flow:
it reads the ANSWERS `--print-lane-table` prints, so a mutation that bypasses one
of those functions at one call site, leaving the function itself intact, is
outside this reader. Not every possible over-acceptance: the `accepts` probe
asks the helper's own predicate about each lane in its table, about `all`, and
about one string that is no lane, so a predicate that additionally admits some
fourth specific spelling is outside it too. Not the lane PROPERTIES render.yml states only in prose —
which lanes are byte-reproducible (`VERIFY_LANES`,
`work/ciw/verify-lane-set-is-a-property-nothing-declares`) and which draw PNGs
(`demos/check_render_provenance.py`'s `LANE_DIRS`,
`work/ciw/provenance-lane-dirs-is-not-the-lane-roster`) are both unreachable for
the same reason: render.yml declares neither anywhere a reader can key on.

WHERE IT IS CONSERVATIVE. A shape it cannot read raises `Bail` and fails the run
rather than passing it: a `lanes` input with no `options` list, a declared lane
with no `rebaseline-lane` step, a re-baselined directory nothing uploads, a job
regex outside the vocabulary below or one Python cannot compile, or a helper
that prints no table, prints an unreadable line, hangs, or exits non-zero. That
direction is a loud failure whose message states the fix — the direction that
gets a checker routed around is the silent one.

THE JOB-REGEX VOCABULARY is the characters GitHub actually puts in these job
names (letters, digits, space and `+()` `,.:_-`), plus the alternation `|`, plus
`\\`-escapes of `+()`, and it must end in `$` so the match is the anchored
suffix the helper's comment claims. It is not "literals only": `.` `+` `(` `)`
unescaped are admitted, and are metacharacters in both engines. The bound is a
MEASUREMENT, taken by this file's correctness review rather than by its author:
twenty patterns run through Python `re` and jq 1.7 side by side, no
in-vocabulary pattern the two engines disagree about, every out-of-vocabulary
metacharacter refused. So what this vocabulary buys is that the pattern the
helper evaluates in jq and this file evaluates in Python means the same thing,
not that it is free of metacharacters. Claim 4 narrows it further on its own
account: it reads the alternatives as NAMES, and refuses any that is not literal
once its `\\`-escapes are undone.

Stdlib only, and a line recogniser rather than a YAML parser: the same posture as
`scripts/check-ci-mirror-parity.py` and `scripts/check-status-capture.py`, whose
headers argue it. The helper's half is not parsed at all — it is EXECUTED, the
way `scripts/doc-gate.sh --print-roots` derives its roots instead of restating
them, so what is compared is what the script answers.

  check-render-lane-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import argparse
import contextlib
import inspect
import io
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

WORKFLOW = ".github/workflows/render.yml"
HELPER = "local-scripts/render-hosted.sh"
SUBJECTS = (WORKFLOW, HELPER)

TOP_KEY = re.compile(r"^(?P<key>[A-Za-z_][A-Za-z0-9_-]*):")
JOB_KEY = re.compile(r"^  (?P<id>[A-Za-z0-9_-]+):\s*$")
JOB_NAME = re.compile(r"^    name:\s*(?P<name>\S.*?)\s*$")
# A step begins at the `- ` of its first key, and that key is `name:` as often
# as it is `uses:` — render.yml spells unnamed upload steps the second way.
STEP_START = re.compile(r"^\s+- (?P<key>[a-z]+):\s*(?P<value>.*?)\s*$")
USES = re.compile(r"^\s+uses:\s*(?P<uses>\S+)\s*$")
WITH_KEY = re.compile(r"^\s+(?P<key>[a-z_]+):\s*(?P<value>\S.*?)\s*$")
OPTIONS = re.compile(r"^\s*options:\s*\[(?P<items>[^\]]*)\]\s*$")
LANES_INPUT = re.compile(r"^\s*lanes:\s*$")
HEADER_ROSTER = re.compile(r"^#.*\bgh run download\b.*\s-n\s+(?P<artifact>\S+)")

REBASELINE_ACTION = "./.github/actions/rebaseline-lane"
UPLOAD_ACTION_PREFIX = "actions/upload-artifact@"

JOBS_RE_VOCABULARY = re.compile(r"^(?:\\[+()]|[A-Za-z0-9 +(),.:_|-])*\$$")

# The claims, named. The selftest holds this roster against the claims the
# mutant table exercises and against `check`'s own source, so a claim deleted,
# renamed, or left with no mutant reds SAYING WHICH — rather than passing with
# a count one lower and a success sentence still claiming it.
CLAIMS = (
    ("claim 1", "the lane sets are equal"),
    ("claim 2", "artifact names"),
    ("claim 3", "committed directories"),
    ("claim 4", "the poll's job roster is the lane jobs"),
    ("claim 5", "`all` expands to every lane and `--lane` accepts exactly them"),
    ("claim 6", "render.yml's header roster names the lane artifacts"),
)


class Bail(Exception):
    """A shape this script cannot read. Fails the run rather than passing it."""


class Fail(Exception):
    """A claim that does not hold, named by the claim it belongs to."""


# --------------------------------------------------------------- render.yml


class Lane:
    __slots__ = ("artifact", "dir", "job", "name")

    def __init__(self, name: str) -> None:
        self.name = name
        self.dir: str | None = None
        self.artifact: str | None = None
        self.job: str | None = None


def declared_lanes(text: str) -> list[str]:
    """The `lanes` input's `options`, minus `all`, in declaration order."""
    lines = text.splitlines()
    for i, line in enumerate(lines):
        if not LANES_INPUT.match(line):
            continue
        for follow in lines[i + 1 : i + 12]:
            m = OPTIONS.match(follow)
            if m:
                items = [x.strip() for x in m.group("items").split(",") if x.strip()]
                if "all" not in items:
                    raise Bail(
                        f"{WORKFLOW}: the `lanes` input's options do not offer "
                        f"`all` ({items!r}); the helper does"
                    )
                return [x for x in items if x != "all"]
    raise Bail(
        f"{WORKFLOW}: found no `options: [...]` list under a `lanes:` input. "
        "The lane roster is read from there; nothing else declares it."
    )


def header_roster(text: str) -> list[str]:
    """The artifacts render.yml's own header tells a reader to download."""
    return [
        m.group("artifact")
        for line in text.splitlines()
        if (m := HEADER_ROSTER.match(line))
    ]


def workflow_lanes(text: str) -> dict[str, Lane]:
    """Lane -> (dir, artifact, job), read out of render.yml's own steps.

    The join is the committed directory: a `rebaseline-lane` step names the lane
    and its directory, and the `upload-artifact` step whose `path:` is that
    directory names the artifact. Nothing here keys on the `renders-<lane>`
    spelling, so a lane whose artifact is named some other way is read, not
    assumed.
    """
    lanes = {name: Lane(name) for name in declared_lanes(text)}
    by_dir = {}
    uploads: list[tuple[str, str, str]] = []  # (artifact, path-dir, job name)

    in_jobs = False
    job = None
    action = None
    fields: dict[str, str] = {}
    step_line = 0

    def close_step() -> None:
        nonlocal action, fields
        if action == REBASELINE_ACTION:
            lane, directory = fields.get("lane"), fields.get("dir")
            if not lane or not directory:
                raise Bail(
                    f"{WORKFLOW}:{step_line}: a rebaseline-lane step with no "
                    f"`lane:`/`dir:` pair ({fields!r})"
                )
            if lane not in lanes:
                raise Bail(
                    f"{WORKFLOW}:{step_line}: re-baselines lane '{lane}', which "
                    "the `lanes` input does not declare"
                )
            lanes[lane].dir = directory
            by_dir[directory] = lane
        elif action and action.startswith(UPLOAD_ACTION_PREFIX):
            name, path = fields.get("name"), fields.get("path")
            if name and path and path.endswith("/**"):
                uploads.append((name, path[: -len("/**")], job or "?"))
        action, fields = None, {}

    for n, line in enumerate(text.splitlines(), 1):
        m = TOP_KEY.match(line)
        if m:
            close_step()
            in_jobs = m.group("key") == "jobs"
            job = None
            continue
        if in_jobs:
            m = JOB_KEY.match(line)
            if m:
                close_step()
                job = m.group("id")
                continue
            m = JOB_NAME.match(line)
            if m and job:
                job = m.group("name")
                continue
        m = STEP_START.match(line)
        if m:
            close_step()
            step_line = n
            if m.group("key") == "uses":
                action = m.group("value")
            continue
        m = USES.match(line)
        if m:
            action = m.group("uses")
            continue
        m = WITH_KEY.match(line)
        if m and action:
            fields.setdefault(m.group("key"), m.group("value"))
    close_step()

    for lane in lanes.values():
        if lane.dir is None:
            raise Bail(
                f"{WORKFLOW}: lane '{lane.name}' is declared in the `lanes` "
                "input and has no rebaseline-lane step, so nothing here says "
                "what directory it lands in"
            )
    for artifact, directory, job_name in uploads:
        lane_name = by_dir.get(directory)
        if lane_name is None:
            continue
        lanes[lane_name].artifact = artifact
        lanes[lane_name].job = job_name
    for lane in lanes.values():
        if lane.artifact is None:
            raise Bail(
                f"{WORKFLOW}: lane '{lane.name}' re-baselines {lane.dir} and "
                "no step uploads that directory, so it has no artifact to pull"
            )
    return lanes


def jobs_re_alternatives(jobs_re: str) -> list[str]:
    """The literal job names an anchored alternation names, or `Bail`.

    `(a|b\\(c\\))$` -> ["a", "b(c)"]. A pattern that is not one anchored group
    of `|`-separated literals — or whose alternatives still carry a
    metacharacter once the `\\`-escapes are undone — is one this reader must not
    read as a roster, so it refuses rather than guessing.
    """
    body = jobs_re[:-1] if jobs_re.endswith("$") else jobs_re
    if not (body.startswith("(") and body.endswith(")")):
        raise Bail(
            f"{HELPER}: RENDER_JOBS_RE is {jobs_re!r}, not one anchored "
            "`(name|name|…)$` group — this reader holds its alternatives "
            "against the lane jobs and cannot read another shape."
        )
    out = []
    for alt in body[1:-1].split("|"):
        literal = ""
        i = 0
        while i < len(alt):
            ch = alt[i]
            if ch == "\\":
                if i + 1 >= len(alt) or alt[i + 1] not in "+()":
                    raise Bail(
                        f"{HELPER}: RENDER_JOBS_RE's alternative {alt!r} "
                        "carries an escape this reader does not know."
                    )
                literal += alt[i + 1]
                i += 2
                continue
            if ch in ".*?[]{}^$+()":
                raise Bail(
                    f"{HELPER}: RENDER_JOBS_RE's alternative {alt!r} carries "
                    f"an unescaped `{ch}` — it is a pattern, and this reader "
                    "holds the alternatives against the lane jobs as names."
                )
            literal += ch
            i += 1
        out.append(literal)
    return out


# ------------------------------------------------------------- the helper


class Helper:
    """What `render-hosted.sh --print-lane-table` answers."""

    def __init__(self) -> None:
        self.table: dict[str, tuple[str, str]] = {}
        self.jobs_re: str | None = None
        self.lanes_all: list[str] | None = None
        self.accepts: dict[str, bool] = {}


# HOW LONG THE TABLE PRINT GETS, and why the selftest is allowed a
# shorter one. A healthy `--print-lane-table` answers in under a tenth of
# a second — it is answered before the checkout, the `gh` checks and any
# network use — so 60 s is a ceiling on a HANG and not a budget anything
# approaches. The selftest proves that ceiling fires by planting a
# `sleep 120` in the helper, and with one ceiling it had to wait the
# whole 60 s out in real time, on every run of a job that carries no
# `if:` and so runs on every tier. It passes its own instead: the arm
# under test is the same one, and what changes is only how long a hang
# is given before it is called one.
HELPER_TIMEOUT_S = 60.0
SELFTEST_HELPER_TIMEOUT_S = 5.0


def helper_answers(root: Path, timeout_s: float = HELPER_TIMEOUT_S) -> Helper:
    script = root / HELPER
    try:
        proc = subprocess.run(
            ["bash", str(script), "--print-lane-table"],
            capture_output=True,
            text=True,
            cwd=root,
            timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        raise Bail(
            f"{HELPER} --print-lane-table did not answer within "
            f"{timeout_s:g}s. It is "
            "answered before the checkout, the `gh` checks and any network "
            "use, so a hang here is a defect in the table print itself."
        ) from None
    if proc.returncode != 0:
        raise Bail(
            f"{HELPER} --print-lane-table exited {proc.returncode}: "
            f"{proc.stderr.strip() or proc.stdout.strip()}"
        )
    h = Helper()
    for line in proc.stdout.splitlines():
        parts = line.split()
        if parts[:1] == ["lane"] and len(parts) == 4:
            h.table[parts[1]] = (parts[2], parts[3])
        elif parts[:1] == ["jobs-re"] and len(parts) >= 2:
            h.jobs_re = line.split(" ", 1)[1]
        elif parts[:1] == ["lanes-all"]:
            h.lanes_all = parts[1:]
        elif parts[:1] == ["accepts"] and len(parts) == 3 and parts[2] in ("yes", "no"):
            h.accepts[parts[1]] = parts[2] == "yes"
        elif line.strip():
            raise Bail(f"{HELPER} --print-lane-table printed {line!r}, unreadable")
    if not h.table:
        raise Bail(f"{HELPER} --print-lane-table named no lanes at all")
    if h.jobs_re is None:
        raise Bail(f"{HELPER} --print-lane-table printed no `jobs-re` line")
    if h.lanes_all is None:
        raise Bail(f"{HELPER} --print-lane-table printed no `lanes-all` line")
    if not h.accepts:
        raise Bail(f"{HELPER} --print-lane-table printed no `accepts` lines")
    if not JOBS_RE_VOCABULARY.match(h.jobs_re):
        raise Bail(
            f"{HELPER}: RENDER_JOBS_RE is {h.jobs_re!r}, outside the vocabulary "
            "this reader has measured Python's and jq's regex engines to agree "
            "on (job-name characters, `|`, `\\`-escaped `+()`, ending in `$`)"
        )
    return h


# ------------------------------------------------------------- the claims


def check(root: Path, helper_timeout_s: float = HELPER_TIMEOUT_S) -> None:
    text = (root / WORKFLOW).read_text()
    declared = workflow_lanes(text)
    helper = helper_answers(root, helper_timeout_s)
    table = helper.table

    # claim 1: the lane sets are equal.
    missing = sorted(set(declared) - set(table))
    extra = sorted(set(table) - set(declared))
    if missing:
        raise Fail(
            f"claim 1: {WORKFLOW} declares lane(s) {missing} that {HELPER}'s "
            "lane table does not carry — `--lane` refuses them and `--lane all` "
            "silently leaves them out. Add a row to LANE_TABLE."
        )
    if extra:
        raise Fail(
            f"claim 1: {HELPER}'s lane table carries {extra}, which {WORKFLOW} "
            "does not declare — a dispatch would be refused by the workflow."
        )

    for name in sorted(declared):
        lane = declared[name]
        pair = table.get(name)
        if pair is None:
            raise Fail(f"claim 1: {HELPER}'s lane table has no row for '{name}'.")
        artifact, directory = pair
        # claim 2: artifact names.
        if artifact != lane.artifact:
            raise Fail(
                f"claim 2: lane '{name}' uploads artifact '{lane.artifact}' in "
                f"{WORKFLOW}; {HELPER} downloads '{artifact}'."
            )
        # claim 3: committed directories.
        if directory != lane.dir:
            raise Fail(
                f"claim 3: lane '{name}' re-baselines '{lane.dir}' in "
                f"{WORKFLOW}; {HELPER} installs into '{directory}'."
            )

    # claim 4: the poll's roster IS the jobs that upload lane artifacts.
    try:
        re.compile(helper.jobs_re)
    except re.error as exc:
        raise Bail(
            f"{HELPER}: RENDER_JOBS_RE is {helper.jobs_re!r}, which Python "
            f"cannot compile ({exc}). It is in the vocabulary and still not a "
            "regex; fix the pattern."
        ) from None
    named = set(jobs_re_alternatives(helper.jobs_re))
    lane_jobs = {lane.job for lane in declared.values() if lane.job}
    unwaited = sorted(lane_jobs - named)
    if unwaited:
        raise Fail(
            f"claim 4: lane job(s) {unwaited} upload a lane artifact and "
            f"{HELPER}'s RENDER_JOBS_RE does not name them — the poll would "
            "neither wait for those lanes nor report them, and could call the "
            "run settled while one is still drawing."
        )
    foreign = sorted(named - lane_jobs)
    if foreign:
        raise Fail(
            f"claim 4: RENDER_JOBS_RE names {foreign}, which upload no lane "
            "artifact — the poll would wait on jobs that are not render lanes. "
            "On the default path it polls a ci.yml run, where the match is on "
            "the anchored suffix, so a foreign name here reaches that file's "
            "jobs too."
        )

    # claim 5: the two derived lists, read as the helper's own answers.
    if sorted(helper.lanes_all or []) != sorted(table):
        raise Fail(
            f"claim 5: `--lane all` expands to {helper.lanes_all} in {HELPER}, "
            f"not to its own lane table {sorted(table)} — a download that asks "
            "for everything would silently leave lanes out."
        )
    for lane_name in sorted(table):
        if not helper.accepts.get(lane_name):
            raise Fail(
                f"claim 5: {HELPER} does not accept `--lane {lane_name}`, "
                "though its own lane table carries that lane."
            )
    if not helper.accepts.get("all"):
        raise Fail(f"claim 5: {HELPER} does not accept `--lane all`.")
    for candidate, accepted in sorted(helper.accepts.items()):
        if accepted and candidate != "all" and candidate not in table:
            raise Fail(
                f"claim 5: {HELPER} accepts `--lane {candidate}`, which is "
                "neither a lane in its table nor `all`."
            )

    # claim 6: render.yml's header tells a reader to download these.
    roster = header_roster(text)
    want = sorted(lane.artifact or "" for lane in declared.values())
    if sorted(roster) != want:
        raise Fail(
            f"claim 6: {WORKFLOW}'s header roster names {sorted(roster)}; the "
            f"lanes it declares upload {want}. That roster is the command a "
            "reader copies, so a stale entry prescribes a download that fails."
        )

    print(
        f"render lane parity: {len(declared)} lanes "
        f"({', '.join(sorted(declared))}), and "
        + "; ".join(f"{n} — {d}" for n, d in CLAIMS)
        + " — all as specified."
    )


# ----------------------------------------------------------------- selftest

# Each row mutates a COPY of the three subject files and asserts the verdict.
# Every edit names its own file: a row whose two edits land in one file is a row
# that still says what it means. The GREEN rows matter as much as the red ones —
# a checker that reds on a correct change gets routed around, and then detects
# nothing at all.
MUTANTS: tuple[tuple[str, tuple[tuple[str, str, str], ...], str | None], ...] = (
    (
        # The lane's own steps outlive the roster entry, so this reds one
        # refusal earlier than claim 1 — at the step that re-baselines a lane
        # the input no longer offers.
        "workflow drops a lane",
        ((WORKFLOW, ", gui]", "]"),),
        "which the `lanes` input does not declare",
    ),
    (
        "helper carries a lane the workflow does not declare",
        (
            (
                HELPER,
                "gui     renders-gui     demos/renders-gui\n",
                "gui     renders-gui     demos/renders-gui\n"
                "plate   renders-plate   demos/renders-plate\n",
            ),
        ),
        "claim 1",
    ),
    (
        "workflow adds a seventh lane, fully declared",
        (
            (WORKFLOW, ", gui]", ", gui, plate]"),
            (
                WORKFLOW,
                "          name: renders-mc\n          path: demos/renders-mc/**\n",
                "          name: renders-mc\n          path: demos/renders-mc/**\n"
                "      - name: upload plate (plate)\n"
                "        uses: actions/upload-artifact@v4\n"
                "        with:\n"
                "          name: renders-plate\n"
                "          path: demos/renders-plate/**\n",
            ),
            (
                WORKFLOW,
                "          lane: mc\n          dir: demos/renders-mc\n",
                "          lane: mc\n          dir: demos/renders-mc\n"
                "      - name: re-baseline committed lane (plate)\n"
                "        uses: ./.github/actions/rebaseline-lane\n"
                "        with:\n"
                "          lane: plate\n"
                "          dir: demos/renders-plate\n",
            ),
            (
                WORKFLOW,
                "#   gh run download <run-id> -n renders-gui",
                "#   gh run download <run-id> -n renders-gui\n"
                "#   gh run download <run-id> -n renders-plate",
            ),
        ),
        "claim 1",
    ),
    (
        "workflow renames a lane",
        ((WORKFLOW, " mc,", " density,"), (WORKFLOW, "lane: mc", "lane: density")),
        "claim 1",
    ),
    (
        "helper drops a lane from its table",
        ((HELPER, "gui     renders-gui     demos/renders-gui\n", ""),),
        "claim 1",
    ),
    (
        "helper's artifact name drifts",
        ((HELPER, "renders-uv      demos", "renders-uvs     demos"),),
        "claim 2",
    ),
    (
        "helper's committed directory drifts",
        (
            (
                HELPER,
                "kernel  renders-kernel  demos/renders\n",
                "kernel  renders-kernel  demos/renders-kernel\n",
            ),
        ),
        "claim 3",
    ),
    (
        "helper's poll regex loses a lane job",
        ((HELPER, "|viewer gui montage)$", ")$"),),
        "claim 4",
    ),
    (
        "helper's poll regex reaches a non-lane job of render.yml",
        ((HELPER, "|viewer gui montage)$", "|viewer gui montage|checkout target)$"),),
        "claim 4",
    ),
    (
        # The default path polls a CI run, and claim 4's roster form is what
        # refuses a name from that file without reading its jobs.
        "helper's poll regex reaches a job of ci.yml",
        (
            (
                HELPER,
                "|viewer gui montage)$",
                "|viewer gui montage|k-lint \\(gate, dev-probe\\))$",
            ),
        ),
        "claim 4",
    ),
    (
        "workflow renames a lane's job",
        ((WORKFLOW, "    name: viewer gui montage", "    name: viewer window montage"),),
        "claim 4",
    ),
    (
        "helper's `all` stops meaning every lane",
        (
            (
                HELPER,
                'if [ "$1" = all ]; then lane_names | tr \'\\n\' \' \'; else echo "$1"; fi',
                'if [ "$1" = all ]; then echo "kernel freecad uv wild"; else echo "$1"; fi',
            ),
        ),
        "claim 5",
    ),
    (
        "helper stops accepting `--lane all`",
        ((HELPER, "for l in $(lane_names) all; do", "for l in $(lane_names); do"),),
        "claim 5",
    ),
    (
        "helper's lane refusal stops refusing",
        ((HELPER, "    for l in $(lane_names) all; do [ \"$l\" != \"$1\" ] || return 0; done\n    return 1",
          "    for l in $(lane_names) all; do [ \"$l\" != \"$1\" ] || return 0; done\n    return 0"),),
        "claim 5",
    ),
    (
        "workflow's header roster loses a lane",
        ((WORKFLOW, "#   gh run download <run-id> -n renders-mc", "#"),),
        "claim 6",
    ),
    (
        "workflow's header roster prescribes a stale artifact",
        ((WORKFLOW, "-n renders-wild ", "-n renders-wilds "),),
        "claim 6",
    ),
    (
        "workflow's lane options list is gone",
        ((WORKFLOW, "        options: [all, kernel", "        removed: [all, kernel"),),
        "no `options:",
    ),
    (
        "workflow re-baselines a lane nothing uploads",
        (
            (
                WORKFLOW,
                "          path: demos/renders-mc/**\n",
                "          path: demos/renders-mc-cells/**\n",
            ),
        ),
        "no step uploads that directory",
    ),
    (
        "helper's lane table is emptied",
        (
            (
                HELPER,
                "kernel  renders-kernel  demos/renders\n"
                "freecad renders-freecad demos/renders-freecad\n"
                "uv      renders-uv      demos/renders-uv\n"
                "mc      renders-mc      demos/renders-mc\n"
                "wild    renders-wild    demos/renders-wild\n"
                "gui     renders-gui     demos/renders-gui\n",
                "",
            ),
        ),
        "named no lanes at all",
    ),
    (
        "helper's poll regex is widened past the measured vocabulary",
        ((HELPER, "(scene inputs", "(.*|scene inputs"),),
        "vocabulary",
    ),
    (
        "helper's poll regex is in the vocabulary and not a regex",
        ((HELPER, "(scene inputs", "(scene inputs ("),),
        "cannot compile",
    ),
    (
        "helper hangs instead of answering",
        (
            (
                HELPER,
                "if [ \"$PRINT_TABLE_ONLY\" = 1 ]; then print_lane_table; exit 0; fi",
                "if [ \"$PRINT_TABLE_ONLY\" = 1 ]; then sleep 120; exit 0; fi",
            ),
        ),
        "did not answer within",
    ),
    # GREEN: the roster's ORDER is cosmetic and is not a claim.
    (
        "helper's table is reordered",
        (
            (
                HELPER,
                "kernel  renders-kernel  demos/renders\n"
                "freecad renders-freecad demos/renders-freecad\n",
                "freecad renders-freecad demos/renders-freecad\n"
                "kernel  renders-kernel  demos/renders\n",
            ),
        ),
        None,
    ),
    # GREEN: a lane may be named anything; the join is the directory, not the
    # `renders-<lane>` spelling — so long as every half says the same name.
    (
        "a lane's artifact is renamed in all three of its homes at once",
        (
            (WORKFLOW, "name: renders-wild", "name: wild-cells"),
            (WORKFLOW, "-n renders-wild ", "-n wild-cells    "),
            (HELPER, "wild    renders-wild ", "wild    wild-cells   "),
        ),
        None,
    ),
)


def claim_roster_holds() -> list[str]:
    """The claim names, held against the mutant table and against `check`."""
    problems = []
    named = {name for name, _ in CLAIMS}
    exercised = {m[2] for m in MUTANTS if m[2] and m[2].startswith("claim ")}
    for name in sorted(named - exercised):
        problems.append(f"{name} is declared and no mutant exercises it")
    for name in sorted(exercised - named):
        problems.append(f"{name} is exercised by a mutant and is not declared")
    source = inspect.getsource(check)
    for name in sorted(named):
        if f'"{name}' not in source and f"'{name}" not in source:
            problems.append(f"{name} is declared and `check` never raises it")
    return problems


def selftest(root: Path) -> int:
    rc = 0
    for problem in claim_roster_holds():
        print(f"  FAIL  claim roster: {problem}")
        rc = 1
    for label, edits, expect in MUTANTS:
        with tempfile.TemporaryDirectory() as tmp:
            tree = Path(tmp)
            for rel in SUBJECTS:
                dst = tree / rel
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(root / rel, dst)
            broken = False
            for rel, old, new in edits:
                path = tree / rel
                body = path.read_text()
                if old not in body:
                    print(f"  SELFTEST BROKEN  {label}: no site in {rel} for {old!r}")
                    rc, broken = 1, True
                    continue
                path.write_text(body.replace(old, new))
            if broken:
                continue
            try:
                # The check's own success line belongs to the tree it ran on,
                # not to this report.
                with contextlib.redirect_stdout(io.StringIO()):
                    check(tree, SELFTEST_HELPER_TIMEOUT_S)
                verdict, message = "GREEN", ""
            except (Bail, Fail) as exc:
                verdict, message = "RED", str(exc)
            ok = verdict == "GREEN" if expect is None else (
                verdict == "RED" and expect in message
            )
            print(
                f"  {'ok  ' if ok else 'FAIL'}  {label}: {verdict}"
                + (f" ({message.splitlines()[0][:90]})" if message else "")
            )
            if not ok:
                rc = 1
    print(
        f"selftest: {len(MUTANTS)} mutants, each asserted on its own tree — "
        f"{sum(1 for m in MUTANTS if m[2] is None)} of them rows that must stay "
        f"GREEN — over the claims {', '.join(n for n, _ in CLAIMS)}, each held "
        "to a mutant of its own and to `check`'s own source."
    )
    return rc


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--root", default=".")
    args = ap.parse_args()
    root = Path(args.root).resolve()
    if args.selftest:
        return selftest(root)
    try:
        check(root)
    except (Bail, Fail) as exc:
        print(f"check-render-lane-parity: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
