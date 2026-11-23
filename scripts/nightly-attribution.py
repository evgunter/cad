#!/usr/bin/env python3
"""Name the merge that turned each newly red nightly test red.

    scripts/nightly-attribution.py --run <run-id> [--junit <dir>] [--archive <file>] [--dry-run]
    scripts/nightly-attribution.py --selftest

<run-id> is a run of `.github/workflows/nightly.yml` that uploaded its
`full-suite-junit-*` artifacts. A test's BASELINE at an eps row is the latest
earlier SCHEDULED run of `main` that reported it there (a dispatched nightly
may run on any branch; a night whose build died reported nothing). A test is
NEWLY RED at a row when it failed there in this run and passed in its
baseline. A test a leg was running when that leg stopped at its budget is
marked failed by nextest without having failed; on either side it counts as
not reported, and this run's are listed as `leg stopped`.

Each newly red test is attributed over EVERY first-parent commit after its
baseline's sha (good) up to this run's sha (bad):

1. Retried once at the bad sha, from `--archive` (the nightly's own test
   binaries) when given. A pass there is `flaky`.
2. One commit in the range: that commit, with no probe.
3. Otherwise a bisection in the PROBE CONFIGURATION: the test's own target
   (`cargo nextest archive -p <crate> --test/--lib ...`) built at the probed
   commit with the job's env, then one nextest call per eps row over every
   open test of that binary. A probe result counts as a failure only when
   nextest reports the failure type the nightly did (a panic's `exit code
   101`); a crash, abort, timeout, missing test or failed build makes the
   commit untestable for that test.

A culprit C is reported only when, in the probe configuration, the test
passed at C^1 and failed at C, and a re-run of both from the same builds
agrees. That covers the ends: a bisect that narrows to the baseline or to
the bad sha probes that end, so a red the probe configuration does not share
ends as a range `(probe config disagrees at the bad end)` or `... good end`,
never as a culprit. Otherwise the answer is a range with why it stopped.

The bisection ASSUMES each test is monotone over the range (green, then red
from one commit on) and does not flake. The retry and the re-runs at C^1 and
C catch a flake that shows within one re-run; a test that went red, green
and red again within the range can still be pinned on the wrong one of its
reds.

It also ASSUMES the probe config and the nightly config turn red at the
same commit. A red-to-green transition in the probe config alone, at
another commit and with the same failure type, would be confirmed and named;
only a nightly-config check at C could rule that out.

Cost order: the binary with most open tests is bisected first and finished
before the next starts, so a spent budget leaves whole answers. Once a
culprit C is confirmed, the other binaries probe C and C^1 first. Bounds:
`--max-probes` builds and `--budget-min` minutes from the script's start;
`--probe-test-s` seconds of tests per nextest call; a test that took over
`--slow-s` at the bad sha, a range of over `--max-range` commits, and tests
past the `--max-tests` cheapest are reported as a range with the reason.
`--dry-run` lists the newly red tests and their ranges; it builds nothing.

Writes `attribution.md` and `attribution.json` into `--out` and the markdown
to `$GITHUB_STEP_SUMMARY`. It exits 0 whatever happens inside it: its own
failure is reported as `attribution failed: <why>`, never as a red test, and
the verdicts reached before it are kept.
"""

import argparse
import json
import os
import re
import signal
import subprocess
import sys
import tempfile
import time
import traceback
import xml.etree.ElementTree as ET
from collections.abc import Callable
from dataclasses import dataclass, field
from pathlib import Path

REPO = os.environ.get("GITHUB_REPOSITORY", "evgunter/cad")
WORKFLOW = "nightly.yml"
JUNIT_PREFIX = "full-suite-junit-"
LEG = re.compile(r"^(?P<row>.+?)(?:-(?P<k>\d+)of(?P<n>\d+))?$")
MERGE = re.compile(r"^Merge pull request #(\d+)")
PANIC = "test failure with exit code 101"
NO_TESTS_RUN = 4
PASS, FAIL = "pass", "fail"
progress = print  # the selftest silences it


class Clock:
    def __init__(self, budget_min: float):
        self.budget = 60 * budget_min
        self.start = time.monotonic()

    def left(self) -> float:
        return self.budget - (time.monotonic() - self.start)

    def used_min(self) -> float:
        return (time.monotonic() - self.start) / 60


# ---- junit ----------------------------------------------------------------


@dataclass
class Case:
    failure: str  # "" when it passed, else nextest's failure type
    time: float
    stopped: bool = False  # marked failed by a leg stopped at its budget

    @property
    def red(self) -> bool:
        return bool(self.failure)


def failure_type(case: ET.Element) -> str:
    for ch in case:
        if ch.tag in ("failure", "error"):
            return ch.get("type") or ch.tag
    return ""


