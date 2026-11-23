#!/usr/bin/env python3
"""The fast-set drift check: name every fast-set test that costs BAR or more
run alone on main.

    fast-set-drift.py scope <shard>
        print the shard's cargo scope on a run that builds everything
        (ci-filter.py's `test_shards`, the gate's own split)
    fast-set-drift.py gather <shard> <dir> [<hours>]
        download the shard's `fast-set-times-<shard>` artifacts of the last
        <hours> (HOURS) of PR runs and write <dir>/readings.tsv
    fast-set-drift.py judge <shard> <dir> [<min readings>]
        re-time alone, on this checkout's build, every test whose median
        over at least <min readings> (MIN_READINGS) readings suggests it
        costs BAR or more, and fail on each one that does
    fast-set-drift.py --selftest

`.github/workflows/fast-set-drift.yml` runs it every day, one job per shard,
under the test job's build env. The per-PR gate (check-fast-set-times.py)
fails a PR only for the tests it adds or edits at BAR, and for any test at
CEILING; a test that drifts past BAR on main without reaching CEILING is this
check's. Its owner is GREEN's orchestrator, who moves what it names into the
slow set.

WHAT IT READS. Each PR run's default-eps step uploads its parallel junit
report. The readings of a test are its passing times in those reports, and
its median over the window is a screen, not its cost: a test whose median is
at least SCREEN, over at least MIN_READINGS readings, is a candidate. U10
measured the gate's median at about 1.73 times the time alone, but that is
the kernel shard's typical ratio: the first drift run read the app shard's
at 1.12 to 1.5, so a screen above BAR would miss tests there. A contended
reading is no faster than the same test alone, so SCREEN is BAR itself. A
candidate the `ci` profile no longer runs (moved to the slow set, renamed,
removed since) is listed and not timed.

WHAT FAILS IT. A candidate re-timed alone (fast_set_alone.py: RUNS passes,
the fastest) at BAR or more. Each is named with the slow-set line to add,
its readings, and the run at which its readings stepped up: the split of
its time-ordered readings whose later median most exceeds its earlier one,
with the earlier under the screen. A test slow throughout the window has no
step in it, and says so. The log also prints the fast set's total test time:
the sum of every test's median.
"""

from __future__ import annotations

import datetime as dt
import importlib.util
import io
import json
import os
import shlex
import statistics
import subprocess
import sys
import zipfile
from pathlib import Path

import fast_set_alone as alone

BAR = 0.25
SCREEN = BAR
MIN_READINGS = 3
HOURS = 24
ANNOTATED = 3
CONFIG = ".config/nextest.toml"

# (created_at, run id, binary id, name, seconds)
Reading = tuple[str, int, str, str, float]
Pair = tuple[str, str]


def _escape(s: str, prop: bool = False) -> str:
    s = s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
    return s.replace(":", "%3A").replace(",", "%2C") if prop else s


def _error(title: str, message: str) -> None:
    print(f"::error title={_escape(title, True)}::{_escape(message)}")


def shard_scope(shard: str) -> str:
    here = Path(__file__).resolve().parent
    spec = importlib.util.spec_from_file_location("ci_filter", here / "ci-filter.py")
    assert spec and spec.loader
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    _, shards = mod.test_shards("all", "--workspace", "true")
    scopes = {s["shard"]: s["scope"] for s in json.loads(shards)}
    if shard not in scopes:
        raise SystemExit(f"no shard {shard!r}; the gate's are {sorted(scopes)}")
    return scopes[shard]


# --- gather ----------------------------------------------------------------------


def _gh(*args: str) -> bytes:
    return subprocess.run(["gh", "api", *args], check=True, capture_output=True).stdout


def junit_readings(xml: bytes, created: str, run: int) -> list[Reading]:
    got = alone.readings(io.BytesIO(xml))
    return [(created, run, b, n, t) for (b, n), t in got.items() if t is not None]


def gather(shard: str, out: Path, hours: float) -> int:
    repo = os.environ["GITHUB_REPOSITORY"]
    since = (dt.datetime.now(dt.UTC) - dt.timedelta(hours=hours)).strftime("%Y-%m-%dT%H:%M:%SZ")
    arts, page = [], 1
    while True:
        batch = json.loads(
            _gh(f"repos/{repo}/actions/artifacts?name=fast-set-times-{shard}&per_page=100&page={page}")
        )["artifacts"]
        arts += [a for a in batch if a["created_at"] >= since and not a["expired"]]
        if not batch or batch[-1]["created_at"] < since:
            break
        page += 1
    rows: list[Reading] = []
    for a in arts:
        run = a["workflow_run"]["id"]
        with zipfile.ZipFile(io.BytesIO(_gh(f"repos/{repo}/actions/artifacts/{a['id']}/zip"))) as z:
            if "junit.xml" in z.namelist():
                rows += junit_readings(z.read("junit.xml"), a["created_at"], run)
    out.mkdir(parents=True, exist_ok=True)
    with open(out / "readings.tsv", "w") as f:
        for r in sorted(rows):
            f.write("\t".join(map(str, r)) + "\n")
    print(f"fast-set drift [{shard}]: {len(arts)} PR-run reports since {since}, {len(rows)} readings")
    if not rows:
        # A day of PR runs uploads dozens; none is the upload broken, not a
        # quiet fast set.
        _error(f"fast-set drift [{shard}]", f"no fast-set-times-{shard} readings since {since}: ci.yml's upload is not running")
        return 1
    return 0


