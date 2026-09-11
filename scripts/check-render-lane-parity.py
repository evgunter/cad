#!/usr/bin/env python3
"""`local-scripts/render-hosted.sh` works from a lane roster, and `render.yml`
declares one. This is the thing that reds when they disagree.

WHY THIS EXISTS. The helper is the one-command front end for a hosted render:
push, dispatch, poll, download, install. It knew `kernel|freecad|uv|wild` and it
never knew `mc` — so `--lane mc` was refused outright for the whole life of that
lane, and `--lane gui` from the day the GUI lane landed. `--lane all`, the
default, was worse than the refusal: it silently took four of six lanes and
called that all. Meanwhile `.github/actions/rebaseline-lane` prints
`render-hosted.sh --run <id> --lane <lane>` as the fix on a drifting lane's
neutral check, so a drifting `mc` or `gui` lane handed the reader a command that
dies.

WHAT WENT STALE WAS PROSE, AND A ROSTER ALONE WOULD GO STALE THE SAME WAY. The
helper's header said "all four lanes" and its usage text said
`<kernel|freecad|uv|wild|all>`; each was true when written, each went silently
wrong when a lane landed, and the sweep for that sentence found it again in two
more files. Re-spelling a bigger number, or even a roster, only
resets that clock: the defect is that nothing compares the two lists. This does.

WHAT IT PROVES. Against `.github/workflows/render.yml`'s own declarations — the
`lanes` input's `options`, each lane's `rebaseline-lane` step (`lane:` + `dir:`),
and each lane's `upload-artifact` step (`name:` + `path:`) — the helper's lane
table, printed by `render-hosted.sh --print-lane-table`:

  1. the lane SETS are equal, and both offer `all`;
  2. each lane's ARTIFACT NAME is the one render.yml uploads for it;
  3. each lane's COMMITTED DIRECTORY is the one render.yml re-baselines;
  4. the helper's `RENDER_JOBS_RE` matches every job that uploads a lane
     artifact, and no job that uploads none.

Claim 4 is not cosmetic. That regex decides which jobs the poll waits for, so a
lane job it does not match is a lane the poll will not wait for: the run reads as
settled while that lane is still drawing, and the download then reports a missing
artifact for a lane that was going to produce one. The GUI lane was in exactly
that state — its job is a third job, and the regex named two.

WHAT IT DOES NOT PROVE. Not that a lane RENDERS anything, or renders the right
thing — that is the lane's own gate. Not the ORDER of the roster, which is
cosmetic and deliberately unchecked. Not that `render.yml`'s per-lane `if:`
conditions select the lane the caller asked for (`MC_WANTED` and its siblings are
not read here). Not anything about lane rosters in files this one does not read:
`demos/check_render_provenance.py`'s `LANE_DIRS` is the PNG trees rather than the
lanes, and `local-scripts/ci-local.sh`'s uv+mc drift row is the byte-reproducible
subset — neither is a lane roster and render.yml declares neither property.

WHERE IT IS CONSERVATIVE. A shape it cannot read raises `Bail` and fails the run
rather than passing it: a `lanes` input with no `options` list, a declared lane
with no `rebaseline-lane` step, a re-baselined directory nothing uploads, an
upload whose `name:`/`path:` pair it cannot pair, a job regex outside the small
literal-alternation vocabulary below, or a helper that prints no table. That
direction is a loud failure whose message states the fix — the direction that
gets a checker routed around is the silent one.

Stdlib only, and a line recogniser rather than a YAML parser: the same posture as
`scripts/check-ci-mirror-parity.py` and `scripts/check-status-capture.py`, whose
headers argue it. The helper's half is not parsed at all — it is EXECUTED, so
what is compared is the table the script actually works from, the way
`scripts/doc-gate.sh --print-roots` derives its roots instead of restating them.

  check-render-lane-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import argparse
import contextlib
import io
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

WORKFLOW = ".github/workflows/render.yml"
HELPER = "local-scripts/render-hosted.sh"

JOB_KEY = re.compile(r"^  (?P<id>[A-Za-z0-9_-]+):\s*$")
JOB_NAME = re.compile(r"^    name:\s*(?P<name>\S.*?)\s*$")
# A step begins at the `- ` of its first key, and that key is `name:` as often
# as it is `uses:` — render.yml spells unnamed upload steps the second way.
STEP_START = re.compile(r"^\s+- (?P<key>[a-z]+):\s*(?P<value>.*?)\s*$")
USES = re.compile(r"^\s+uses:\s*(?P<uses>\S+)\s*$")
WITH_KEY = re.compile(r"^\s+(?P<key>[a-z_]+):\s*(?P<value>\S.*?)\s*$")
OPTIONS = re.compile(r"^\s*options:\s*\[(?P<items>[^\]]*)\]\s*$")
LANES_INPUT = re.compile(r"^\s*lanes:\s*$")

REBASELINE_ACTION = "./.github/actions/rebaseline-lane"
UPLOAD_ACTION_PREFIX = "actions/upload-artifact@"

# The job regex is read with Python's `re` and evaluated by jq's in the helper.
# Every character outside this vocabulary raises `Bail`, because agreement
# between the two engines is only claimed for anchored alternations of literal
# job names: `|`, `$`, and `\`-escapes of the characters GitHub puts in a job
# name. A pattern that reaches past it is a pattern this reader must not judge.
JOBS_RE_VOCABULARY = re.compile(r"^(?:\\[+()]|[A-Za-z0-9 +(),.:_|-])*\$$")


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
            if name and name.startswith("renders") and not path:
                raise Bail(f"{WORKFLOW}:{step_line}: upload '{name}' with no `path:`")
            if name and path and path.endswith("/**"):
                uploads.append((name, path[: -len("/**")], job or "?"))
        action, fields = None, {}

    for n, line in enumerate(text.splitlines(), 1):
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


def workflow_job_names(text: str) -> list[str]:
    names = []
    job = None
    for line in text.splitlines():
        m = JOB_KEY.match(line)
        if m:
            job = m.group("id")
            names.append(job)
            continue
        m = JOB_NAME.match(line)
        if m and job:
            names[-1] = m.group("name")
            job = None
    return names


# ------------------------------------------------------------- the helper


def helper_table(root: Path) -> tuple[dict[str, tuple[str, str]], str]:
    """Run the helper's `--print-lane-table` and read what it works from."""
    script = root / HELPER
    proc = subprocess.run(
        ["bash", str(script), "--print-lane-table"],
        capture_output=True,
        text=True,
        cwd=root,
    )
    if proc.returncode != 0:
        raise Bail(
            f"{HELPER} --print-lane-table exited {proc.returncode}: "
            f"{proc.stderr.strip() or proc.stdout.strip()}"
        )
    table: dict[str, tuple[str, str]] = {}
    jobs_re = None
    for line in proc.stdout.splitlines():
        parts = line.split()
        if parts[:1] == ["lane"] and len(parts) == 4:
            table[parts[1]] = (parts[2], parts[3])
        elif parts[:1] == ["jobs-re"] and len(parts) >= 2:
            jobs_re = line.split(" ", 1)[1]
        elif line.strip():
            raise Bail(f"{HELPER} --print-lane-table printed {line!r}, unreadable")
    if not table:
        raise Bail(f"{HELPER} --print-lane-table named no lanes at all")
    if jobs_re is None:
        raise Bail(f"{HELPER} --print-lane-table printed no `jobs-re` line")
    if not JOBS_RE_VOCABULARY.match(jobs_re):
        raise Bail(
            f"{HELPER}: RENDER_JOBS_RE is {jobs_re!r}, outside the anchored "
            "literal-alternation vocabulary this reader can judge two regex "
            "engines to agree on"
        )
    return table, jobs_re