def read_junit_dir(base: Path, leg_budget_s: float) -> dict[str, dict[str, Case]]:
    """{row: {"<binary-id> <test>": Case}} over `<base>/full-suite-junit-<leg>/junit.xml`.

    A leg whose run lasted `leg_budget_s` or more was stopped at its budget;
    its failures other than a panic are the tests it was running then."""
    rows: dict[str, dict[str, Case]] = {}
    for report in sorted(base.glob(f"{JUNIT_PREFIX}*/junit.xml")):
        leg = report.parent.name[len(JUNIT_PREFIX) :]
        cases = rows.setdefault(LEG.match(leg)["row"], {})
        root = ET.parse(report).getroot()  # noqa: S314 — a junit the nightly wrote
        stopped = float(root.get("time") or 0) >= leg_budget_s
        for c in root.iter("testcase"):
            ft = failure_type(c)
            killed = stopped and bool(ft) and ft != PANIC
            cases[f"{c.get('classname')} {c.get('name')}"] = Case(ft, float(c.get("time") or 0), killed)
    return rows


def eps_env(row: str) -> str | None:
    """The CAD_TOLERANCE_EPS value a row ran at (its artifact spells it), None for default."""
    return None if row == "default" else row


# ---- GitHub ---------------------------------------------------------------


def gh_json(path: str) -> dict:
    out = subprocess.run(["gh", "api", path], check=True, capture_output=True, text=True).stdout
    return json.loads(out)


def earlier_scheduled_main(before: str, limit: int) -> list[dict]:
    """The scheduled main nightlies created before `before`, newest first."""
    page = gh_json(f"repos/{REPO}/actions/workflows/{WORKFLOW}/runs?event=schedule&branch=main&per_page=50")
    runs = [
        r
        for r in page["workflow_runs"]
        if r["event"] == "schedule" and r["head_branch"] == "main" and r["created_at"] < before
    ]
    runs.sort(key=lambda r: r["created_at"], reverse=True)
    return runs[:limit]


def download_junit(run: int, dest: Path) -> bool:
    """Download a run's junit artifacts into `dest`; False when it has none."""
    arts = gh_json(f"repos/{REPO}/actions/runs/{run}/artifacts?per_page=100")["artifacts"]
    if not any(a["name"].startswith(JUNIT_PREFIX) and not a["expired"] for a in arts):
        return False
    dest.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["gh", "run", "download", str(run), "-R", REPO, "-p", f"{JUNIT_PREFIX}*", "-D", str(dest)],
        check=True,
        capture_output=True,
    )
    return True


# ---- the newly red set ----------------------------------------------------


@dataclass
class Red:
    test: str  # "<binary-id> <test name>"
    row: str
    time: float  # seconds at the bad sha
    failure: str = PANIC  # the nightly's failure type
    base_run: int = 0
    base_sha: str = ""
    verdict: str = ""
    note: str = ""  # why it is no longer bisected
    base_idx: int = 0
    lo: int = 0  # latest commit index known good
    hi: int = 0  # earliest commit index known bad
    seen: dict[int, list[str]] = field(default_factory=dict)  # probe-config results
    untestable: set[int] = field(default_factory=set)

    @property
    def binary(self) -> str:
        return self.test.split(" ", 1)[0]

    @property
    def name(self) -> str:
        return self.test.split(" ", 1)[1]

    @property
    def open(self) -> bool:
        return not self.verdict and not self.note


def newly_red(
    head: dict[str, dict[str, Case]], bases: list[tuple[int, str, dict]]
) -> tuple[list[Red], int, list[tuple[str, str]]]:
    """(newly red, count of reds no baseline reported, (test, row) a stopped leg held).

    `bases` is (run id, sha, rows) newest first; each red test's baseline is
    the first of them that reported it at its row, not counting a test a
    stopped leg was running."""
    out: list[Red] = []
    unreported = 0
    stopped: list[tuple[str, str]] = []
    for row, cases in sorted(head.items()):
        for test, case in sorted(cases.items()):
            if not case.red:
                continue
            if case.stopped:
                stopped.append((test, row))
                continue
            for run, sha, rows in bases:
                before = rows.get(row, {}).get(test)
                if before is None or before.stopped:
                    continue
                if not before.red:
                    out.append(Red(test, row, case.time, case.failure, run, sha))
                break
            else:
                unreported += 1
    return out, unreported, stopped


# ---- git ------------------------------------------------------------------


def git(*args: str, cwd: Path | None = None) -> str:
    return subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True, text=True).stdout.strip()


def first_parent_range(good: str, bad: str) -> list[str]:
    """`good`, then every first-parent commit after it up to `bad`, oldest first.

    No commit is dropped for the paths it touches: tests read the tracker
    and the docs too."""
    out = git("rev-list", "--first-parent", "--reverse", f"{good}..{bad}")
    return [good, *out.split()]