def load(path: Path) -> dict[Pair, list[tuple[str, int, float]]]:
    by: dict[Pair, list[tuple[str, int, float]]] = {}
    for line in path.read_text().splitlines():
        created, run, b, n, t = line.split("\t")
        by.setdefault((b, n), []).append((created, int(run), float(t)))
    for v in by.values():
        v.sort()
    return by


# --- judge -----------------------------------------------------------------------


def candidates(by: dict[Pair, list[tuple[str, int, float]]], least: int = MIN_READINGS) -> list[Pair]:
    return sorted(
        p
        for p, rs in by.items()
        if len(rs) >= least and statistics.median(t for _, _, t in rs) >= SCREEN
    )


def step_up(rs: list[tuple[str, int, float]]) -> int | None:
    """The run at which the time-ordered readings stepped over the screen:
    the split that puts the most readings under SCREEN before it and at or
    over it from it on, the earliest such, among the splits whose earlier
    readings' median is under SCREEN. `None` when no split has one."""
    times = [t for _, _, t in rs]
    best, at = -1, None
    for k in range(1, len(rs)):
        if statistics.median(times[:k]) >= SCREEN:
            continue
        fit = sum(t < SCREEN for t in times[:k]) + sum(t >= SCREEN for t in times[k:])
        if fit > best:
            best, at = fit, rs[k][1]
    return at


