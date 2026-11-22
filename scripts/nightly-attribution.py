#!/usr/bin/env python3
"""Name the merge that turned each newly red nightly test red.

    scripts/nightly-attribution.py --run <run-id> [--archive <file>] [--dry-run]
    scripts/nightly-attribution.py --selftest

<run-id> is a nightly run of `.github/workflows/nightly.yml` whose
`full-suite-junit-*` artifacts are uploaded. Its baseline, per eps row and
test, is the latest earlier SCHEDULED nightly of `main` that reported that
test at that row: a dispatched nightly can run on any branch, and a night
whose build died reported nothing. A test is NEWLY RED at a row when it
failed there in this run and passed in its baseline.

Each newly red test is attributed over the first-parent commits between its
baseline's sha (good) and this run's (bad):

1. Retried once at the bad sha: from `--archive` (the nightly's own test
   binaries) when given, else from a per-package build. A pass there is
   `flaky`, and the test is not bisected.
2. One commit in the range: that commit, no probe.
3. Otherwise a bisection that builds only the packages and test targets of
   the tests still open at the probed commit and runs only those tests,
   each at the eps of the row that went red. One probe serves every test
   whose open range holds the probed commit, so tests that went red
   together are bisected together. A commit where a test's target does not
   build or the test is absent or times out is skipped for that test.

Bounds: `--max-probes` probes and `--budget-min` minutes in all; a test that
took over `--slow-s` seconds at the bad sha, a range of over `--max-range`
commits, and tests past the first `--max-tests` are reported as a range with
the reason. `--dry-run` prints the newly red tests, their ranges and the
probes it would start with, and builds and runs nothing.

Writes `attribution.md` and `attribution.json` into `--out` and the markdown
to `$GITHUB_STEP_SUMMARY`. It exits 0 whatever happens inside it: a failure
of its own is reported as `attribution failed: <why>`, never as a red test.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import traceback
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from pathlib import Path

REPO = os.environ.get("GITHUB_REPOSITORY", "evgunter/cad")
WORKFLOW = "nightly.yml"
JUNIT_PREFIX = "full-suite-junit-"
LEG = re.compile(r"^(?P<row>.+?)(?:-(?P<k>\d+)of(?P<n>\d+))?$")
MERGE = re.compile(r"^Merge pull request #(\d+)")
NO_TESTS_RUN = 4
TEST_FAILED = 100


# ---- junit ----------------------------------------------------------------


@dataclass
class Case:
    red: bool
    time: float


def read_junit_dir(base: Path) -> dict[str, dict[str, Case]]:
    """{row: {"<binary-id> <test>": Case}} over `<base>/full-suite-junit-<leg>/junit.xml`."""
    rows: dict[str, dict[str, Case]] = {}
    for report in sorted(base.glob(f"{JUNIT_PREFIX}*/junit.xml")):
        leg = report.parent.name[len(JUNIT_PREFIX) :]
        row = LEG.match(leg)["row"]
        cases = rows.setdefault(row, {})
        root = ET.parse(report).getroot()  # noqa: S314 — a junit the nightly wrote
        for c in root.iter("testcase"):
            red = any(ch.tag in ("failure", "error") for ch in c)
            cases[f"{c.get('classname')} {c.get('name')}"] = Case(red, float(c.get("time") or 0))
    return rows


def eps_env(row: str) -> str | None:
    """The CAD_TOLERANCE_EPS value a row ran at (its artifact spells it), None for default."""
    return None if row == "default" else row


# ---- GitHub ---------------------------------------------------------------


def gh_json(path: str) -> dict:
    out = subprocess.run(["gh", "api", path], check=True, capture_output=True, text=True).stdout
    return json.loads(out)


def run_info(run: int) -> dict:
    return gh_json(f"repos/{REPO}/actions/runs/{run}")


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
    base_run: int = 0
    base_sha: str = ""
    verdict: str = ""
    lo: int = -1  # index of the latest known-good commit (-1: the baseline sha)
    hi: int = 0  # index of the earliest known-bad commit
    untestable: set[int] = field(default_factory=set)
    note: str = ""

    @property
    def binary(self) -> str:
        return self.test.split(" ", 1)[0]

    @property
    def name(self) -> str:
        return self.test.split(" ", 1)[1]


def newly_red(head: dict[str, dict[str, Case]], bases: list[tuple[int, str, dict]]) -> tuple[list[Red], int]:
    """(newly red tests, count of red tests with no earlier report to compare).

    `bases` is (run id, sha, rows) newest first; each red test's baseline is
    the first of them that reported it at its row."""
    out: list[Red] = []
    unreported = 0
    for row, cases in sorted(head.items()):
        for test, case in sorted(cases.items()):
            if not case.red:
                continue
            for run, sha, rows in bases:
                before = rows.get(row, {}).get(test)
                if before is None:
                    continue
                if not before.red:
                    out.append(Red(test, row, case.time, run, sha))
                break
            else:
                unreported += 1
    return out, unreported


# ---- git ------------------------------------------------------------------


def git(*args: str, cwd: Path | None = None) -> str:
    return subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True, text=True).stdout.strip()


INERT = ("work/", "docs/perf-data/")


def first_parent_range(good: str, bad: str) -> list[str]:
    """First-parent commits after `good` up to `bad`, oldest first, less those
    that touch only the tracker or the perf history (INERT): a test cannot
    read either, and they are half of a day's first-parent commits."""
    out = git("rev-list", "--first-parent", "--reverse", f"{good}..{bad}")
    keep = []
    for c in out.split():
        touched = git("diff", "--name-only", f"{c}^1", c).splitlines()
        if c == bad or any(not t.startswith(INERT) for t in touched):
            keep.append(c)
    return keep