def label(sha: str) -> str:
    subject = git("log", "-1", "--format=%s", sha)
    m = MERGE.match(subject)
    return f"#{m[1]} ({sha[:10]})" if m else f"{sha[:10]} ({subject[:60]})"


# ---- running tests --------------------------------------------------------


def target_args(binary: str) -> list[str]:
    """Cargo args that build exactly the test target nextest calls `binary`."""
    pkg, _, rest = binary.partition("::")
    if not rest:
        return ["-p", pkg, "--lib"]
    kind, _, name = rest.partition("/")
    if not name:
        return ["-p", pkg, "--test", rest]
    flag = {"bin": "--bin", "example": "--example", "bench": "--bench"}.get(kind)
    if flag is None:
        raise ValueError(f"unknown binary-id kind in {binary!r}")
    return ["-p", pkg, flag, name]


def filterset(reds: list[Red]) -> str:
    return " | ".join(f"(binary_id({r.binary}) & test(={r.name}))" for r in reds)


def sh(cmd: list[str], cwd: Path, env: dict, timeout: float) -> tuple[int | None, str]:
    """(exit code, or None on timeout, and the output's tail). A timeout kills
    the whole process group, test processes included."""
    p = subprocess.Popen(
        cmd, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True
    )
    try:
        out, _ = p.communicate(timeout=max(timeout, 1))
        return p.returncode, out[-3000:]
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGKILL)
        out, _ = p.communicate()
        return None, (out or "")[-3000:]


def verdict_of(r: Red, case: ET.Element | None) -> str:
    """A junit testcase as a probe result for `r`."""
    if case is None:
        return "absent"
    ft = failure_type(case)
    if not ft:
        return PASS
    return FAIL if ft == r.failure else f"failed otherwise ({ft})"


class Runner:
    """Builds and runs tests at the range's commits, in one worktree."""

    def __init__(self, commits: list[str], work: Path, clock: Clock, args):
        self.commits, self.work, self.clock, self.args = commits, work, clock, args
        self.tree = work / "tree"
        self.archives: dict[tuple[int, str], Path | str] = {}  # a path, or why there is none
        self.builds = 0
        self.last_build_s = 0.0
        work.mkdir(parents=True, exist_ok=True)
        subprocess.run(["git", "worktree", "remove", "--force", str(self.tree)], check=False, capture_output=True)
        git("worktree", "add", "--detach", str(self.tree), commits[-1])

    def close(self) -> None:
        subprocess.run(["git", "worktree", "remove", "--force", str(self.tree)], check=False, capture_output=True)

    def built(self, idx: int, binary: str) -> bool:
        return (idx, binary) in self.archives

    def checkout(self, idx: int) -> None:
        git("checkout", "--quiet", "--force", "--detach", self.commits[idx], cwd=self.tree)

    def build(self, idx: int, binary: str) -> Path | str:
        self.checkout(idx)
        self.builds += 1
        dest = self.work / f"{idx}-{binary.replace(':', '_').replace('/', '_')}.tar.zst"
        cmd = ["cargo", "nextest", "archive", *target_args(binary), "--archive-file", str(dest)]
        t0 = time.monotonic()
        rc, tail = sh(cmd, self.tree, dict(os.environ), self.clock.left())
        if rc not in (0, None) and "SIGKILL" in tail:
            # A rustc killed for memory says nothing about the commit.
            print(f"  {binary}: a rustc was killed (memory?); retrying at -j 2", flush=True)
            rc, tail = sh([*cmd, "-j", "2"], self.tree, dict(os.environ), self.clock.left())
        self.last_build_s = time.monotonic() - t0
        if rc == 0:
            mb = dest.stat().st_size >> 20
            print(f"  built {binary} at {self.commits[idx][:10]} in {self.last_build_s:.0f} s ({mb} MB)", flush=True)
            got: Path | str = dest
        else:
            got = "its build timed out" if rc is None else f"its build exited {rc}"
            print(f"  {binary} at {self.commits[idx][:10]}: {got} after {self.last_build_s:.0f} s\n{tail}", flush=True)
        self.archives[(idx, binary)] = got
        return got

    def prune(self, binary: str, keep: set[int]) -> None:
        """Delete `binary`'s archives but those at `keep`: a test binary is
        gigabytes, and only an open range's two ends can still be re-run."""
        for (idx, b), a in list(self.archives.items()):
            if b == binary and idx not in keep and isinstance(a, Path):
                a.unlink(missing_ok=True)
                self.archives[(idx, b)] = "its build was pruned"

    def run(self, idx: int, binary: str, reds: list[Red]) -> dict[int, str]:
        """{id(red): result} for `reds` (one binary) at commit `idx`."""
        archive = self.archives.get((idx, binary)) or self.build(idx, binary)
        if not isinstance(archive, Path):
            return {id(r): archive for r in reds}
        self.checkout(idx)
        return self.run_archive(archive, reds)

    def run_archive(self, archive: Path, reds: list[Red]) -> dict[int, str]:
        """Run `reds` from a nextest archive against the worktree as it stands:
        one nextest call per eps row, read back from its junit."""
        got: dict[int, str] = {}
        junit = self.tree / "target" / "nextest" / "ci" / "junit.xml"
        for row in sorted({r.row for r in reds}):
            these = [r for r in reds if r.row == row]
            env = dict(os.environ)
            env.pop("CAD_TOLERANCE_EPS", None)
            if eps_env(row):
                env["CAD_TOLERANCE_EPS"] = eps_env(row)
            cmd = [
                "cargo", "nextest", "run", "--archive-file", str(archive), "--workspace-remap", str(self.tree),
                "--profile", "ci", "--ignore-default-filter", "--no-fail-fast", "-E", filterset(these),
            ]  # fmt: skip
            want = min(sum(max(120.0, 4 * r.time) for r in these), self.args.probe_test_s, self.clock.left())
            junit.unlink(missing_ok=True)
            rc, tail = sh(cmd, self.tree, env, want)
            if rc is None:
                for r in these:
                    got[id(r)] = f"timed out ({want:.0f} s for {len(these)} test(s))"
                continue
            cases = {}
            if junit.exists():
                root = ET.parse(junit).getroot()  # noqa: S314 — nextest's own report
                cases = {f"{c.get('classname')} {c.get('name')}": c for c in root.iter("testcase")}
            elif rc != NO_TESTS_RUN:
                print(tail, flush=True)
                for r in these:
                    got[id(r)] = f"nextest exited {rc} with no report"
                continue
            for r in these:
                got[id(r)] = verdict_of(r, cases.get(r.test))
        return got


