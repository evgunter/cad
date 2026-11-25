#!/usr/bin/env python3
"""A fast-set test's cost: its time run alone on the gate's build, scaled to
the median hosted runner by a reference workload timed beside it.

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

THE RUNNER'S SPEED CLASS. Hosted runners come in classes, and a test read
alone on one class reads about one factor off its reading on another (0.77x
and 1.32x between two drift runs; work/green/a-test-timed-alone-still-
reads-the-runners-speed-class.md). So every pass also runs REFERENCE, the one
ignored test of `crates/speed-reference` (a fixed workload that calls no
kernel code), and the runner's factor is its fastest reading over
REFERENCE_NOMINAL, its time on the median hosted runner. A test's cost is
its fastest reading divided by that factor (`scaled`): seconds on the
median runner. `retime` prints the factor with the runner's CPU model, and
a factor under FASTEST is a RetimeError: the callers' screens (BAR x
FASTEST over a contended reading) assume no runner is faster than that.
REFERENCE_DIGEST pins the workload's files to the nominal time it was
measured at, so an edit to the crate that does not measure it again is red.

BAR, the fast set's one line (implementer-discipline.md §2), lives here and
both callers import it.

A pass overwrites the `ci` profile's junit report: a caller that still
needs the report it had keeps a copy (check-fast-set-times.py restores it).

    fast_set_alone.py --selftest
"""

from __future__ import annotations

import hashlib
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