def fast_set(scope: list[str]) -> set[Pair]:
    """Every test the `ci` profile runs in this scope, from `nextest list`."""
    out = subprocess.run(
        ["cargo", "nextest", "list", "--profile", "ci", *scope, "--message-format", "json"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    suites = json.loads(out)["rust-suites"]
    return {
        (binary, name)
        for binary, suite in suites.items()
        for name, case in suite["testcases"].items()
        if case.get("filter-match", {}).get("status") == "matches"
    }


def judge(
    shard: str,
    by: dict[Pair, list[tuple[str, int, float]]],
    runs_now: set[Pair],
    run: alone.Runner,
    server: str = "",
    least: int = MIN_READINGS,
) -> int:
    meds = {p: statistics.median(t for _, _, t in rs) for p, rs in by.items()}
    nruns = len({r for rs in by.values() for _, r, _ in rs})
    print(
        f"fast-set drift [{shard}]: the fast set's total test time is {sum(meds.values()):.1f} s "
        f"(the sum of {len(meds)} tests' medians over {nruns} PR runs)"
    )
    cands = candidates(by, least)
    gone = [p for p in cands if p not in runs_now]
    timed = [p for p in cands if p in runs_now]
    print(
        f"fast-set drift [{shard}]: {len(cands)} tests with {least}+ readings and a median "
        f"of {SCREEN:.3f} s or more; {len(gone)} no longer in the "
        f"fast set; re-running {len(timed)} alone x{alone.RUNS}"
    )
    for b, n in gone:
        print(f"  not timed, the `ci` profile no longer runs it: {b} {n}")
    got = alone.retime(timed, run) if timed else {}
    over = []
    for p in timed:
        cost = alone.cost(got[p])
        if cost is None or cost >= BAR:
            over.append((p, cost))
        else:
            print(f"  under the bar alone: {p[0]} {p[1]} {cost:.3f} s (median {meds[p]:.3f} s)")
    over.sort(key=lambda v: -(v[1] if v[1] is not None else 99.0))
    lines = []
    for i, ((b, n), cost) in enumerate(over):
        rs = by[b, n]
        at = step_up(rs)
        where = (
            f"stepped up at run {server}{at}"
            if at is not None
            else f"over the screen throughout the window, from run {server}{rs[0][1]}"
        )
        shown = "failed alone" if cost is None else f"{cost:.3f} s alone"
        text = (
            f"{b} {n}: {shown} ({alone.show(got[b, n])}); median {meds[b, n]:.3f} s over "
            f"{len(rs)} PR runs ({min(t for *_, t in rs):.3f}-{max(t for *_, t in rs):.3f} s); "
            f"{where}. Add `(binary_id({b}) & test(={n})) |` to the slow set in {CONFIG}"
        )
        lines.append(text)
        if i < ANNOTATED:
            _error(f"fast-set drift [{shard}]", text)
        else:
            print(f"  {text}")
    if len(over) > ANNOTATED:
        _error(f"fast-set drift [{shard}]", f"{len(over) - ANNOTATED} more: see the step summary")
    print(f"fast-set drift [{shard}]: {len(over)} fast-set tests cost {BAR} s or more alone")
    if over:
        print("The slow-set lines to add:")
        for (b, n), _ in over:
            print(f"  (binary_id({b}) & test(={n})) |")
    if over and (path := os.environ.get("GITHUB_STEP_SUMMARY")):
        with open(path, "a") as f:
            f.write(f"### Fast-set tests costing {BAR} s or more alone: {shard}\n\n")
            f.writelines(f"- {t}\n" for t in lines)
            f.write("\n")
    return 1 if over else 0


# --- selftest --------------------------------------------------------------------


def selftest() -> int:
    import tempfile

    def rs(*ts: float) -> list[tuple[str, int, float]]:
        return [(f"2026-11-22T{i:02}:00:00Z", 100 + i, t) for i, t in enumerate(ts)]

    by = {
        ("sweep::all", "a::stepped"): rs(0.2, 0.2, 0.21, 0.9, 0.95, 0.92, 0.9),
        ("sweep::all", "a::always"): rs(0.8, 0.85, 0.9),
        ("sweep::all", "a::cheap_alone"): rs(0.5, 0.5, 0.5),
        ("sweep::all", "a::few_readings"): rs(2.0, 2.0),
        ("topo", "b::under_screen"): rs(0.2, 0.24, 0.2),
        ("topo", "b::moved"): rs(1.0, 1.0, 1.0),
    }
    assert candidates(by) == sorted(
        [
            ("sweep::all", "a::stepped"),
            ("sweep::all", "a::always"),
            ("sweep::all", "a::cheap_alone"),
            ("topo", "b::moved"),
        ]
    ), candidates(by)
    assert step_up(by["sweep::all", "a::stepped"]) == 103, "the first run of the slow readings"
    assert step_up(by["sweep::all", "a::always"]) is None
    table = {"a::stepped": 0.31, "a::always": 0.25, "a::cheap_alone": 0.2}
    with tempfile.TemporaryDirectory() as tmp:
        report = Path(tmp) / "j.xml"
        asked: list[str] = []

        def run(f: str) -> Path:
            asked.append(f)
            cases = "".join(
                f'<testcase classname="sweep::all" name="{n}" time="{t}"/>'
                for n, t in table.items()
                if f"test(={n})" in f
            )
            report.write_text(f"<testsuites><testsuite>{cases}</testsuite></testsuites>")
            return report

        runs_now = {p for p in by if p[1] != "b::moved"}
        out = io.StringIO()
        old = sys.stdout
        sys.stdout = out
        try:
            rc = judge("kernel", by, runs_now, run)
        finally:
            sys.stdout = old
        log = out.getvalue()
        assert rc == 1, log
        assert len(asked) == alone.RUNS and all("b::moved" not in f for f in asked), asked
        errors = [x for x in log.splitlines() if x.startswith("::error")]
        assert len(errors) == 2, log
        assert any("a::stepped" in e and "stepped up at run 103" in e for e in errors), errors
        assert any("a::always" in e and "throughout the window" in e for e in errors), errors
        assert not any("cheap_alone" in e for e in errors), "0.2 s alone is under the bar"
        assert "  (binary_id(sweep::all) & test(=a::stepped)) |" in log, log
        assert "not timed, the `ci` profile no longer runs it: topo b::moved" in log, log
        assert "total test time is" in log, log
        tsv = Path(tmp) / "readings.tsv"
        tsv.write_text("2026-11-22T01:00:00Z\t7\tgeom::all\tx::y\t0.5\n")
        assert load(tsv) == {("geom::all", "x::y"): [("2026-11-22T01:00:00Z", 7, 0.5)]}
    print("fast-set-drift: selftest ok")
    return 0


def runs_url() -> str:
    repo = os.environ.get("GITHUB_REPOSITORY")
    return f"{os.environ.get('GITHUB_SERVER_URL', 'https://github.com')}/{repo}/actions/runs/" if repo else ""


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        return selftest()
    if len(argv) == 2 and argv[0] == "scope":
        print(shard_scope(argv[1]))
        return 0
    if len(argv) in (3, 4) and argv[0] == "gather":
        return gather(argv[1], Path(argv[2]), float(argv[3]) if len(argv) == 4 else HOURS)
    if len(argv) in (3, 4) and argv[0] == "judge":
        scope = shlex.split(shard_scope(argv[1]))
        by = load(Path(argv[2]) / "readings.tsv")
        least = int(argv[3]) if len(argv) == 4 else MIN_READINGS
        return judge(argv[1], by, fast_set(scope), alone.nextest_runner(scope), runs_url(), least)
    raise SystemExit(
        "usage: fast-set-drift.py scope <shard> | gather <shard> <dir> [<hours>] | judge <shard> <dir> [<min readings>] | --selftest"
    )


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