# ---- the bisection --------------------------------------------------------


def wants(r: Red, suspects: list[int]) -> list[int]:
    """The commits `r` would take a probe at now, best first (empty: none)."""
    cands = [i for i in range(r.lo + 1, r.hi) if i not in r.untestable]
    if cands:
        first = [s for s in suspects if s in cands]
        mid = (r.lo + r.hi) // 2
        return first + sorted((i for i in cands if i not in first), key=lambda i: (abs(i - mid), i))
    if r.hi - r.lo != 1:
        return []
    # Adjacent: both ends observed in the probe config, then each once more,
    # unless an observation already disagrees.
    hi, lo = r.seen.get(r.hi, []), r.seen.get(r.lo, [])
    if any(x != FAIL for x in hi) or any(x != PASS for x in lo):
        return []
    for end, seen in ((r.hi, hi), (r.lo, lo)):
        if not seen:
            return [end]
    return [end for end, seen in ((r.hi, hi), (r.lo, lo)) if len(seen) < 2]


def judge(r: Red, n_last: int, label_of: Callable[[int], str]) -> None:
    """Settle `r` once it wants no further probe."""
    if not r.open or wants(r, []):
        return
    lo, hi = r.seen.get(r.lo, []), r.seen.get(r.hi, [])
    if r.hi - r.lo != 1:
        r.note = "every commit left between is untestable"
    elif lo and hi and all(x == PASS for x in lo) and all(x == FAIL for x in hi):
        r.verdict = f"introduced by {label_of(r.hi)}"
    else:
        why = []
        if any(x != FAIL for x in hi):
            why.append(end_note("the bad end" if r.hi == n_last else "the culprit", hi, FAIL))
        if any(x != PASS for x in lo):
            why.append(end_note("the good end" if r.lo == r.base_idx else "its parent", lo, PASS))
        r.note = "; ".join(why)


def end_note(end: str, got: list[str], want: str) -> str:
    """Why `end` did not give `want`. Only a test result that differs is a
    disagreement; a build that failed or was cut leaves the end untestable."""
    if any(x != want and not x.startswith("its build") for x in got):
        return f"probe config disagrees at {end} ({', '.join(got)})"
    builds = sorted({x.removeprefix("its ") for x in got if x.startswith("its build")})
    return f"untestable at {end} ({', '.join(builds) or 'not probed'})"


def settle(r: Red, commits: list[str]) -> None:
    if r.verdict:
        return
    n = r.hi - r.lo
    r.verdict = (
        f"range {commits[r.lo][:10]}..{commits[r.hi][:10]} ({n} commit{'s' * (n != 1)}; {r.note or 'not reached'})"
    )