def label(sha: str) -> str:
    subject = git("log", "-1", "--format=%s", sha)
    m = MERGE.match(subject)
    return f"#{m[1]} ({sha[:10]})" if m else f"{sha[:10]} ({subject[:60]})"


# ---- running a test -------------------------------------------------------


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
        raise ValueError(f"unknown binary id kind in {binary!r}")
    return ["-p", pkg, flag, name]


def filterset(red: Red) -> str:
    return f"binary_id({red.binary}) & test(={red.name})"


class Clock:
    def __init__(self, budget_min: float):
        self.end = time.monotonic() + 60 * budget_min

    def left(self) -> float:
        return self.end - time.monotonic()


def sh(cmd: list[str], cwd: Path, env: dict, timeout: float) -> tuple[int | None, str]:
    """(exit code or None on timeout, the output's tail)."""
    try:
        p = subprocess.run(cmd, cwd=cwd, env=env, timeout=max(timeout, 1), capture_output=True, text=True)
        return p.returncode, (p.stdout + p.stderr)[-2000:]
    except subprocess.TimeoutExpired as e:
        out = (e.stdout or b"") + (e.stderr or b"")
        return None, out.decode(errors="replace")[-2000:] if isinstance(out, bytes) else str(out)[-2000:]


def test_env(red: Red) -> dict:
    env = dict(os.environ)
    env.pop("CAD_TOLERANCE_EPS", None)
    eps = eps_env(red.row)
    if eps:
        env["CAD_TOLERANCE_EPS"] = eps
    return env


def run_one(red: Red, cwd: Path, extra: list[str], clock: Clock, test_timeout: float) -> str:
    """'pass' | 'fail' | 'absent' | 'timeout' | 'error:<code>' for one test."""
    cmd = [
        "cargo",
        "nextest",
        "run",
        "--profile",
        "ci",
        "--ignore-default-filter",
        "--no-fail-fast",
        *extra,
        "-E",
        filterset(red),
    ]
    rc, tail = sh(cmd, cwd, test_env(red), min(test_timeout, clock.left()))
    if rc == 0:
        return "pass"
    if rc == TEST_FAILED:
        return "fail"
    if rc == NO_TESTS_RUN:
        return "absent"
    if rc is None:
        return "timeout"
    print(tail, file=sys.stderr)
    return f"error:{rc}"


def build(binaries: set[str], cwd: Path, clock: Clock) -> dict[str, str]:
    """Build each test target at `cwd`; {binary: '' if built, else why not}."""
    why: dict[str, str] = {}
    for b in sorted(binaries):
        t0 = time.monotonic()
        rc, tail = sh(["cargo", "test", "--no-run", *target_args(b)], cwd, dict(os.environ), clock.left())
        took = time.monotonic() - t0
        if rc == 0:
            why[b] = ""
            print(f"  built {b} in {took:.0f} s", flush=True)
        else:
            why[b] = "build timed out" if rc is None else f"build exited {rc}"
            print(f"  {b}: {why[b]} after {took:.0f} s\n{tail}", flush=True)
    return why


def test_timeout(red: Red) -> float:
    return max(120.0, 4 * red.time)


# ---- attribution ----------------------------------------------------------


