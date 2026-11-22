#!/usr/bin/env python3
"""A fast-set test's cost: its time run alone on the gate's build.

The gate's parallel pass and the PR-run medians are contended readings, so
both `check-fast-set-times.py` (the per-PR gate) and `fast-set-drift.py`
(the scheduled drift check) judge a test on `retime`'s reading instead: the
tests named run one at a time (`--test-threads 1`), RUNS times over, under
the `ci` profile and the caller's cargo scope and env, and the fastest of
the RUNS readings is the test's cost. Each pass is one nextest invocation
whose filterset names every test (split where the text would pass
EXPR_LIMIT), so a pass costs the tests' own time plus one cargo freshness
check and one listing of the binaries it names.

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


def nextest_runner(scope: list[str]) -> Runner:
    """One `cargo nextest run --profile ci <scope> -E <f> --test-threads 1`."""

    def run(filterset: str) -> Path:
        report = report_path()
        report.unlink(missing_ok=True)
        cmd = [
            "cargo", "nextest", "run", "--profile", PROFILE, *scope,
            "-E", filterset, "--test-threads", "1", "--no-fail-fast",
        ]  # fmt: skip
        rc = subprocess.run(cmd, stdout=subprocess.DEVNULL).returncode
        if not report.exists():
            raise RetimeError(f"a re-run alone exited {rc} and wrote no junit report at {report}")
        return report

    return run


def retime(pairs: list[Pair], run: Runner, runs: int = RUNS) -> dict[Pair, list[float | None]]:
    """Each pair's `runs` readings alone, in pass order; `None` for a pass
    in which it did not pass. Raises RetimeError when a pass's report lacks
    a pair it was asked for: a test the re-run did not time is never read
    as cheap."""
    out: dict[Pair, list[float | None]] = {p: [] for p in pairs}
    for _ in range(runs):
        for chunk in chunks(pairs):
            got = readings(run(expr(chunk)))
            missing = [p for p in chunk if p not in got]
            if missing:
                raise RetimeError(
                    f"{len(missing)} test(s) the re-run alone was asked for are not in its report, "
                    f"first {missing[0][0]} {missing[0][1]}"
                )
            for p in chunk:
                out[p].append(got[p])
    return out


def cost(alone: list[float | None]) -> float | None:
    """The fastest reading, `None` when any pass failed the test."""
    return None if not alone or any(a is None for a in alone) else min(a for a in alone if a is not None)


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
        table = {("geom::all", "a::x"): [0.30, 0.28, 0.31], ("topo", "b::y"): [0.20, None, 0.21]}
        asked: list[str] = []

        def run(filterset: str) -> Path:
            i = len(asked)
            asked.append(filterset)
            report.write_text(_junit({p: ts[i] for p, ts in table.items() if p[1] in filterset}))
            return report

        got = retime(list(table), run)
        assert len(asked) == RUNS, f"one pass per run over every test: {asked}"
        assert all("test(=a::x)" in f and "test(=b::y)" in f for f in asked), asked
        assert got == {p: ts for p, ts in table.items()}, got
        assert cost(got[("geom::all", "a::x")]) == 0.28, "the fastest of the runs"
        assert cost(got[("topo", "b::y")]) is None, "a pass that failed it gives no cost"

        def skipping(filterset: str) -> Path:
            report.write_text(_junit({("geom::all", "a::x"): 0.1}))
            return report

        try:
            retime(list(table), skipping)
        except RetimeError:
            pass
        else:
            raise AssertionError("a re-run that skips a test it was asked for is an error, not a pass")
        many = [("sweep::all", f"m::t_{i:05}_" + "x" * 60) for i in range(2000)]
        cut = chunks(many)
        assert len(cut) > 1 and all(len(expr(c)) <= EXPR_LIMIT for c in cut), len(cut)
        assert [p for c in cut for p in c] == many, "chunking keeps every test, in order"
    print("fast_set_alone: selftest ok")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        sys.exit(selftest())
    sys.exit("usage: fast_set_alone.py --selftest")