def bisect(reds: list[Red], commits: list[str], runner, clock: Clock, args, label_of, log: list[str]) -> None:
    """Probe until every red is settled or a bound stops it."""
    n_last = len(commits) - 1
    suspects: list[int] = []
    current = None
    while True:
        for r in reds:
            judge(r, n_last, label_of)
            if r.verdict.startswith("introduced by") and r.hi not in suspects:
                suspects[:0] = [r.hi, r.hi - 1]
        live = [r for r in reds if r.open and wants(r, suspects)]
        if not live:
            return
        groups: dict[str, list[Red]] = {}
        for r in live:
            groups.setdefault(r.binary, []).append(r)
        if current not in groups:
            if current is not None:
                runner.prune(current, set())
            current = min(groups, key=lambda b: (-len(groups[b]), b))
        group = sorted(groups[current], key=lambda r: (r.hi - r.lo, r.time, r.row))
        at = wants(group[0], suspects)[0]
        these = [r for r in group if at in wants(r, suspects)]
        needs_build = not runner.built(at, current)
        stop = ""
        if needs_build and runner.builds >= args.max_probes:
            stop = f"the {args.max_probes}-build cap was reached"
        elif clock.left() < max(120, 1.2 * runner.last_build_s if needs_build else 0):
            stop = f"the {args.budget_min:.0f}-minute budget was spent"
        if stop:
            for r in live:
                r.note = stop
            return
        what = "probe" if needs_build else "re-run"
        progress(f"{what} {current} at {label_of(at)}: {len(these)} test(s)", flush=True)
        got = runner.run(at, current, these)
        for r in these:
            res = got[id(r)]
            log.append(f"{what} {commits[at][:10]} {r.test} @ {r.row}: {res}")
            r.seen.setdefault(at, []).append(res)
            if r.lo < at < r.hi:
                if res == PASS:
                    r.lo = at
                elif res == FAIL:
                    r.hi = at
                else:
                    r.untestable.add(at)
        runner.prune(current, {i for r in reds if r.open and r.binary == current for i in (r.lo, r.hi)})


def attribute(reds: list[Red], args, clock: Clock, out: Path, log: list[str]) -> None:
    """Fill each red's verdict. The verdicts reached are kept whatever raises."""
    head = args.head_sha
    widest = max({r.base_sha for r in reds}, key=lambda s: int(git("rev-list", "--count", f"{s}..{head}")))
    commits = first_parent_range(widest, head)
    index = {c: i for i, c in enumerate(commits)}
    labels: dict[int, str] = {}

    def label_of(i: int) -> str:
        if i not in labels:
            labels[i] = label(commits[i])
        return labels[i]

    for r in reds:
        r.hi = len(commits) - 1
        if r.base_sha not in index:
            r.lo = 0
            r.note = f"the baseline {r.base_sha[:10]} is not on {head[:10]}'s first-parent chain"
            continue
        r.base_idx = r.lo = index[r.base_sha]
        n = r.hi - r.lo
        try:
            target_args(r.binary)
        except ValueError as e:
            r.note = str(e)
        if n > args.max_range:
            r.note = f"{n} commits, over the {args.max_range}-commit cap"
        elif r.time > args.slow_s:
            r.note = f"the test took {r.time:.0f} s at the bad sha, over the {args.slow_s:.0f} s cap"
    for i, r in enumerate(sorted((r for r in reds if r.open), key=lambda r: (r.time, r.test, r.row))):
        if i >= args.max_tests:
            r.note = f"past the {args.max_tests} cheapest tests"
    log.append(f"{len(commits) - 1} first-parent commits in the widest range {widest[:10]}..{head[:10]}")
    if args.dry_run:
        for r in reds:
            n = r.hi - r.lo
            if n == 1:
                plan = "one commit, no probe"
            elif r.note:
                plan = f"skipped: {r.note}"
            else:
                plan = f"bisect, about {max(1, (n - 1).bit_length())} probe builds"
            log.append(f"- {r.test} @ {r.row}: {n} commit(s) after {r.base_sha[:10]} (run {r.base_run}); {plan}")
        for r in reds:
            settle(r, commits)
        return

    runner = None
    try:
        runner = Runner(commits, out / "probe", clock, args)
        if args.archive:
            todo = [r for r in reds if r.open]
            runner.checkout(len(commits) - 1)
            got = runner.run_archive(Path(args.archive), todo)
            for r in todo:
                res = got[id(r)]
                log.append(f"retry {r.test} @ {r.row} at {head[:10]} from the nightly's archive: {res}")
                if res == PASS:
                    r.verdict = "flaky (passed on retry at the bad sha)"
                elif res != FAIL:
                    r.note = f"the retry at the bad sha gave: {res}"
        for r in reds:
            if r.open and r.hi - r.lo == 1:
                r.verdict = f"introduced by {label_of(r.hi)} (the only commit between the two nightlies)"
        bisect(reds, commits, runner, clock, args, label_of, log)
    except Exception as e:
        for r in reds:
            if r.open:
                r.note = f"stopped by an error ({type(e).__name__})"
        raise
    finally:
        if runner is not None:
            log.append(f"{runner.builds} probe build(s); {clock.used_min():.1f} of {args.budget_min:.0f} minutes used")
            runner.close()
        for r in reds:
            settle(r, commits)


