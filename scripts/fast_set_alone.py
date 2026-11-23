#!/usr/bin/env python3
"""A fast-set test's cost: its time run alone on the gate's build.

The gate's parallel pass and the PR-run medians are contended readings, so
both `check-fast-set-times.py` (the per-PR gate) and `fast-set-drift.py`
(the scheduled drift check) judge a test on `retime`'s reading instead: the
tests named run one at a time (`--test-threads 1`, `nextest_command`) under
the `ci` profile and the caller's cargo scope and env, up to RUNS passes,
and the fastest reading is the test's cost. A pass runs only the tests
still at or over their bar on every pass so far: a reading under it is
already the test's cost under it, since noise only makes a reading slower,
and a test that failed a pass alone has no cost to read. So a cheap test
costs one pass and a slow one RUNS. Each pass is one nextest invocation per
chunk of names (split where the text would pass EXPR_LIMIT), costing the
tests' own time plus one cargo freshness check and one listing of the
binaries it names.

BAR, the fast set's one line (implementer-discipline.md §2), lives here and
both callers import it.

A pass overwrites the `ci` profile's junit report: a caller that still
needs the report it had keeps a copy (check-fast-set-times.py restores it).

    fast_set_alone.py --selftest
"""

from __future__ import annotations

import os
import subprocess
import sys
import tomllib
import xml.etree.ElementTree as ET
from collections.abc import Callable
from pathlib import Path
from typing import BinaryIO

from slow_set import EXPR_LIMIT, expr

PROFILE = "ci"
CONFIG = ".config/nextest.toml"
RUNS = 3
BAR = 0.25

Pair = tuple[str, str]
# (filterset) -> the junit report of one `--test-threads 1` pass over it.
Runner = Callable[[str], Path]


class RetimeError(RuntimeError):
    """A re-run that could not give every test named its reading."""


def report_path() -> Path:
    with open(CONFIG, "rb") as f:
        name = tomllib.load(f)["profile"][PROFILE]["junit"]["path"]
    target = Path(os.environ.get("CARGO_TARGET_DIR") or "target")
    return target / "nextest" / PROFILE / name


def readings(report: Path | BinaryIO) -> dict[Pair, float | None]:
    """Every testcase of a junit report: its seconds, `None` when it did not
    pass."""
    out: dict[Pair, float | None] = {}
    for case in ET.parse(report).getroot().iter("testcase"):  # noqa: S314 — nextest wrote it, this job
        key = (case.get("classname") or "?", case.get("name") or "?")
        failed = any(case.find(t) is not None for t in ("failure", "error", "skipped"))
        out[key] = None if failed else float(case.get("time") or 0)
    return out


def chunks(pairs: list[Pair]) -> list[list[Pair]]:
    """`pairs` cut so each chunk's `expr` fits EXPR_LIMIT."""
    out: list[list[Pair]] = [[]]
    for p in pairs:
        if out[-1] and len(expr([*out[-1], p])) > EXPR_LIMIT:
            out.append([])
        out[-1].append(p)
    return [c for c in out if c]


def nextest_command(scope: list[str], filterset: str) -> list[str]:
    """One pass alone over `filterset`, in the caller's cargo scope."""
    return [
        "cargo", "nextest", "run", "--profile", PROFILE, *scope,
        "-E", filterset, "--test-threads", "1", "--no-fail-fast",
    ]  # fmt: skip


def nextest_runner(scope: list[str]) -> Runner:
    def run(filterset: str) -> Path:
        report = report_path()
        report.unlink(missing_ok=True)
        cmd = nextest_command(scope, filterset)
        rc = subprocess.run(cmd, stdout=subprocess.DEVNULL).returncode
        if not report.exists():
            raise RetimeError(f"a re-run alone exited {rc} and wrote no junit report at {report}")
        return report

    return run


def cost(alone: list[float | None]) -> float | None:
    """The fastest reading, `None` when any pass failed the test."""
    if not alone or any(a is None for a in alone):
        return None
    return min(a for a in alone if a is not None)