def choose_probe(open_reds: list[Red]) -> tuple[int, str] | None:
    """(commit index, test binary) to probe next: the middle of the narrowest
    open range, so one binary's bisect finishes before the next starts and a
    spent budget leaves whole answers, not every range half-narrowed. A probe
    builds that one binary; commits found untestable for a test are avoided."""
    for red in sorted(open_reds, key=lambda r: (r.hi - r.lo, r.binary, r.row)):
        mid = (red.lo + red.hi) // 2
        cands = [i for i in range(red.lo + 1, red.hi) if i not in red.untestable]
        if cands:
            return min(cands, key=lambda i: (abs(i - mid), i)), red.binary
    return None


def is_open(red: Red) -> bool:
    return not red.verdict and any(i not in red.untestable for i in range(red.lo + 1, red.hi))


def settle(red: Red, commits: list[str]) -> None:
    if red.verdict:
        return
    if red.hi - red.lo == 1:
        red.verdict = f"introduced by {label(commits[red.hi])}"
    else:
        start = git("rev-parse", "--short=10", red.base_sha) if red.lo < 0 else commits[red.lo][:10]
        red.verdict = (
            f"range {start}..{commits[red.hi][:10]} ({red.hi - red.lo} commits; bisect stopped: "
            f"{red.note or 'every commit between was untestable'})"
        )


def attribute(reds: list[Red], args, out: Path) -> list[str]:
    """Fill each red's verdict; returns log lines for the report."""
    log: list[str] = []
    clock = Clock(args.budget_min)
    head = args.head_sha
    ranges = {r.base_sha: first_parent_range(r.base_sha, head) for r in reds}
    # The probes index one commit list; a test whose baseline is older sees a
    # longer prefix of it, so every range is a suffix of the longest.
    commits = max(ranges.values(), key=len)
    for r in reds:
        n = len(ranges[r.base_sha])
        if n == 0:
            r.verdict = f"attribution failed: no commit between {r.base_sha[:10]} and {head[:10]}"
            continue
        r.lo, r.hi = len(commits) - n - 1, len(commits) - 1
        if n > args.max_range:
            r.note = f"{n} commits, over the {args.max_range}-commit cap"
        elif r.time > args.slow_s:
            r.note = f"the test took {r.time:.0f} s at the bad sha, over the {args.slow_s:.0f} s cap"
    for i, r in enumerate(r for r in reds if not r.verdict and not r.note):
        if i >= args.max_tests:
            r.note = f"over the {args.max_tests}-test cap"
    log.append(f"{len(commits)} first-parent commits in the widest range ..{head[:10]}")

    if args.dry_run:
        for r in reds:
            n = r.hi - r.lo
            plan = (
                "one commit: no probe"
                if n == 1
                else (
                    f"skipped: {r.note}" if r.note else f"bisect, about {max(1, (n - 1).bit_length())} probes"
                )
            )
            log.append(
                f"- {r.test} @ {r.row}: {n} commit(s) from {r.base_sha[:10]} (run {r.base_run}); {plan}"
            )
        first = choose_probe([r for r in reds if not r.note and r.hi - r.lo > 1])
        if first is not None:
            log.append(f"first probe would be {first[1]} at {label(commits[first[0]])}")
        for r in reds:
            if r.hi - r.lo == 1 or r.note:
                settle(r, commits)
        return log

    probe = out / "probe"
    if probe.exists():
        git("worktree", "remove", "--force", str(probe))
    git("worktree", "add", "--detach", str(probe), head)
    try:
        # 1. the retry at the bad sha
        retry_from = "the nightly's archive" if args.archive else "a per-package build"
        if args.archive:
            extra = ["--archive-file", str(args.archive), "--workspace-remap", str(probe)]
            built = {r.binary: "" for r in reds}
        else:
            extra = []
            built = build({r.binary for r in reds if not r.note}, probe, clock)
        for r in reds:
            if r.note:
                continue
            if built[r.binary]:
                r.note = r.note or f"the retry at the bad sha could not run ({built[r.binary]})"
                continue
            got = run_one(r, probe, extra if args.archive else target_args(r.binary), clock, test_timeout(r))
            log.append(f"retry {r.test} @ {r.row} at {head[:10]} from {retry_from}: {got}")
            if got == "pass":
                r.verdict = "flaky (passed on retry at the bad sha)"
            elif got != "fail":
                r.note = r.note or f"the retry at the bad sha gave {got}"
        # 2. one commit is its own answer; 3. bisect the rest
        probes = 0
        last = 0.0
        while True:
            live = [r for r in reds if not r.note and is_open(r)]
            if not live:
                break
            if probes >= args.max_probes:
                for r in live:
                    r.note = f"the {args.max_probes}-probe cap was reached"
                break
            if clock.left() < max(last * 1.2, 60):
                for r in live:
                    r.note = f"the {args.budget_min:.0f}-minute budget was spent"
                break
            pick = choose_probe(live)
            if pick is None:
                break
            at, binary = pick
            sha = commits[at]
            these = [r for r in live if r.binary == binary and r.lo < at < r.hi and at not in r.untestable]
            t0 = time.monotonic()
            print(f"probe {probes + 1} at {label(sha)}: {len(these)} test(s)", flush=True)
            git("checkout", "--quiet", "--detach", sha, cwd=probe)
            built = build({r.binary for r in these}, probe, clock)
            for r in these:
                if built[r.binary]:
                    got = built[r.binary]
                else:
                    got = run_one(r, probe, target_args(r.binary), clock, test_timeout(r))
                if got == "pass":
                    r.lo = at
                elif got == "fail":
                    r.hi = at
                else:
                    r.untestable.add(at)
                log.append(f"probe {probes + 1} {sha[:10]} {r.test} @ {r.row}: {got}")
            probes += 1
            last = time.monotonic() - t0
        log.append(
            f"{probes} probe(s); {(args.budget_min * 60 - clock.left()) / 60:.1f} of {args.budget_min:.0f} minutes used"
        )
    finally:
        git("worktree", "remove", "--force", str(probe))
    for r in reds:
        settle(r, commits)
    return log