# ---- report ---------------------------------------------------------------


def report(run, head_sha: str, reds: list[Red], unreported: int, stopped, log: list[str], failed: str) -> str:
    lines = [f"## Newly red tests in nightly run {run} ({head_sha[:10]})", ""]
    if failed:
        lines += [f"**attribution failed: {failed}**", ""]
    if reds or stopped:
        lines += ["| test | eps | verdict |", "|---|---|---|"]
        for r in sorted(reds, key=lambda r: (r.verdict, r.test, r.row)):
            lines.append(f"| `{r.test}` | {r.row} | {r.verdict or 'not attributed'} |")
        for test, row in stopped:
            lines.append(f"| `{test}` | {row} | leg stopped (its leg hit its budget while this test ran) |")
    elif not failed:
        lines.append("No test is red here that passed in its baseline.")
    if unreported:
        lines += [
            "",
            f"{unreported} red test(s) no earlier scheduled main nightly reported at their row: not compared.",
        ]
    if log:
        lines += ["", "<details><summary>log</summary>", "", *[f"    {line}" for line in log], "", "</details>"]
    return "\n".join(lines) + "\n"


def main(args) -> int:
    clock = Clock(args.budget_min)
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    reds: list[Red] = []
    stopped: list = []
    unreported = 0
    log: list[str] = []
    failed = ""
    head_sha = ""
    try:
        info = gh_json(f"repos/{REPO}/actions/runs/{args.run}")
        head_sha = args.head_sha = info["head_sha"]
        log.append(f"run {args.run}: {info['event']} on {info['head_branch']} at {head_sha[:10]}")
        budget_s = 60 * args.leg_budget_min
        dl = Path(tempfile.mkdtemp(prefix="attrib-junit-"))
        head_dir = Path(args.junit) if args.junit else dl / str(args.run)
        if not args.junit and not download_junit(args.run, head_dir):
            raise RuntimeError(f"run {args.run} uploaded no junit report")
        head = read_junit_dir(head_dir, budget_s)
        if not head:
            raise RuntimeError(f"no junit report under {head_dir}: this run's full-suite rows uploaded none")
        bases: list[tuple[int, str, dict]] = []
        for prev in earlier_scheduled_main(info["created_at"], args.lookback):
            d = dl / str(prev["id"])
            try:
                have = download_junit(prev["id"], d)
            except (subprocess.CalledProcessError, OSError, KeyError) as e:
                log.append(f"run {prev['id']}: its junit could not be fetched ({type(e).__name__}): skipped")
                continue
            if have:
                bases.append((prev["id"], prev["head_sha"], read_junit_dir(d, budget_s)))
                log.append(f"baseline candidate: run {prev['id']} at {prev['head_sha'][:10]} ({prev['created_at']})")
            else:
                log.append(f"run {prev['id']} ({prev['created_at']}) has no junit report: skipped")
        if not bases:
            raise RuntimeError(f"no earlier scheduled main nightly in the last {args.lookback} has a junit report")
        reds, unreported, stopped = newly_red(head, bases)
        if args.only:
            kept = [r for r in reds if re.search(args.only, r.test)]
            log.append(f"--only {args.only!r}: {len(kept)} of {len(reds)} newly red test(s) kept")
            reds = kept
        # A shallow or stale checkout lacks the endpoints; a full one has them.
        subprocess.run(["git", "fetch", "--quiet", "origin", head_sha, *{r.base_sha for r in reds}], check=False)
        if reds:
            attribute(reds, args, clock, out, log)
    except Exception as e:  # noqa: BLE001 — reported, never raised
        traceback.print_exc()
        failed = f"{type(e).__name__}: {e}"
    md = report(args.run, head_sha or "?", reds, unreported, stopped, log, failed)
    print(md)
    (out / "attribution.md").write_text(md)
    record = {
        "run": args.run,
        "head_sha": head_sha,
        "failed": failed,
        "newly_red": [
            {"test": r.test, "eps": r.row, "baseline_run": r.base_run, "baseline_sha": r.base_sha, "verdict": r.verdict}
            for r in reds
        ],
        "leg_stopped": [{"test": t, "eps": row} for t, row in stopped],
    }
    (out / "attribution.json").write_text(json.dumps(record, indent=2))
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a") as f:
            f.write(md)
    return 0


# ---- selftest -------------------------------------------------------------