def retime(
    pairs: list[Pair], run: Runner, bar: Callable[[Pair], float], runs: int = RUNS
) -> dict[Pair, list[float | None]]:
    """Each pair's readings alone, in pass order, `None` for a pass that
    failed it: one pass over every pair, then passes 2..`runs` over the
    pairs whose fastest reading so far is still at or over `bar(pair)`.
    Raises RetimeError when a pass's report lacks a pair it was asked for:
    a test the re-run did not time is never read as cheap."""
    out: dict[Pair, list[float | None]] = {p: [] for p in pairs}
    todo = list(pairs)
    for _ in range(runs):
        for chunk in chunks(todo):
            got = readings(run(expr(chunk)))
            missing = [p for p in chunk if p not in got]
            if missing:
                raise RetimeError(
                    f"{len(missing)} test(s) the re-run alone was asked for are not in its report, "
                    f"first {missing[0][0]} {missing[0][1]}"
                )
            for p in chunk:
                out[p].append(got[p])
        todo = [p for p in todo if (c := cost(out[p])) is not None and c >= bar(p)]
        if not todo:
            break
    return out


def show(alone: list[float | None]) -> str:
    return ", ".join("failed" if a is None else f"{a:.3f}" for a in alone)


# --- selftest --------------------------------------------------------------------


def _junit(rows: dict[Pair, float | None]) -> str:
    cases = "".join(
        f'<testcase classname="{b}" name="{n}" time="{0 if t is None else t}">'
        + ("<failure/>" if t is None else "")
        + "</testcase>"
        for (b, n), t in rows.items()
    )
    return f'<testsuites><testsuite name="x">{cases}</testsuite></testsuites>'


def selftest() -> int:
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        report = Path(tmp) / "junit.xml"
        table = {
            ("geom::all", "a::x"): [0.30, 0.28, 0.31],  # over on every pass: three passes
            ("geom::all", "a::dips"): [0.40, 0.20, 0.50],  # under on its second: two
            ("geom::all", "a::cheap"): [0.10, 0.50, 0.50],  # under on its first: one
            ("topo", "b::y"): [None, 0.10, 0.10],  # fails alone: one, and no cost
        }
        asked: list[str] = []

        def run(filterset: str) -> Path:
            i = len(asked)
            asked.append(filterset)
            report.write_text(_junit({p: ts[i] for p, ts in table.items() if f"test(={p[1]})" in filterset}))
            return report

        got = retime(list(table), run, lambda p: BAR)
        assert len(asked) == RUNS, asked
        assert all(f"test(={n})" in asked[0] for _, n in table), "pass 1 is every test"
        assert {n for _, n in table if f"test(={n})" in asked[1]} == {"a::x", "a::dips"}, asked[1]
        assert {n for _, n in table if f"test(={n})" in asked[2]} == {"a::x"}, asked[2]
        assert got == {
            ("geom::all", "a::x"): [0.30, 0.28, 0.31],
            ("geom::all", "a::dips"): [0.40, 0.20],
            ("geom::all", "a::cheap"): [0.10],
            ("topo", "b::y"): [None],
        }, got
        assert cost(got[("geom::all", "a::x")]) == 0.28, "the fastest of the runs"
        assert cost(got[("geom::all", "a::dips")]) == 0.20
        assert cost(got[("topo", "b::y")]) is None, "a pass that failed it gives no cost"
        assert cost([0.30, None]) is None, "a failed pass is never outweighed by a passing one"
        asked.clear()
        retime([("geom::all", "a::cheap")], run, lambda p: BAR)
        assert len(asked) == 1, "a test under its bar on pass 1 costs one pass"

        def skipping(filterset: str) -> Path:
            report.write_text(_junit({("geom::all", "a::x"): 0.1}))
            return report

        try:
            retime(list(table), skipping, lambda p: BAR)
        except RetimeError:
            pass
        else:
            raise AssertionError("a re-run that skips a test it was asked for is an error, not a pass")
        many = [("sweep::all", f"m::t_{i:05}_" + "x" * 60) for i in range(2000)]
        cut = chunks(many)
        assert len(cut) > 1 and all(len(expr(c)) <= EXPR_LIMIT for c in cut), len(cut)
        assert [p for c in cut for p in c] == many, "chunking keeps every test, in order"
    cmd = nextest_command(["-p", "geom", "-p", "topo"], "test(=a)")
    assert cmd[:5] == ["cargo", "nextest", "run", "--profile", PROFILE], cmd
    assert cmd[5:9] == ["-p", "geom", "-p", "topo"], f"the caller's scope, so its binaries: {cmd}"
    assert "--test-threads" in cmd and cmd[cmd.index("--test-threads") + 1] == "1", (
        f"one test at a time is what 'alone' means: {cmd}"
    )
    assert cmd[cmd.index("-E") + 1] == "test(=a)", cmd
    assert BAR == 0.25, "the ratified line (implementer-discipline.md §2)"
    print("fast_set_alone: selftest ok")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        sys.exit(selftest())
    sys.exit("usage: fast_set_alone.py --selftest")