# ------------------------------------------------------------- the claims


def check(root: Path) -> None:
    text = (root / WORKFLOW).read_text()
    declared = workflow_lanes(text)
    table, jobs_re = helper_table(root)

    # 1. the lane sets are equal.
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
        artifact, directory = table[name]
        # 2. artifact names.
        if artifact != lane.artifact:
            raise Fail(
                f"claim 2: lane '{name}' uploads artifact '{lane.artifact}' in "
                f"{WORKFLOW}; {HELPER} downloads '{artifact}'."
            )
        # 3. committed directories.
        if directory != lane.dir:
            raise Fail(
                f"claim 3: lane '{name}' re-baselines '{lane.dir}' in "
                f"{WORKFLOW}; {HELPER} installs into '{directory}'."
            )

    # 4. the poll waits for every lane job, and for nothing else.
    pattern = re.compile(jobs_re)
    for name in sorted(declared):
        job = declared[name].job
        if not pattern.search(job or ""):
            raise Fail(
                f"claim 4: lane '{name}' uploads from job '{job}', which "
                f"{HELPER}'s RENDER_JOBS_RE does not match — the poll would "
                "neither wait for that lane nor report it, and could call the "
                "run settled while it is still drawing."
            )
    lane_jobs = {lane.job for lane in declared.values()}
    for job in workflow_job_names(text):
        if pattern.search(job) and job not in lane_jobs:
            raise Fail(
                f"claim 4: RENDER_JOBS_RE matches job '{job}', which uploads no "
                "lane artifact — the poll would wait on a job that is not a "
                "render lane."
            )

    print(
        f"render lane parity: {len(declared)} lanes "
        f"({', '.join(sorted(declared))}), artifact, directory and poll-job "
        "agreement between render.yml and render-hosted.sh — all as specified."
    )