def _junit(d: Path, leg: str, cases: dict[str, str], time_s: float = 100) -> None:
    (d / f"{JUNIT_PREFIX}{leg}").mkdir(parents=True)
    body = "".join(
        f'<testcase classname="{t.split()[0]}" name="{t.split()[1]}" time="1.0">'
        + (f"<failure type='{ft}' message='x'/>" if ft else "")
        + "</testcase>"
        for t, ft in cases.items()
    )
    (d / f"{JUNIT_PREFIX}{leg}" / "junit.xml").write_text(
        f'<testsuites time="{time_s}"><testsuite>{body}</testsuite></testsuites>'
    )


class _FakeRunner:
    """A history in which test `t` is red from commit `red_from[t]` on.
    `config[(t, commit)]` is a list of results that override it in turn;
    the `boom`-th build raises."""

    def __init__(self, red_from: dict[str, int], config=None, boom: int | None = None):
        self.red_from, self.config, self.boom = red_from, config or {}, boom
        self.builds = 0
        self.last_build_s = 0.0
        self.have: set[tuple[int, str]] = set()
        self.reruns = 0

    def built(self, idx, binary):
        return (idx, binary) in self.have

    def prune(self, binary, keep):
        self.have -= {(i, b) for i, b in self.have if b == binary and i not in keep}

    def run(self, idx, binary, reds):
        if (idx, binary) in self.have:
            self.reruns += 1
        else:
            self.builds += 1
            if self.builds == self.boom:
                raise RuntimeError("planted")
            self.have.add((idx, binary))
        got = {}
        for r in reds:
            queued = self.config.get((r.test, idx))
            res = queued.pop(0) if queued else None
            got[id(r)] = res or (FAIL if idx >= self.red_from[r.test] else PASS)
        return got


class _Args:
    max_probes, budget_min = 30, 60


def _bisect(red_from: dict[str, int], config=None, boom=None, n=16):
    reds = [Red(t, "1e-12", 1.0, lo=0, hi=n) for t in red_from]
    commits = [f"{i:040x}" for i in range(n + 1)]
    runner = _FakeRunner(red_from, config, boom)
    try:
        bisect(reds, commits, runner, Clock(60), _Args, lambda i: f"#{i}", [])
    except RuntimeError:
        for r in reds:
            if r.open:
                r.note = "stopped by an error"
    for r in reds:
        settle(r, commits)
    return {r.test: r for r in reds}, runner


def _git_selftest(t: Path) -> None:
    t.mkdir()

    def g(*a):
        return subprocess.run(["git", *a], cwd=t, check=True, capture_output=True, text=True).stdout.strip()

    g("init", "-q", "-b", "main")
    g("config", "user.email", "selftest@example.invalid")
    g("config", "user.name", "selftest")
    shas = []
    for path in ("crates/a.rs", "work/ciw/row.md", "docs/perf-data/x.json", "crates/b.rs"):
        (t / path).parent.mkdir(parents=True, exist_ok=True)
        (t / path).write_text(path)
        g("add", path)
        g("commit", "-q", "-m", path)
        shas.append(g("rev-parse", "HEAD"))
    cwd = os.getcwd()
    os.chdir(t)
    try:
        got = first_parent_range(shas[0], shas[-1])
    finally:
        os.chdir(cwd)
    assert got == shas, f"every first-parent commit, work/- and docs/-only ones included: {got} != {shas}"