# ---- report ---------------------------------------------------------------


def report(run: int, head_sha: str, reds: list[Red], unreported: int, log: list[str], failed: str) -> str:
    lines = [f"## Newly red tests in nightly run {run} ({head_sha[:10]})", ""]
    if failed:
        lines += [f"**attribution failed: {failed}**", ""]
    if reds:
        lines += ["| test | eps | verdict |", "|---|---|---|"]
        for r in sorted(reds, key=lambda r: (r.verdict, r.test, r.row)):
            lines.append(f"| `{r.test}` | {r.row} | {r.verdict or 'not attributed'} |")
    elif not failed:
        lines.append("No test is red here that passed in its baseline.")
    if unreported:
        lines += [
            "",
            f"{unreported} red test(s) were reported by no earlier scheduled main nightly at their row, so are not compared.",
        ]
    if log:
        lines += [
            "",
            "<details><summary>log</summary>",
            "",
            *[f"    {line}" for line in log],
            "",
            "</details>",
        ]
    return "\n".join(lines) + "\n"


def main(args) -> int:
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    reds: list[Red] = []
    unreported = 0
    log: list[str] = []
    failed = ""
    head_sha = ""
    try:
        info = run_info(args.run)
        head_sha = args.head_sha = info["head_sha"]
        log.append(f"run {args.run}: {info['event']} on {info['head_branch']} at {head_sha[:10]}")
        dl = Path(tempfile.mkdtemp(prefix="attrib-junit-"))
        head_dir = Path(args.junit) if args.junit else dl / str(args.run)
        if not args.junit and not download_junit(args.run, head_dir):
            raise RuntimeError(f"run {args.run} uploaded no junit report")
        head = read_junit_dir(head_dir)
        if not head:
            raise RuntimeError(f"no junit report under {head_dir}: this run's full-suite rows uploaded none")
        bases: list[tuple[int, str, dict]] = []
        for prev in earlier_scheduled_main(info["created_at"], args.lookback):
            d = dl / str(prev["id"])
            if download_junit(prev["id"], d):
                bases.append((prev["id"], prev["head_sha"], read_junit_dir(d)))
                log.append(
                    f"baseline candidate: run {prev['id']} at {prev['head_sha'][:10]} ({prev['created_at']})"
                )
            else:
                log.append(f"run {prev['id']} ({prev['created_at']}) has no junit report: skipped")
        if not bases:
            raise RuntimeError(
                f"no earlier scheduled main nightly in the last {args.lookback} has a junit report"
            )
        reds, unreported = newly_red(head, bases)
        if args.only:
            kept = [r for r in reds if re.search(args.only, r.test)]
            log.append(f"--only {args.only!r}: {len(kept)} of {len(reds)} newly red test(s) kept")
            reds = kept
        # Shallow or stale checkouts lack the endpoints; a full one has them.
        subprocess.run(
            ["git", "fetch", "--quiet", "origin", head_sha, *{r.base_sha for r in reds}], check=False
        )
        if reds:
            log += attribute(reds, args, out)
    except Exception as e:  # noqa: BLE001 — reported, never raised
        traceback.print_exc()
        failed = f"{type(e).__name__}: {e}"
    md = report(args.run, head_sha or "?", reds, unreported, log, failed)
    print(md)
    (out / "attribution.md").write_text(md)
    (out / "attribution.json").write_text(
        json.dumps(
            {
                "run": args.run,
                "head_sha": head_sha,
                "failed": failed,
                "newly_red": [
                    {
                        "test": r.test,
                        "eps": r.row,
                        "baseline_run": r.base_run,
                        "baseline_sha": r.base_sha,
                        "verdict": r.verdict,
                    }
                    for r in reds
                ],
            },
            indent=2,
        )
    )
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a") as f:
            f.write(md)
    return 0