# The speed reference, and its fastest reading alone on the median hosted
# runner class, calibrated on PR 7739's drift dispatches
# (work/green/a-test-timed-alone-still-reads-the-runners-speed-class.md).
REFERENCE_PACKAGE = "speed-reference"
REFERENCE: Pair = (f"{REFERENCE_PACKAGE}::all", "the_runners_speed_reference")
REFERENCE_NOMINAL = 0.20
# sha256 over REFERENCE_FILES, as `reference_digest` reads them: the
# workload REFERENCE_NOMINAL was measured on.
REFERENCE_DIGEST = "939cd84edd0671011ae69ea2621372a5c15543c030a559db224f321f482c2929"
REFERENCE_FILES = ("Cargo.toml", "src/lib.rs", "tests/all.rs")
# The fastest runner class's factor, with room: a runner reading faster is
# a RetimeError, since the screens assume none is.
FASTEST = 0.75
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
    """One pass alone over `filterset`, in the caller's cargo scope plus the
    speed reference's package (a `--workspace` scope holds it already), with
    ignored tests run: REFERENCE is one, and a filterset names every test
    a pass runs."""
    extra = [] if "--workspace" in scope else ["-p", REFERENCE_PACKAGE]
    return [
        "cargo", "nextest", "run", "--profile", PROFILE, *scope, *extra,
        "-E", filterset, "--run-ignored", "all", "--test-threads", "1", "--no-fail-fast",
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


def scaled(alone: list[float | None], factor: float) -> float | None:
    """The test's cost: its fastest reading on a runner whose factor is
    `factor`, in the median runner's seconds. `None` when a pass failed it."""
    c = cost(alone)
    return None if c is None else c / factor


def cpu_model(cpuinfo: str = "/proc/cpuinfo") -> str:
    try:
        with open(cpuinfo) as f:
            for line in f:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return "an unknown CPU"


def reference_digest(root: Path) -> str:
    """sha256 over REFERENCE_FILES under `root` (the crate's directory)."""
    h = hashlib.sha256()
    for name in REFERENCE_FILES:
        h.update(name.encode() + b"\0" + (root / name).read_bytes() + b"\0")
    return h.hexdigest()


def factor_of(reference: list[float | None]) -> float:
    """The runner's factor: the reference's fastest reading over its
    nominal. Raises RetimeError when a pass failed the reference, or when
    the runner reads faster than FASTEST."""
    c = cost(reference)
    if c is None:
        raise RetimeError(f"the speed reference {REFERENCE[0]} {REFERENCE[1]} failed a pass alone ({show(reference)})")
    f = c / REFERENCE_NOMINAL
    if f < FASTEST:
        raise RetimeError(
            f"the speed reference read {show(reference)} s alone against its nominal {REFERENCE_NOMINAL} s, "
            f"a runner factor of {f:.3f} on {cpu_model()}: under FASTEST ({FASTEST}), which the screens "
            "assume no runner reads under. Either this runner class is new and FASTEST moves, or "
            "REFERENCE_NOMINAL is stale (a toolchain or profile change) and is measured again"
        )
    return f


def retime(
    pairs: list[Pair], run: Runner, bar: Callable[[Pair], float], runs: int = RUNS
) -> tuple[dict[Pair, list[float | None]], float]:
    """(each pair's readings alone, in pass order, `None` for a pass that
    failed it; the runner's factor). One pass over every pair, then passes
    2..`runs` over the pairs whose cost so far (`scaled` by the factor so
    far) is still at or over `bar(pair)`. Every pass also runs REFERENCE, in
    its first chunk, and the factor is `factor_of` its readings. Raises
    RetimeError when a pass's report lacks a pair it was asked for (a test
    the re-run did not time is never read as cheap), and as `factor_of`
    does."""
    out: dict[Pair, list[float | None]] = {p: [] for p in pairs}
    ref: list[float | None] = []
    todo = list(pairs)
    for _ in range(runs):
        for i, chunk in enumerate(chunks([REFERENCE, *todo])):
            got = readings(run(expr(chunk)))
            missing = [p for p in chunk if p not in got]
            if missing:
                raise RetimeError(
                    f"{len(missing)} test(s) the re-run alone was asked for are not in its report, "
                    f"first {missing[0][0]} {missing[0][1]}"
                )
            for p in chunk:
                (ref if i == 0 and p == REFERENCE else out[p]).append(got[p])
        factor = factor_of(ref)
        todo = [p for p in todo if (c := scaled(out[p], factor)) is not None and c >= bar(p)]
        if not todo:
            break
    print(
        f"fast_set_alone: the speed reference read {show(ref)} s alone (nominal {REFERENCE_NOMINAL} s): "
        f"runner factor {factor:.3f} on {cpu_model()}"
    )
    return out, factor


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
    import contextlib
    import io
    import tempfile

    def quiet(f: Callable[[], object]) -> tuple[object, str]:
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            got = f()
        return got, out.getvalue()

    nominal = REFERENCE_NOMINAL
    with tempfile.TemporaryDirectory() as tmp:
        report = Path(tmp) / "junit.xml"
        # Each test's readings on a runner whose factor is 1 (the reference
        # reads its nominal on its fastest pass), pass by pass.
        table = {
            REFERENCE: [nominal * 1.02, nominal, nominal * 1.01],
            ("geom::all", "a::x"): [0.30, 0.28, 0.31],  # over on every pass: three passes
            ("geom::all", "a::dips"): [0.40, 0.20, 0.50],  # under on its second: two
            ("geom::all", "a::cheap"): [0.10, 0.50, 0.50],  # under on its first: one
            ("topo", "b::y"): [None, 0.10, 0.10],  # fails alone: one, and no cost
        }
        tests = [p for p in table if p != REFERENCE]
        asked: list[str] = []
        speed = [1.0]  # every reading times this: the runner's class

        def run(filterset: str) -> Path:
            i = len(asked)
            asked.append(filterset)
            rows = {
                p: None if ts[i] is None else ts[i] * speed[0]
                for p, ts in table.items()
                if f"test(={p[1]})" in filterset
            }
            report.write_text(_junit(rows))
            return report

        (got, factor), log = quiet(lambda: retime(tests, run, lambda p: BAR))
        assert len(asked) == RUNS, asked
        assert all(f"test(={n})" in asked[0] for _, n in table), "pass 1 is every test, and the reference"
        assert all(f"test(={REFERENCE[1]})" in a for a in asked), f"every pass times the reference: {asked}"
        assert {n for _, n in tests if f"test(={n})" in asked[1]} == {"a::x", "a::dips"}, asked[1]
        assert {n for _, n in tests if f"test(={n})" in asked[2]} == {"a::x"}, asked[2]
        assert got == {
            ("geom::all", "a::x"): [0.30, 0.28, 0.31],
            ("geom::all", "a::dips"): [0.40, 0.20],
            ("geom::all", "a::cheap"): [0.10],
            ("topo", "b::y"): [None],
        }, got
        assert REFERENCE not in got, "the reference is the runner's reading, not a test's"
        assert factor == 1.0, "the reference's fastest reading over its nominal"
        assert "runner factor 1.000 on " in log, f"the factor is logged with the CPU model: {log}"
        assert cost(got[("geom::all", "a::x")]) == 0.28, "the fastest of the runs"
        assert cost(got[("geom::all", "a::dips")]) == 0.20
        assert cost(got[("topo", "b::y")]) is None, "a pass that failed it gives no cost"
        assert cost([0.30, None]) is None, "a failed pass is never outweighed by a passing one"

        # The same tests on a slow runner (every reading 1.32x): the factor
        # carries the class, and the scaled costs and the passes are the
        # median runner's.
        asked.clear()
        speed[0] = 1.32
        (slow, factor), _ = quiet(lambda: retime(tests, run, lambda p: BAR))
        assert abs(factor - 1.32) < 1e-9, factor
        assert len(asked) == RUNS, f"a slow runner pays the passes a median one does: {asked}"
        assert abs(scaled(slow[("geom::all", "a::x")], factor) - 0.28) < 1e-9, "scaled to the median runner"
        assert abs(scaled(slow[("geom::all", "a::dips")], factor) - 0.20) < 1e-9
        assert scaled(slow[("topo", "b::y")], factor) is None
        assert scaled([0.33], 1.32) == 0.25 and scaled([0.25], 1.0) == 0.25
        # A fast runner (0.8x): a test at 0.28 s on the median runner reads
        # 0.224 s alone, under BAR raw, and is still over it scaled.
        asked.clear()
        speed[0] = 0.8
        (fast, factor), _ = quiet(lambda: retime(tests, run, lambda p: BAR))
        assert abs(factor - 0.8) < 1e-9 and cost(fast[("geom::all", "a::x")]) < BAR, (factor, fast)
        assert len(asked) == RUNS, f"the raw reading under BAR is not what drops a test: {asked}"
        assert scaled(fast[("geom::all", "a::x")], factor) >= BAR
        # Faster than FASTEST: the screens' premise is broken, and it says so.
        speed[0] = FASTEST * 0.98
        asked.clear()
        try:
            quiet(lambda: retime(tests, run, lambda p: BAR))
        except RetimeError as e:
            assert "under FASTEST" in str(e) and "a runner factor of 0.7" in str(e), e
        else:
            raise AssertionError("a runner faster than FASTEST is an error, not a reading")
        speed[0] = FASTEST
        asked.clear()
        quiet(lambda: retime(tests, run, lambda p: BAR))  # at FASTEST is a reading
        speed[0] = 1.0
        asked.clear()
        quiet(lambda: retime([("geom::all", "a::cheap")], run, lambda p: BAR))
        assert len(asked) == 1, "a test under its bar on pass 1 costs one pass"

        def skipping(filterset: str) -> Path:
            report.write_text(_junit({REFERENCE: nominal, ("geom::all", "a::x"): 0.1}))
            return report

        def no_reference(filterset: str) -> Path:
            report.write_text(_junit({p: 0.1 for p in tests}))
            return report

        def failed_reference(filterset: str) -> Path:
            report.write_text(_junit({REFERENCE: None, **{p: 0.1 for p in tests}}))
            return report

        for bad, why in (
            (skipping, "a re-run that skips a test it was asked for is an error, not a pass"),
            (no_reference, "a re-run that does not time the reference has no factor"),
            (failed_reference, "a reference that fails a pass gives no factor"),
        ):
            try:
                quiet(lambda bad=bad: retime(tests, bad, lambda p: BAR))
            except RetimeError:
                pass
            else:
                raise AssertionError(why)
        many = [("sweep::all", f"m::t_{i:05}_" + "x" * 60) for i in range(2000)]
        cut = chunks(many)
        assert len(cut) > 1 and all(len(expr(c)) <= EXPR_LIMIT for c in cut), len(cut)
        assert [p for c in cut for p in c] == many, "chunking keeps every test, in order"
        info = Path(tmp) / "cpuinfo"
        info.write_text("processor\t: 0\nmodel name\t: AMD EPYC 9V74 80-Core Processor\n")
        assert cpu_model(str(info)) == "AMD EPYC 9V74 80-Core Processor"
        assert cpu_model(str(Path(tmp) / "none")) == "an unknown CPU"
    cmd = nextest_command(["-p", "geom", "-p", "topo"], "test(=a)")
    assert cmd[:5] == ["cargo", "nextest", "run", "--profile", PROFILE], cmd
    assert cmd[5:11] == ["-p", "geom", "-p", "topo", "-p", REFERENCE_PACKAGE], (
        f"the caller's scope, so its binaries, and the reference's: {cmd}"
    )
    assert "--test-threads" in cmd and cmd[cmd.index("--test-threads") + 1] == "1", (
        f"one test at a time is what 'alone' means: {cmd}"
    )
    assert cmd[cmd.index("--run-ignored") + 1] == "all", f"the reference is an ignored test: {cmd}"
    assert cmd[cmd.index("-E") + 1] == "test(=a)", cmd
    wide = nextest_command(["--workspace", "--exclude", "viewer"], "test(=a)")
    assert REFERENCE_PACKAGE not in wide, f"a workspace scope holds the reference already: {wide}"
    assert BAR == 0.25, "the ratified line (implementer-discipline.md §2)"
    crate = Path(__file__).resolve().parent.parent / "crates" / REFERENCE_PACKAGE
    digest = reference_digest(crate)
    assert digest == REFERENCE_DIGEST, (
        f"crates/{REFERENCE_PACKAGE} is not the workload REFERENCE_NOMINAL was measured on "
        f"(its digest is {digest}): measure the nominal again, then set both"
    )
    print("fast_set_alone: selftest ok")
    return 0


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        sys.exit(selftest())
    sys.exit("usage: fast_set_alone.py --selftest")
