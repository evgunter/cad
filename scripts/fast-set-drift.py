#!/usr/bin/env python3
"""The fast-set drift check: name every fast-set test that costs BAR or more
run alone on main, scaled to the median hosted runner.

    fast-set-drift.py scope <shard>
        print the shard's cargo scope on a run that builds everything
        (ci-filter.py's `test_shards`, the gate's own split)
    fast-set-drift.py gather <shard> <dir> [<hours>]
        download the shard's `fast-set-times-<shard>` artifacts of the last
        <hours> (HOURS) of PR runs, at most MAX_REPORTS of them spread evenly
        over the window, and write <dir>/readings.tsv
    fast-set-drift.py judge <shard> <dir> [<min readings>]
        re-time alone, on this checkout's build, every test whose median
        over at least <min readings> (MIN_READINGS) readings suggests it
        costs BAR or more, and fail on each one that does
    fast-set-drift.py --selftest

`.github/workflows/fast-set-drift.yml` runs it every day, one job per shard,
under the test job's build env. The per-PR gate (check-fast-set-times.py)
fails a PR only for the tests it adds or edits at BAR, and for any test at
CEILING; a test that drifts past BAR on main without reaching CEILING is this
check's. Its owner is GREEN's orchestrator, who reads its latest run at the
start of each sitting and moves what it names into the slow set.

WHAT IT READS. Each PR run's default-eps step uploads its parallel junit
report. The readings of a test are its passing times in those reports, and
its median over the window is a screen, not its cost: a test whose median is
at least SCREEN, over at least MIN_READINGS readings, is a candidate. A
contended reading is no faster than the same test alone on the same runner,
but the readings come from runners of every speed class, and no class reads
faster than FASTEST times the median one, so SCREEN is BAR times FASTEST
(the app shard's contention ratio reads as low as 1.12, so a screen scaled
up from U10's typical 1.73 misses tests there). A candidate the `ci`
profile no longer runs (moved to the slow set, renamed, removed since) is
listed and not timed. A reading of a package this shard's build does not
hold (`test-utils`, which a closure run tests in the app shard) is the other
shard's, and is set aside.

WHAT IT CANNOT SEE. PR runs are scoped to what the diff reaches, so a test
is read only by PRs that reach its crate, and MAX_REPORTS (a GITHUB_TOKEN
rate budget) samples a busy window. A test read fewer than MIN_READINGS times
among the reports read is never a candidate, whatever it costs: the window is
72 h so a crate few PRs reach still collects readings, and the nightly's
full-suite times are not read.

WHAT FAILS IT.
- A candidate re-timed alone (`fast_set_alone.retime`: up to RUNS passes, the
  fastest, scaled by the runner's factor) at BAR or more, or one that fails a pass alone. Each is named with
  the slow-set line to add, its readings, and the run at which its readings
  stepped over the screen (`step_up`), or "throughout the window".
- A window with no readings (the upload is broken), a `ci` listing with no
  test in it, or a listing that holds fewer than half the candidates: each
  is a misread, not a quiet fast set.
The log also prints the fast set's total test time: the sum of every test's
median.
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
from collections.abc import Callable
from pathlib import Path

import fast_set_alone as alone

BAR = alone.BAR
SCREEN = BAR * alone.FASTEST
MIN_READINGS = 3
HOURS = 72
MAX_REPORTS = 350
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


def spread(items: list, most: int) -> list:
    """At most `most` of `items`, evenly spaced, the first and last kept."""
    if len(items) <= most:
        return items
    step = (len(items) - 1) / (most - 1)
    return [items[round(i * step)] for i in range(most)]


def gather(
    shard: str,
    out: Path,
    hours: float,
    gh: Callable[..., bytes] = _gh,
    now: dt.datetime | None = None,
    most: int = MAX_REPORTS,
) -> int:
    repo = os.environ.get("GITHUB_REPOSITORY", "evgunter/cad")
    since = ((now or dt.datetime.now(dt.UTC)) - dt.timedelta(hours=hours)).strftime("%Y-%m-%dT%H:%M:%SZ")
    arts, page = [], 1
    while True:
        batch = json.loads(
            gh(f"repos/{repo}/actions/artifacts?name=fast-set-times-{shard}&per_page=100&page={page}")
        )["artifacts"]
        arts += [a for a in batch if a["created_at"] >= since and not a["expired"]]
        if not batch or batch[-1]["created_at"] < since:
            break
        page += 1
    read = spread(sorted(arts, key=lambda a: a["created_at"]), most)
    rows: list[Reading] = []
    for a in read:
        run = a["workflow_run"]["id"]
        with zipfile.ZipFile(io.BytesIO(gh(f"repos/{repo}/actions/artifacts/{a['id']}/zip"))) as z:
            if "junit.xml" in z.namelist():
                rows += junit_readings(z.read("junit.xml"), a["created_at"], run)
    out.mkdir(parents=True, exist_ok=True)
    with open(out / "readings.tsv", "w") as f:
        for r in sorted(rows):
            f.write("\t".join(map(str, r)) + "\n")
    print(
        f"fast-set drift [{shard}]: {len(arts)} PR-run reports since {since}, "
        f"{len(read)} read, {len(rows)} readings"
    )
    if not rows:
        _error(
            f"fast-set drift [{shard}]",
            f"no fast-set-times-{shard} readings since {since}: ci.yml's upload is not running",
        )
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
    if not runs_now:
        _error(f"fast-set drift [{shard}]", "the `ci` profile's listing holds no test in this shard: it is misread")
        return 1
    held = {b.split("::")[0] for b, _ in runs_now}
    other = sorted({b.split("::")[0] for b, _ in by} - held)
    if other:
        print(f"fast-set drift [{shard}]: set aside the readings of packages this build does not hold: {', '.join(other)}")
    kept = {p: rs for p, rs in by.items() if p[0].split("::")[0] in held}
    if len(kept) * 2 < len(by):
        _error(
            f"fast-set drift [{shard}]",
            f"{len(by) - len(kept)} of {len(by)} tests read are of packages the `ci` profile's listing "
            "does not hold: the listing is misread or the scope moved",
        )
        return 1
    by = kept
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
        f"fast set; re-running {len(timed)} alone (up to {alone.RUNS} passes)"
    )
    for b, n in gone:
        print(f"  not timed, the `ci` profile no longer runs it: {b} {n}")
    misread = len(gone) * 2 > len(cands)
    if misread:
        _error(
            f"fast-set drift [{shard}]",
            f"{len(gone)} of {len(cands)} candidates are not in the `ci` profile's listing: "
            "the listing is misread or the scope moved",
        )
    got, factor = alone.retime(timed, run, lambda p: BAR) if timed else ({}, 1.0)
    over = []
    for p in timed:
        cost = alone.scaled(got[p], factor)
        if cost is None or cost >= BAR:
            over.append((p, cost))
        else:
            print(
                f"  under the bar alone: {p[0]} {p[1]} {cost:.3f} s scaled "
                f"({alone.show(got[p])} s alone; median {meds[p]:.3f} s)"
            )
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
        shown = "failed alone" if cost is None else f"{cost:.3f} s alone, scaled"
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
    print(
        f"fast-set drift [{shard}]: {len(over)} fast-set tests cost {BAR} s or more alone, "
        f"scaled from a runner at {factor:.3f}x the median one"
    )
    if over:
        print("The slow-set lines to add:")
        for (b, n), _ in over:
            print(f"  (binary_id({b}) & test(={n})) |")
    if over and (path := os.environ.get("GITHUB_STEP_SUMMARY")):
        with open(path, "a") as f:
            f.write(f"### Fast-set tests costing {BAR} s or more alone: {shard}\n\n")
            f.write(f"Costs are scaled to the median runner; this runner read at {factor:.3f}x it.\n\n")
            f.writelines(f"- {t}\n" for t in lines)
            f.write("\n")
    return 1 if over or misread else 0


# --- selftest --------------------------------------------------------------------


def _quiet(f: Callable[[], int]) -> tuple[int, str]:
    import contextlib

    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        rc = f()
    return rc, out.getvalue()


def gather_selftest(tmp: Path) -> None:
    def zipped(xml: str) -> bytes:
        buf = io.BytesIO()
        with zipfile.ZipFile(buf, "w") as z:
            z.writestr("junit.xml", xml)
            z.writestr("alone.tsv", "")
        return buf.getvalue()

    def art(i: int, created: str, expired: bool = False) -> dict:
        return {"id": i, "created_at": created, "expired": expired, "workflow_run": {"id": 900 + i}}

    xml = '<testsuites><testsuite><testcase classname="geom::all" name="x::y" time="0.4"/>' \
        '<testcase classname="geom::all" name="x::bad" time="9"><failure/></testcase></testsuite></testsuites>'
    pages = {
        1: [art(1, "2026-11-22T10:00:00Z"), art(2, "2026-11-22T09:00:00Z", expired=True)],
        2: [art(3, "2026-11-21T08:00:00Z"), art(4, "2026-11-18T08:00:00Z")],
    }
    asked: list[str] = []

    def gh(path: str) -> bytes:
        asked.append(path)
        if "/zip" in path:
            return zipped(xml)
        page = int(path.rsplit("page=", 1)[1])
        return json.dumps({"artifacts": pages.get(page, [])}).encode()

    now = dt.datetime(2026, 11, 22, 12, tzinfo=dt.UTC)
    rc, log = _quiet(lambda: gather("kernel", tmp / "g", 72, gh, now))
    assert rc == 0, log
    assert "name=fast-set-times-kernel" in asked[0], asked
    zips = sorted(x for x in asked if "/zip" in x)
    assert zips == ["repos/evgunter/cad/actions/artifacts/1/zip", "repos/evgunter/cad/actions/artifacts/3/zip"], (
        f"the window's live artifacts, and not one expired or older: {zips}"
    )
    got = load(tmp / "g" / "readings.tsv")
    assert got == {("geom::all", "x::y"): [("2026-11-21T08:00:00Z", 903, 0.4), ("2026-11-22T10:00:00Z", 901, 0.4)]}, (
        f"passing readings only, each with its run: {got}"
    )
    pages.clear()
    rc, log = _quiet(lambda: gather("kernel", tmp / "e", 72, gh, now))
    assert rc == 1 and "upload is not running" in log, f"an empty window is red: {log}"
    assert spread(list(range(10)), 4) == [0, 3, 6, 9] and spread([1, 2], 5) == [1, 2]


def selftest() -> int:
    import tempfile

    assert SCREEN == BAR * alone.FASTEST == 0.1875, (
        "a contended reading is no faster than the test alone on its runner, and no runner is "
        "faster than FASTEST: the screen is the bar times FASTEST"
    )

    def rs(*ts: float) -> list[tuple[str, int, float]]:
        return [(f"2026-11-22T{i:02}:00:00Z", 100 + i, t) for i, t in enumerate(ts)]

    by = {
        ("sweep::all", "a::stepped"): rs(0.15, 0.15, 0.16, 0.9, 0.95, 0.92, 0.9),
        ("sweep::all", "a::always"): rs(0.8, 0.85, 0.9),
        ("sweep::all", "a::cheap_alone"): rs(0.5, 0.5, 0.5),
        ("sweep::all", "a::between"): rs(0.28, 0.27, 0.29),  # under a 0.324 s screen
        ("sweep::all", "a::fails_alone"): rs(0.6, 0.6, 0.6),
        ("sweep::all", "a::few_readings"): rs(2.0, 2.0),
        ("topo", "b::under_screen"): rs(0.15, 0.24, 0.18),
        ("topo", "b::over_screen_under_bar"): rs(0.2, 0.2, 0.2),  # a fast runner's 0.25 s test
        ("topo", "b::moved"): rs(1.0, 1.0, 1.0),
        ("test-utils", "c::other_shard"): rs(1.0, 1.0, 1.0),
    }
    assert candidates(by) == sorted(
        [
            ("sweep::all", "a::stepped"),
            ("sweep::all", "a::always"),
            ("sweep::all", "a::cheap_alone"),
            ("sweep::all", "a::between"),
            ("sweep::all", "a::fails_alone"),
            ("topo", "b::moved"),
            ("topo", "b::over_screen_under_bar"),
            ("test-utils", "c::other_shard"),
        ]
    ), candidates(by)
    assert step_up(by["sweep::all", "a::stepped"]) == 103, "the first run of the slow readings"
    assert step_up(by["sweep::all", "a::always"]) is None
    # Each test's reading alone on the median runner, every pass.
    table = {
        "a::stepped": 0.31, "a::always": 0.25, "a::cheap_alone": 0.2, "a::between": 0.26,
        "a::fails_alone": None, alone.REFERENCE[1]: alone.REFERENCE_NOMINAL,
    }  # fmt: skip
    speed = [1.0]  # the runner's factor: every reading times it
    with tempfile.TemporaryDirectory() as tmp:
        report = Path(tmp) / "j.xml"
        asked: list[str] = []

        def run(f: str) -> Path:
            asked.append(f)
            cases = "".join(
                f'<testcase classname="{alone.REFERENCE[0] if n == alone.REFERENCE[1] else "sweep::all"}" '
                f'name="{n}" time="{(t or 0) * speed[0]}">'
                + ("<failure/>" if t is None else "")
                + "</testcase>"
                for n, t in table.items()
                if f"test(={n})" in f
            )
            report.write_text(f"<testsuites><testsuite>{cases}</testsuite></testsuites>")
            return report

        runs_now = {p for p in by if p[1] not in ("b::moved", "c::other_shard", "b::over_screen_under_bar")}
        rc, log = _quiet(lambda: judge("kernel", by, runs_now, run))
        assert rc == 1, log
        assert len(asked) == alone.RUNS and all("b::moved" not in f and "other_shard" not in f for f in asked), asked
        errors = [x for x in log.splitlines() if x.startswith("::error")]
        assert len(errors) == ANNOTATED + 1, log
        named = "\n".join(errors) + log
        assert "a::stepped: 0.310 s alone, scaled" in named and "stepped up at run 103" in named, log
        assert "a::always: 0.250 s alone, scaled" in named and "throughout the window" in named, log
        assert "a::between: 0.260 s alone, scaled" in named, "a median of 0.28 s is a candidate, and 0.26 s alone is over"
        assert "a::fails_alone: failed alone" in named, "a test that fails a pass alone is red"
        assert "under the bar alone: sweep::all a::cheap_alone 0.200 s scaled (0.200 s alone" in log, log
        assert "scaled from a runner at 1.000x the median one" in log and "runner factor 1.000 on " in log, log
        # The same window re-timed on a slow runner (1.32x) and a fast one
        # (0.8x): the same tests are named, at the same scaled costs.
        for f, raw in ((1.32, "0.264"), (0.8, "0.160")):
            speed[0] = f
            rc, slog = _quiet(lambda: judge("kernel", by, runs_now, run))
            assert rc == 1, slog
            assert f"under the bar alone: sweep::all a::cheap_alone 0.200 s scaled ({raw} s alone" in slog, slog
            for n in ("a::stepped: 0.310 s", "a::always: 0.250 s", "a::between: 0.260 s", "a::fails_alone: failed"):
                assert n in slog, f"named on a {f}x runner too: {slog}"
            assert f"scaled from a runner at {f:.3f}x the median one" in slog, slog
        speed[0] = 0.7
        try:
            _quiet(lambda: judge("kernel", by, runs_now, run))
        except alone.RetimeError as e:
            assert "under FASTEST" in str(e), e
        else:
            raise AssertionError("a runner faster than FASTEST is a red, never a drift list")
        speed[0] = 1.0
        assert "  (binary_id(sweep::all) & test(=a::stepped)) |" in log, log
        assert "not timed, the `ci` profile no longer runs it: topo b::moved" in log, log
        assert "set aside the readings of packages this build does not hold: test-utils" in log, log
        assert "other_shard" not in log, "another shard's readings are neither timed nor counted gone"
        assert "total test time is" in log, log
        rc, log = _quiet(lambda: judge("kernel", by, {("sweep::all", "a::cheap_alone")}, run))
        assert rc == 1 and "candidates are not in the `ci` profile's listing" in log, (
            f"most candidates gone is a misread: {log}"
        )
        rc, log = _quiet(lambda: judge("kernel", by, {("geom::all", "z::z")}, run))
        assert rc == 1 and "packages the `ci` profile's listing does not hold" in log, (
            f"most readings set aside is a misread: {log}"
        )
        rc, log = _quiet(lambda: judge("kernel", by, set(), run))
        assert rc == 1 and "holds no test" in log, f"an empty listing is a misread: {log}"
        tsv = Path(tmp) / "readings.tsv"
        tsv.write_text("2026-11-22T01:00:00Z\t7\tgeom::all\tx::y\t0.5\n")
        assert load(tsv) == {("geom::all", "x::y"): [("2026-11-22T01:00:00Z", 7, 0.5)]}
        gather_selftest(Path(tmp))
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