# ---- selftest -------------------------------------------------------------


def _junit(d: Path, leg: str, cases: dict[str, bool]) -> None:
    (d / f"{JUNIT_PREFIX}{leg}").mkdir(parents=True)
    body = "".join(
        f'<testcase classname="{t.split()[0]}" name="{t.split()[1]}" time="1.0">'
        + ("<failure message='x'/>" if red else "")
        + "</testcase>"
        for t, red in cases.items()
    )
    (d / f"{JUNIT_PREFIX}{leg}" / "junit.xml").write_text(
        f"<testsuites><testsuite>{body}</testsuite></testsuites>"
    )


def selftest() -> int:
    with tempfile.TemporaryDirectory() as t:
        t = Path(t)
        _junit(t / "head", "default", {"a::all x": True, "a::all y": True, "b z": False, "c new": True})
        _junit(t / "head", "1E-12-1of2", {"a::all x": True})
        _junit(t / "head", "1E-12-2of2", {"b z": True})
        _junit(t / "n1", "default", {"a::all x": False, "a::all y": True})
        _junit(t / "n2", "default", {"b z": False})
        _junit(t / "n2", "1E-12-1of2", {"a::all x": True})
        _junit(t / "n2", "1E-12-2of2", {"b z": False})
        head = read_junit_dir(t / "head")
        assert set(head) == {"default", "1E-12"}, f"shards fold into one row: {set(head)}"
        bases = [(1, "s1", read_junit_dir(t / "n1")), (2, "s2", read_junit_dir(t / "n2"))]
        reds, unreported = newly_red(head, bases)
        got = {(r.test, r.row, r.base_run) for r in reds}
        # x: green at n1 (default), red at n2 (1e-12) -> newly red at default only;
        # y: red before; z: green at its 1e-12 baseline n2 (n1 lacks that row);
        # c new: in no baseline.
        assert got == {("a::all x", "default", 1), ("b z", "1E-12", 2)}, f"newly red: {got}"
        assert unreported == 1, f"unreported: {unreported}"
    assert target_args("topo") == ["-p", "topo", "--lib"]
    assert target_args("sweep::all") == ["-p", "sweep", "--test", "all"]
    assert target_args("pncad::bin/pncad") == ["-p", "pncad", "--bin", "pncad"]
    assert eps_env("default") is None and eps_env("1E-06") == "1E-06"
    # choose_probe: the widest range's middle, around untestable commits
    a = Red("a::all x", "default", 1, lo=-1, hi=8)
    b = Red("b y", "default", 1, lo=5, hi=8)
    assert choose_probe([a, b]) == (6, "b"), choose_probe([a, b])
    assert choose_probe([a]) == (3, "a::all"), choose_probe([a])
    a.untestable = {3}
    assert choose_probe([a]) == (2, "a::all"), choose_probe([a])
    a.untestable = set(range(0, 8))
    assert not is_open(a) and choose_probe([a]) is None
    print(
        "nightly-attribution selftest OK: shards fold into a row; the baseline is the newest earlier report of the test at its row; a test red before, or in no baseline, is not newly red; target args per binary-id kind; a probe takes the narrowest open range's binary and steps around untestable commits"
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
    p.add_argument(
        "--lookback", type=int, default=7, help="earlier scheduled main nightlies searched for a baseline"
    )
    p.add_argument("--max-probes", type=int, default=12)
    p.add_argument("--budget-min", type=float, default=70)
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