# ----------------------------------------------------------------- selftest

# Each row mutates a COPY of the two real files and asserts the verdict. The
# GREEN rows matter as much as the red ones: a checker that reds on a correct
# change gets routed around, and then detects nothing at all.
MUTANTS: tuple[tuple[str, str, tuple[tuple[str, str], ...], str | None], ...] = (
    (
        # The lane's own steps outlive the roster entry, so this reds one
        # refusal earlier than claim 1 — at the step that re-baselines a lane
        # the input no longer offers.
        "workflow drops a lane",
        WORKFLOW,
        ((", gui]", "]"),),
        "which the `lanes` input does not declare",
    ),
    (
        "helper carries a lane the workflow does not declare",
        HELPER,
        (
            (
                "gui     renders-gui     demos/renders-gui\n",
                "gui     renders-gui     demos/renders-gui\n"
                "plate   renders-plate   demos/renders-plate\n",
            ),
        ),
        "claim 1",
    ),
    (
        "workflow adds a seventh lane, fully declared",
        WORKFLOW,
        (
            (", gui]", ", gui, plate]"),
            (
                "          name: renders-mc\n          path: demos/renders-mc/**\n",
                "          name: renders-mc\n          path: demos/renders-mc/**\n"
                "      - name: upload plate (plate)\n"
                "        uses: actions/upload-artifact@v4\n"
                "        with:\n"
                "          name: renders-plate\n"
                "          path: demos/renders-plate/**\n",
            ),
            (
                "          lane: mc\n          dir: demos/renders-mc\n",
                "          lane: mc\n          dir: demos/renders-mc\n"
                "      - name: re-baseline committed lane (plate)\n"
                "        uses: ./.github/actions/rebaseline-lane\n"
                "        with:\n"
                "          lane: plate\n"
                "          dir: demos/renders-plate\n",
            ),
        ),
        "claim 1",
    ),
    (
        "workflow renames a lane",
        WORKFLOW,
        ((" mc,", " density,"), ("lane: mc", "lane: density")),
        "claim 1",
    ),
    (
        "helper drops a lane from its table",
        HELPER,
        (("gui     renders-gui     demos/renders-gui\n", ""),),
        "claim 1",
    ),
    (
        "helper's artifact name drifts",
        HELPER,
        (("renders-uv      demos", "renders-uvs     demos"),),
        "claim 2",
    ),
    (
        "helper's committed directory drifts",
        HELPER,
        (("kernel  renders-kernel  demos/renders\n", "kernel  renders-kernel  demos/renders-kernel\n"),),
        "claim 3",
    ),
    (
        "helper's poll regex loses a lane job",
        HELPER,
        (("|viewer gui montage)$", ")$"),),
        "claim 4",
    ),
    (
        "helper's poll regex reaches a job that is not a lane",
        HELPER,
        (("|viewer gui montage)$", "|viewer gui montage|checkout target)$"),),
        "claim 4",
    ),
    (
        "workflow renames a lane's job",
        WORKFLOW,
        (("    name: viewer gui montage", "    name: viewer window montage"),),
        "claim 4",
    ),
    (
        "workflow's lane options list is gone",
        WORKFLOW,
        (("        options: [all, kernel", "        removed: [all, kernel"),),
        "no `options:",
    ),
    (
        "workflow re-baselines a lane nothing uploads",
        WORKFLOW,
        (("          path: demos/renders-mc/**\n", "          path: demos/renders-mc-cells/**\n"),),
        "no step uploads that directory",
    ),
    (
        "helper's lane table is emptied",
        HELPER,
        (
            (
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
        "helper's poll regex is widened past what two engines agree on",
        HELPER,
        (("(scene inputs", "(.*|scene inputs"),),
        "vocabulary",
    ),
    # GREEN: the roster's ORDER is cosmetic and is not a claim.
    (
        "helper's table is reordered",
        HELPER,
        (
            (
                "kernel  renders-kernel  demos/renders\n"
                "freecad renders-freecad demos/renders-freecad\n",
                "freecad renders-freecad demos/renders-freecad\n"
                "kernel  renders-kernel  demos/renders\n",
            ),
        ),
        None,
    ),
    # GREEN: a lane may be named anything; the join is the directory, not the
    # `renders-<lane>` spelling, and the helper is told the same name.
    (
        "a lane's artifact is renamed in both halves at once",
        "both",
        (
            ("name: renders-wild", "name: wild-cells"),
            ("wild    renders-wild ", "wild    wild-cells   "),
        ),
        None,
    ),
)


def selftest(root: Path) -> int:
    rc = 0
    for label, target, edits, expect in MUTANTS:
        with tempfile.TemporaryDirectory() as tmp:
            tree = Path(tmp)
            for rel in (WORKFLOW, HELPER):
                dst = tree / rel
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(root / rel, dst)
            for old, new in edits:
                for rel in (WORKFLOW, HELPER) if target == "both" else (target,):
                    path = tree / rel
                    body = path.read_text()
                    if old in body:
                        path.write_text(body.replace(old, new))
                        break
                else:
                    print(f"  SELFTEST BROKEN  {label}: no site for {old!r}")
                    rc = 1
                    continue
            try:
                # The check's own success line belongs to the tree it ran on,
                # not to this report.
                with contextlib.redirect_stdout(io.StringIO()):
                    check(tree)
                verdict, message = "GREEN", ""
            except (Bail, Fail) as exc:
                verdict, message = "RED", str(exc)
            if expect is None:
                ok = verdict == "GREEN"
            else:
                ok = verdict == "RED" and expect in message
            print(
                f"  {'ok  ' if ok else 'FAIL'}  {label}: {verdict}"
                + (f" ({message.splitlines()[0][:90]})" if message else "")
            )
            if not ok:
                rc = 1
    print(
        f"selftest: {len(MUTANTS)} mutants, each asserted on its own tree — "
        f"{sum(1 for m in MUTANTS if m[3] is None)} of them rows that must stay "
        "GREEN."
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