def selftest() -> int:
    global progress
    progress = lambda *a, **k: None  # noqa: E731
    with tempfile.TemporaryDirectory() as t:
        t = Path(t)
        _junit(t / "head", "default", {"a::all x": PANIC, "a::all y": PANIC, "b z": "", "c new": PANIC, "d s": ""})
        _junit(t / "head", "1e-12-1of2", {"a::all x": PANIC, "d s": "test abort with signal 9"}, time_s=6400)
        _junit(t / "head", "1e-12-2of2", {"b z": PANIC})
        _junit(t / "n1", "default", {"a::all x": "", "a::all y": PANIC})
        _junit(t / "n1", "1e-12-1of2", {"a::all x": "test abort with signal 9"}, time_s=6400)
        _junit(t / "n2", "default", {"b z": ""})
        _junit(t / "n2", "1e-12-1of2", {"a::all x": ""})
        _junit(t / "n2", "1e-12-2of2", {"b z": ""})
        head = read_junit_dir(t / "head", 6300)
        assert set(head) == {"default", "1e-12"}, f"shards fold into one row: {set(head)}"
        bases = [(1, "s1", read_junit_dir(t / "n1", 6300)), (2, "s2", read_junit_dir(t / "n2", 6300))]
        reds, unreported, stopped = newly_red(head, bases)
        got = {(r.test, r.row, r.base_run) for r in reds}
        # x @ default: green at n1. x @ 1e-12: n1's stopped leg was running it,
        # so its baseline is n2, green. y: red before. z @ 1e-12: n1 lacks the
        # row, n2 has it green. c: in no baseline. d @ 1e-12: this run's
        # stopped leg was running it.
        want = {("a::all x", "default", 1), ("a::all x", "1e-12", 2), ("b z", "1e-12", 2)}
        assert got == want, f"newly red: {got}"
        assert unreported == 1, f"unreported: {unreported}"
        assert stopped == [("d s", "1e-12")], f"leg stopped: {stopped}"
        _git_selftest(t / "repo")
    assert target_args("topo") == ["-p", "topo", "--lib"]
    assert target_args("sweep::all") == ["-p", "sweep", "--test", "all"]
    assert target_args("pncad::bin/pncad") == ["-p", "pncad", "--bin", "pncad"]
    assert eps_env("default") is None and eps_env("1e-12") == "1e-12"

    # Monotone, red from commit 11 of 16: pass at 10 and fail at 11, each re-run.
    by, run = _bisect({"a::all x": 11})
    r = by["a::all x"]
    assert r.verdict == "introduced by #11", r.verdict
    assert r.seen[10] == [PASS, PASS] and r.seen[11] == [FAIL, FAIL], r.seen
    assert run.reruns == 2, f"the confirming re-runs reuse the builds: {run.reruns}"
    # The culprit's re-run passes: a range, never a culprit.
    by, _ = _bisect({"a::all x": 11}, {("a::all x", 11): [FAIL, PASS]})
    assert "disagrees at the culprit" in by["a::all x"].verdict, by["a::all x"].verdict
    # Passes everywhere in the probe config: the bad end is probed and disagrees.
    by, _ = _bisect({"a::all x": 99})
    assert "disagrees at the bad end" in by["a::all x"].verdict, by["a::all x"].verdict
    # Fails everywhere in the probe config: the good end disagrees.
    by, _ = _bisect({"a::all x": 0})
    assert "disagrees at the good end" in by["a::all x"].verdict, by["a::all x"].verdict
    # A failed build at an end leaves it untestable; only differing results disagree.
    assert end_note("the bad end", ["its build exited 101"], FAIL) == "untestable at the bad end (build exited 101)"
    assert end_note("the good end", [FAIL], PASS) == "probe config disagrees at the good end (fail)"
    # An abort at the culprit is not a failure: the commit is untestable.
    by, _ = _bisect({"a::all x": 11}, {("a::all x", 11): ["failed otherwise (test abort)"] * 3})
    v = by["a::all x"].verdict
    assert v.startswith("range") and "untestable" in v, v
    # The binary with most open tests goes first; the next tries C and C^1 first.
    by, _ = _bisect({"a::all x": 11, "a::all x2": 11, "b y": 11})
    assert by["b y"].verdict == "introduced by #11", by["b y"].verdict
    assert sorted(by["b y"].seen) == [10, 11], f"b probed only C^1 and C: {sorted(by['b y'].seen)}"
    # An error part-way keeps the verdict already reached.
    by, _ = _bisect({"a::all x": 11, "b y": 3}, boom=7)
    assert by["a::all x"].verdict == "introduced by #11", by["a::all x"].verdict
    assert "stopped by an error" in by["b y"].verdict, by["b y"].verdict
    print(
        "nightly-attribution selftest OK: shards fold into a row; the baseline is the newest earlier report of the "
        "test at its row, a stopped leg's killed tests counting as unreported on both sides; work/- and docs/-only "
        "commits stay in the range; a culprit needs a pass at C^1 and a fail at C, each re-run; a red the probe "
        "config does not share ends as a range at the end that disagrees; an abort is not a failure; the next "
        "binary tries C and C^1 first; an error keeps the verdicts already reached"
    )
    return 0


def parse(argv: list[str]):
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--selftest", action="store_true")
    p.add_argument("--run", type=int, help="the nightly run to attribute")
    p.add_argument("--junit", help="its junit artifacts, already downloaded (else fetched)")
    p.add_argument("--archive", help="that run's nextest archive, for the retry at the bad sha")
    p.add_argument("--out", default="attribution")
    p.add_argument("--dry-run", action="store_true")
    p.add_argument("--only", help="attribute only the newly red tests matching this regex")
    p.add_argument("--lookback", type=int, default=7, help="earlier scheduled main nightlies searched for a baseline")
    p.add_argument("--leg-budget-min", type=float, default=104, help="a leg that ran this long was stopped")
    p.add_argument("--max-probes", type=int, default=12)
    p.add_argument("--budget-min", type=float, default=60)
    p.add_argument("--probe-test-s", type=float, default=900)
    p.add_argument("--slow-s", type=float, default=300)
    p.add_argument("--max-range", type=int, default=200)
    p.add_argument("--max-tests", type=int, default=25)
    a = p.parse_args(argv)
    if not a.selftest and a.run is None:
        p.error("--run is required")
    return a


if __name__ == "__main__":
    a = parse(sys.argv[1:])
    sys.exit(selftest() if a.selftest else main(a))
