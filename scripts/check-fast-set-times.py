#!/usr/bin/env python3
"""Fail the per-PR gate when a fast-set test costs too much.

    check-fast-set-times.py --new-since <base> > new-tests.txt
        print the `pkg:ident` tokens of the Rust lines HEAD adds to <base>,
        and the `pkg:fn:` / `pkg:file:` tokens of the files it edits
    check-fast-set-times.py <label> <nextest rc> <new-tests.txt> <cargo scope> [--save <dir>]
        read the `ci` profile's junit report, re-time alone the passing
        tests it reads over a bar, and fail on a test whose cost is over it
    check-fast-set-times.py --selftest

The `ci` profile's default filter holds back the slow set, so every test the
default-eps step runs is a fast-set test. A test's cost is its time run alone
on the gate's build (fast_set_alone.py), and two bars apply to it:

- BAR (0.25 s) to a test this diff adds or edits (`is_new` or `is_edited`);
- CEILING (1.0 s) to every fast-set test, so the PR that makes an old test
  dramatically slower is the one that goes red.

The parallel pass's reading is a screen: no test reads faster alone than in
the contended pass, so only a test the pass reads at or over its bar is
re-run alone, in the same step and on the same binaries, under `<cargo
scope>` (the step's own): one pass, then up to RUNS in all for a test still
at or over its bar on every pass so far (`fast_set_alone.retime`). The
fastest re-run is its cost. A test
whose cost is under its bar is printed in the log and fails nothing; a test
that drifts past BAR on main without reaching CEILING is the drift check's
(`fast-set-drift.py`), not this PR's.

A test over its bar gets an `::error` naming the bar, its parallel reading
and its readings alone (the first ANNOTATED, then one counting the
rest: GitHub keeps 10 error annotations per step, and the step also carries
nextest-failures.py's and the runner's own) and a row in the step summary
naming its fix. A test that fails a re-run alone, having passed in parallel,
is over its bar too: its cost cannot be read. A failing test is not timed:
its own report is nextest-failures.py's, which reads the same junit
afterwards.

WHAT COUNTS AS ADDED. `--new-since` takes `git diff --no-renames -U0 <base>
HEAD` (two-dot: the per-PR checkout is the test merge, whose first parent is
the base, and the base is fetched at depth 1 beside it) over the
`*.rs` files and collects every identifier on an added line, keyed by the
package whose Cargo.toml is nearest above the file, plus a `mod` token for
each module an added line declares. A test is added when its leaf name is
one of its package's identifiers, or a module on its path is one of its
package's added `mod`s (`slow_set.is_new`). A renamed or moved test is
added; so is an existing test whose name an added line merely mentions.

WHAT COUNTS AS EDITED: for every Rust file the diff edits and HEAD holds,
`pkg:fn:<name>` for each `fn` it defines and `pkg:file:<module>` for the
module it stands for (`file_module`). A Rust file whose string literals
spell the file name of a non-Rust file the diff edits counts as edited too
(`slow_set.is_edited`). The same tokens select the slow-set tests the PR
runs. What neither reading sees is slow_set.py's to say; such a test meets
CEILING here and BAR in the drift check.

The parallel report is the one nextest-failures.py reads and deletes, so
this runs first; the re-runs overwrite it, so it is put back after them,
and `--save <dir>` copies it to `<dir>/junit.xml` (with the re-timed rows in
`<dir>/alone.tsv`) for the drift check's upload. It exits 0 when nothing is
over its bar, 1 when a test is, and 2 when it cannot read a report nextest's
rc 0 says exists, or a re-run cannot time a test it was asked to.
"""

from __future__ import annotations

import os
import re
import shlex
import subprocess
import sys
import tempfile
import time
import tomllib
from collections.abc import Callable
from pathlib import Path, PurePosixPath

import fast_set_alone as alone
from slow_set import is_edited, is_new, selected

BAR = alone.BAR
CEILING = 1.0
ANNOTATED = 3
CONFIG = ".config/nextest.toml"
IDENT_RE = re.compile(r"\b[a-z_][A-Za-z0-9_]*\b")
FN_RE = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)")
MOD_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)")


def _escape_data(s: str) -> str:
    return s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")


def _escape_property(s: str) -> str:
    return _escape_data(s).replace(":", "%3A").replace(",", "%2C")


def _error(title: str, message: str) -> None:
    print(f"::error title={_escape_property(title)}::{_escape_data(message)}")


# --- what the diff adds -------------------------------------------------------


def _package_of(path: Path, cache: dict[Path, str | None]) -> str | None:
    for d in path.parents:
        if d not in cache:
            manifest = d / "Cargo.toml"
            name = None
            if manifest.is_file():
                with open(manifest, "rb") as f:
                    name = tomllib.load(f).get("package", {}).get("name")
            cache[d] = name
        if cache[d]:
            return cache[d]
    return None


def added_tokens(diff: str, root: Path) -> set[str]:
    """`pkg:ident` for every identifier on an added line, `pkg:mod:name` for
    every module an added line declares."""
    tokens: set[str] = set()
    cache: dict[Path, str | None] = {}
    pkg = None
    for line in diff.splitlines():
        if line.startswith("+++ "):
            target = line[4:].strip()
            pkg = None
            if target.startswith("b/") and target.endswith(".rs"):
                pkg = _package_of(root / target[2:], cache)
        elif line.startswith("+") and pkg:
            body = line[1:]
            tokens.update(f"{pkg}:{w}" for w in IDENT_RE.findall(body))
            if m := MOD_RE.match(body):
                tokens.add(f"{pkg}:mod:{m.group(1)}")
    return tokens


def file_module(path: str) -> str:
    """The module name a Rust file stands for: its stem, its directory's name
    for a `mod.rs` or a `tests.rs` below the crate root, `tests` for the
    crate root's own `tests.rs`, and `lib` for `lib.rs`."""
    p = PurePosixPath(path)
    if p.stem == "mod" or (p.stem == "tests" and p.parent.name != "src"):
        return p.parent.name
    return p.stem


def spells_a_name(names: list[str]) -> re.Pattern[str]:
    """A line whose string literal spells one of `names`: a `"` opens the
    run the name ends."""
    return re.compile("|".join(f'"[^"]*{re.escape(n)}' for n in names))


def edited_tokens(
    paths: list[str],
    root: Path,
    read: Callable[[str], str | None],
    referrers: Callable[[list[str]], list[str]],
) -> set[str]:
    """`pkg:fn:name` for every `fn` defined in a Rust file the diff edits,
    and `pkg:file:module` for the module that file stands for. A Rust file
    whose string literals spell the name of a non-Rust file the diff edits
    (`include_str!("golden.txt")`, a fixture path) counts as edited too.
    `read(path)` is the file at HEAD, `None` when the diff deletes it."""
    rust = {p for p in paths if p.endswith(".rs")}
    data = {PurePosixPath(p).name for p in paths if not p.endswith(".rs")}
    rust.update(referrers(sorted(data)))
    tokens: set[str] = set()
    cache: dict[Path, str | None] = {}
    for path in sorted(rust):
        text = read(path)
        pkg = _package_of(root / path, cache)
        if text is None or not pkg:
            continue
        tokens.add(f"{pkg}:file:{file_module(path)}")
        tokens.update(f"{pkg}:fn:{n}" for n in FN_RE.findall(text))
    return tokens


def _git(root: Path, *args: str, ok: tuple[int, ...] = (0,)) -> str | None:
    """git's stdout in `root`, paths unquoted; `None` on an exit code in
    `ok` other than 0, and a raise on any other."""
    out = subprocess.run(
        ["git", "-c", "core.quotepath=off", *args],
        cwd=root,
        capture_output=True,
        text=True,
        errors="replace",
    )
    if out.returncode not in ok:
        raise RuntimeError(f"git {args[0]} exited {out.returncode}: {out.stderr.strip()}")
    return out.stdout if out.returncode == 0 else None


def _git_referrers(root: Path) -> Callable[[list[str]], list[str]]:
    def referrers(names: list[str]) -> list[str]:
        if not names:
            return []
        pats = (a for n in names for a in ("-e", n))
        out = _git(root, "grep", "-I", "-F", *pats, "HEAD", "--", "*.rs", ok=(0, 1)) or ""
        spelled = spells_a_name(names)
        hits = (line.split(":", 2)[1:] for line in out.splitlines())
        return sorted({path for path, text in hits if spelled.search(text)})

    return referrers


def diff_tokens(base: str, root: Path) -> set[str]:
    """Every token `--new-since` prints, for the diff `base..HEAD` of the
    checkout at `root`. Packages are read from the checkout's own
    `Cargo.toml`s, which is why the far end is HEAD and nothing else."""

    def diff(*args: str) -> str:
        return _git(root, "diff", "--no-renames", "--no-color", *args) or ""

    paths = diff("--name-only", base, "HEAD").splitlines()
    return added_tokens(diff("-U0", base, "HEAD", "--", "*.rs"), root) | edited_tokens(
        paths,
        root,
        lambda path: _git(root, "show", f"HEAD:{path}", ok=(0, 128)),
        _git_referrers(root),
    )


def new_since(base: str) -> int:
    print(" ".join(sorted(diff_tokens(base, Path.cwd()))))
    return 0


# --- the report ----------------------------------------------------------------

# (binary id, name, seconds in the parallel pass, added or edited by the diff)
Row = tuple[str, str, float, bool]
# (its row, its readings alone, its cost: the fastest, `None` if one failed)
Verdict = tuple[Row, list[float | None], float | None]


def bar(touched: bool) -> float:
    return BAR if touched else CEILING


def bar_text(touched: bool) -> str:
    return f"added or edited in this diff: {BAR} s" if touched else f"any fast-set test: {CEILING} s"


def screen(report: Path, tokens: set[str]) -> tuple[int, list[Row]]:
    """(passing tests timed, the ones the parallel pass reads at or over
    their bar)."""
    timed, rows = 0, []
    for (binary, name), secs in alone.readings(report).items():
        if secs is None:
            continue
        timed += 1
        touched = is_new(binary, name, tokens) or is_edited(binary, name, tokens)
        if secs >= bar(touched):
            rows.append((binary, name, secs, touched))
    return timed, rows


def judge(rows: list[Row], run: alone.Runner) -> tuple[list[Verdict], list[Verdict]]:
    """The screened rows re-timed alone: (over their bar, slowest first;
    under it)."""
    if not rows:
        return [], []
    touched = {(b, n): t for b, n, _, t in rows}
    got = alone.retime(list(touched), run, lambda p: bar(touched[p]))
    verdicts = [(r, got[r[0], r[1]], alone.cost(got[r[0], r[1]])) for r in rows]
    over = [v for v in verdicts if v[2] is None or v[2] >= bar(v[0][3])]
    under = [v for v in verdicts if v not in over]
    over.sort(key=lambda v: (v[2] is not None, -(v[2] or 0.0)))
    return over, under


def fix(binary: str, name: str) -> str:
    return (
        f"make it faster, or add `binary_id({binary}) & test(={name})` "
        f"to the slow set in {CONFIG}"
    )


def verdict_text(v: Verdict) -> str:
    (binary, name, secs, touched), readings, cost = v
    if cost is None:
        return (
            f"{binary} {name} failed a re-run alone (readings alone: {alone.show(readings)}; "
            f"{secs:.3f} s in the parallel pass): its cost cannot be read"
        )
    runs = f"{alone.show(readings)} s alone, {secs:.3f} s in the parallel pass"
    return (
        f"{binary} {name} costs {cost:.3f} s ({runs}; bar: {bar_text(touched)}): "
        f"{fix(binary, name)}"
    )


def report_rows(
    label: str,
    timed: int,
    rows: list[Row],
    over: list[Verdict],
    under: list[Verdict],
    spent: float,
) -> None:
    print(
        f"check-fast-set-times [{label}]: {timed} passing tests timed; {len(rows)} read at or "
        f"over their bar in the parallel pass ({BAR} s added or edited, {CEILING} s any) and "
        f"were re-run alone (up to {alone.RUNS} passes) in {spent:.1f} s; {len(over)} cost their bar or more"
    )
    for v in under:
        (binary, name, secs, touched), readings, cost = v
        print(
            f"  under its bar alone: {binary} {name} {cost:.3f} s "
            f"({alone.show(readings)}; {secs:.3f} s in parallel; {bar_text(touched)})"
        )
    for i, v in enumerate(over):
        if i < ANNOTATED:
            _error(f"{label}: slow fast-set test", verdict_text(v))
        else:
            print(f"  {verdict_text(v)}")
    if len(over) > ANNOTATED:
        _error(
            label,
            f"{len(over) - ANNOTATED} more slow fast-set tests: see the step summary",
        )
    if over and (path := os.environ.get("RUNNER_TEMP")):
        # The file the test job's last step prints, so the slow rows close the log.
        with open(Path(path) / "nextest-failures.txt", "a") as f:
            for v in over:
                f.write(f"SLOW [{label}] {verdict_text(v)}\n")
    if over and (path := os.environ.get("GITHUB_STEP_SUMMARY")):
        with open(path, "a") as f:
            f.write(
                f"### Fast-set tests over their bar: {label}\n\n"
                "Make each faster, or add its entry to the slow set "
                f"(`{CONFIG}`, the `ci` profile's `default-filter`).\n\n"
                "| cost alone | alone | parallel | bar | entry |\n|---|---|---|---|---|\n"
            )
            for (binary, name, secs, touched), readings, cost in over:
                shown = "failed" if cost is None else f"{cost:.3f} s"
                f.write(
                    f"| {shown} | {alone.show(readings)} | {secs:.3f} s | {bar_text(touched)} "
                    f"| `binary_id({binary}) & test(={name})` |\n"
                )
            f.write("\n")


def save_rows(save: Path, verdicts: list[Verdict]) -> None:
    with open(save / "alone.tsv", "w") as f:
        f.write("binary\tname\tparallel\talone\tcost\ttouched\n")
        for (binary, name, secs, touched), readings, cost in verdicts:
            shown = "" if cost is None else f"{cost:.3f}"
            f.write(f"{binary}\t{name}\t{secs:.3f}\t{alone.show(readings)}\t{shown}\t{touched}\n")


def main(
    label: str,
    rc: int,
    tokens: set[str],
    run: alone.Runner,
    save: Path | None = None,
) -> int:
    report = alone.report_path()
    if not report.exists():
        if rc == 0:
            _error(
                label,
                f"nextest exited 0 but wrote no junit report at {report}; no test was timed",
            )
            return 2
        return 0
    parallel = report.read_bytes()
    if save:
        save.mkdir(parents=True, exist_ok=True)
        (save / "junit.xml").write_bytes(parallel)
    timed, rows = screen(report, tokens)
    started = time.monotonic()
    try:
        over, under = judge(rows, run)
    finally:
        report.write_bytes(parallel)
    if save:
        save_rows(save, over + under)
    report_rows(label, timed, rows, over, under, time.monotonic() - started)
    return 1 if over else 0


# --- selftest --------------------------------------------------------------------


FIXTURE = """<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="12" failures="1" errors="0">
  <testsuite name="geom::all" tests="7" failures="1">
    <testcase name="fit::under" classname="geom::all" time="0.200"/>
    <testcase name="fit::untouched_mid" classname="geom::all" time="0.600"/>
    <testcase name="fit::added_row" classname="geom::all" time="0.600"/>
    <testcase name="fit::added_cheap" classname="geom::all" time="0.400"/>
    <testcase name="fit::added_tiny" classname="geom::all" time="0.200"/>
    <testcase name="fit::added_noisy" classname="geom::all" time="0.500"/>
    <testcase name="fit::slow_but_failing" classname="geom::all" time="3.000"><failure message="boom"/></testcase>
  </testsuite>
  <testsuite name="topo" tests="5">
    <testcase name="chord::tests::off_path_near_bar" classname="topo" time="1.050"/>
    <testcase name="chord::tests::dramatically_slower" classname="topo" time="1.600"/>
    <testcase name="chord::tests::just_under" classname="topo" time="0.999"/>
    <testcase name="chord::tests::flips_alone" classname="topo" time="0.500"/>
    <testcase name="fresh::tests::inherits_its_module" classname="topo" time="0.250"/>
  </testsuite>
</testsuites>
"""

# Each screened test's three readings alone; `None` is a pass it failed.
ALONE: dict[tuple[str, str], list[float | None]] = {
    ("geom::all", "fit::added_row"): [0.31, 0.30, 0.33],  # added, 0.30 s: over BAR
    ("geom::all", "fit::added_cheap"): [0.20, 0.22, 0.19],  # added, 0.4 s in parallel only
    # Off the diff's path at 1.05 s in parallel: the old rule's 1.0 s bar on
    # the parallel reading redded the PR for it.
    ("topo", "chord::tests::off_path_near_bar"): [0.41, 0.40, 0.44],
    ("topo", "chord::tests::dramatically_slower"): [1.20, 1.00, 1.10],  # at CEILING
    # Edited, and fails its first pass alone: red, however cheap a later
    # pass would have read it.
    ("topo", "chord::tests::flips_alone"): [None, 0.10, 0.10],
    # Added, over BAR on its first pass and under it on its second: the
    # fastest of the passes is its cost, and no third pass is paid for.
    ("geom::all", "fit::added_noisy"): [0.40, 0.22, 0.50],
    ("topo", "fresh::tests::inherits_its_module"): [0.12, 0.11, 0.13],  # added module, at BAR
}
OVER = {"fit::added_row", "chord::tests::dramatically_slower", "chord::tests::flips_alone"}

DIFF = """diff --git a/crates/geom/tests/all/fit.rs b/crates/geom/tests/all/fit.rs
--- a/crates/geom/tests/all/fit.rs
+++ b/crates/geom/tests/all/fit.rs
@@ -10,0 +11,2 @@
+#[test]
+fn added_row() {
+fn added_cheap() {
+fn added_tiny() {
+fn added_noisy() {
diff --git a/crates/topo/src/lib.rs b/crates/topo/src/lib.rs
--- a/crates/topo/src/lib.rs
+++ b/crates/topo/src/lib.rs
@@ -3,0 +4 @@
+pub(crate) mod fresh;
+// mentions at_new_bar_old, but in another package
diff --git a/docs/x.md b/docs/x.md
--- a/docs/x.md
+++ b/docs/x.md
@@ -1 +1 @@
+fit under at_bar
"""


def report_selftest(root: Path, tokens: set[str]) -> None:
    """`main` over FIXTURE with ALONE's re-runs, from a checkout at `root`."""
    import contextlib
    import io

    (root / ".config").mkdir()
    (root / CONFIG).write_text('[profile.ci.junit]\npath = "junit.xml"\n')
    report = root / "target" / "nextest" / "ci" / "junit.xml"
    report.parent.mkdir(parents=True)
    asked: list[str] = []

    def run(filterset: str) -> Path:
        i = len(asked)
        asked.append(filterset)
        rows = "".join(
            f'<testcase classname="{b}" name="{n}" time="{ts[i] or 0}">'
            + ("<failure/>" if ts[i] is None else "")
            + "</testcase>"
            for (b, n), ts in ALONE.items()
            if f"test(={n})" in filterset
        )
        report.write_text(f"<testsuites><testsuite>{rows}</testsuite></testsuites>")
        return report

    def skipped(filterset: str) -> Path:
        report.write_text("<testsuites><testsuite></testsuite></testsuites>")
        return report

    old_cwd, old_target = os.getcwd(), os.environ.pop("CARGO_TARGET_DIR", None)
    old_runner_temp = os.environ.pop("RUNNER_TEMP", None)
    old_summary = os.environ.pop("GITHUB_STEP_SUMMARY", None)
    os.chdir(root)
    try:
        report.write_text(FIXTURE)
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            rc = main("default eps", 0, tokens, run, root / "saved")
        log = out.getvalue()
        assert rc == 1, log
        assert len(asked) == alone.RUNS, f"up to RUNS passes alone: {asked}"
        passes = [{n for _, n in ALONE if f"test(={n})" in f} for f in asked]
        assert passes[0] == {n for _, n in ALONE}, f"pass 1 is every screened test: {asked[0]}"
        assert passes[1] == {"fit::added_row", "chord::tests::dramatically_slower", "fit::added_noisy"}, (
            f"pass 2 is the tests still at or over their bar: {passes[1]}"
        )
        assert passes[2] == {"fit::added_row", "chord::tests::dramatically_slower"}, passes[2]
        for f in asked:
            for n in ("fit::untouched_mid", "chord::tests::just_under", "fit::added_tiny"):
                assert n not in f, f"{n} reads under its bar in parallel and is not re-run"
        errors = [line for line in log.splitlines() if line.startswith("::error")]
        assert len(errors) == len(OVER), log
        red = {n for n in OVER if any(n in e for e in errors)}
        assert red == OVER, f"the tests over their bar alone, and only those: {log}"
        assert not any("off_path_near_bar" in e for e in errors), (
            "an off-path test 1.05 s in parallel and 0.41 s alone is under CEILING"
        )
        added = next(e for e in errors if "fit::added_row" in e)
        for part in (f"added or edited in this diff: {BAR} s", "0.310, 0.300, 0.330 s alone",
                     "costs 0.300 s", "0.600 s in the parallel pass"):  # fmt: skip
            assert part in added, f"the annotation names the bar and every reading: {added}"
        assert f"any fast-set test: {CEILING} s" in next(
            e for e in errors if "dramatically_slower" in e
        ), errors
        assert "under its bar alone: topo chord::tests::off_path_near_bar 0.410 s" in log, log
        assert "under its bar alone: geom::all fit::added_noisy 0.220 s" in log, log
        assert "chord::tests::flips_alone failed a re-run alone (readings alone: failed;" in next(
            e for e in errors if "flips_alone" in e
        ), errors
        assert report.read_text() == FIXTURE, "the parallel report is put back for nextest-failures.py"
        assert (root / "saved" / "junit.xml").read_text() == FIXTURE
        assert (root / "saved" / "alone.tsv").read_text().count("\n") == 1 + len(ALONE)
        try:
            main("default eps", 0, tokens, skipped)
        except alone.RetimeError:
            pass
        else:
            raise AssertionError("a re-run that times no test is a red, never a pass")
        assert report.read_text() == FIXTURE, "put back after a re-run that failed too"
        quiet = FIXTURE.replace('time="1.600"', 'time="0.900"').replace('time="1.050"', 'time="0.950"')
        report.write_text(quiet)
        asked.clear()
        with contextlib.redirect_stdout(io.StringIO()):
            assert main("default eps", 0, set(), run) == 0
        assert asked == [], "nothing at or over its bar in parallel: no re-run is paid for"
    finally:
        os.chdir(old_cwd)
        for k, v in (("CARGO_TARGET_DIR", old_target), ("RUNNER_TEMP", old_runner_temp),
                     ("GITHUB_STEP_SUMMARY", old_summary)):  # fmt: skip
            if v is not None:
                os.environ[k] = v


def selftest() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for crate, name in (("geom", "geom"), ("topo", "topo")):
            (root / "crates" / crate).mkdir(parents=True)
            (root / "crates" / crate / "Cargo.toml").write_text(
                f'[package]\nname = "{name}"\n'
            )
        tokens = added_tokens(DIFF, root)
        assert "geom:added_row" in tokens and "topo:mod:fresh" in tokens, tokens
        assert (
            "topo:at_new_bar_old" in tokens and "geom:at_new_bar_old" not in tokens
        ), "a token is keyed by the package of the file it is added in"
        assert not any(t.endswith(":at_bar") for t in tokens), (
            "a non-Rust file adds nothing"
        )
        tokens.add("topo:fn:flips_alone")  # an edited file defines it
        report = root / "junit.xml"
        report.write_text(FIXTURE)
        timed, rows = screen(report, tokens)
        got = {name: touched for _, name, _, touched in rows}
        assert timed == 11, f"the failing test is not timed ({timed})"
        assert got == {
            "fit::added_row": True,
            "fit::added_cheap": True,
            "fit::added_noisy": True,
            "chord::tests::off_path_near_bar": False,
            "chord::tests::dramatically_slower": False,
            "chord::tests::flips_alone": True,
            "fresh::tests::inherits_its_module": True,
        }, got
        assert "fit::untouched_mid" not in got, "an untouched test at 0.6 s is under CEILING"
        assert "chord::tests::just_under" not in got, "0.999 s is under CEILING"
        assert "fit::added_tiny" not in got, "an added test at 0.2 s is under BAR"
        assert "fit::slow_but_failing" not in got, (
            "a failing test's report is nextest-failures.py's"
        )
        assert "binary_id(geom::all) & test(=fit::added_row)" in fix(
            "geom::all", "fit::added_row"
        )
        report_selftest(root, tokens)
        edited_selftest(root)
    print("check-fast-set-times: selftest ok")
    return 0


# A slow test's pinned count moves on a line that does not spell its name; a
# table it reads moves in the same file; a golden it `include_str!`s moves
# with no Rust line at all; a file the diff deletes holds no test.
EDITED_DIFF = """diff --git a/crates/geom/tests/all/census.rs b/crates/geom/tests/all/census.rs
--- a/crates/geom/tests/all/census.rs
+++ b/crates/geom/tests/all/census.rs
@@ -4 +4 @@
-    assert_eq!(refused, 12);
+    assert_eq!(refused, 10);
"""
EDITED_FILES = {
    "crates/geom/tests/all/census.rs": (
        "const ROSTER: &[(&str, usize)] = &[(\"a\", 1)];\n"
        "#[test]\nfn every_read_is_counted() {\n    let refused = 10;\n"
        "    assert_eq!(refused, 10);\n}\n"
    ),
    "crates/topo/src/chord/tests.rs": "#[test]\nfn reads_its_golden() {\n"
    '    let g = include_str!("golden/chord_rows.txt");\n}\n',
    "crates/topo/src/lib.rs": "#[test]\nfn at_the_root() {}\n",
}


def git_selftest(root: Path) -> None:
    """`diff_tokens` through real git: the reads `--new-since` makes."""

    def git(*args: str) -> None:
        _git(root, "-c", "user.name=t", "-c", "user.email=t@t", *args)

    files = {**EDITED_FILES, "crates/topo/src/chord/golden/chord_rows.txt": "1\n"}
    for rel, text in files.items():
        (root / rel).parent.mkdir(parents=True, exist_ok=True)
        (root / rel).write_text(text.replace("10", "12"))
    git("init", "-q")
    git("add", "-A")
    git("commit", "-qm", "base")
    git("tag", "base")
    (root / "crates/topo/src/chord/golden/chord_rows.txt").write_text("2\n")
    git("commit", "-qam", "re-cut the golden")
    got = diff_tokens("base", root)
    assert got == {"topo:file:chord", "topo:fn:reads_its_golden"}, (
        f"a golden's re-cut reaches the file that reads it: {got}"
    )
    census = "crates/geom/tests/all/census.rs"
    (root / census).write_text(EDITED_FILES[census])
    (root / "crates/geom/tests/all/\u00e9t\u00e9.rs").write_text("fn summer_row() {}\n")
    git("add", "-A")
    git("commit", "-qm", "move the pin")
    got = diff_tokens("HEAD~1", root)
    assert "geom:fn:every_read_is_counted" in got and not any(":file:chord" in t for t in got), got
    assert {"geom:file:\u00e9t\u00e9", "geom:fn:summer_row", "geom:summer_row"} <= got, (
        f"a non-ASCII path is read unquoted, by both readers: {got}"
    )


def edited_selftest(root: Path) -> None:
    paths = [
        "crates/geom/tests/all/census.rs",
        "crates/topo/src/chord/golden/chord_rows.txt",
        "crates/topo/src/gone.rs",
        "docs/x.md",
    ]

    def referrers(names: list[str]) -> list[str]:
        assert names == ["chord_rows.txt", "x.md"], names
        spelled = spells_a_name(names)
        return [p for p, text in EDITED_FILES.items() if spelled.search(text)]

    edited = edited_tokens(paths, root, EDITED_FILES.get, referrers)
    assert edited == {
        "geom:file:census",
        "geom:fn:every_read_is_counted",
        "topo:file:chord",
        "topo:fn:reads_its_golden",
    }, edited
    assert file_module("crates/topo/src/lib.rs") == "lib"
    assert file_module("crates/topo/src/tests.rs") == "tests", "the root's `tests` module"
    assert file_module("crates/topo/src/chord/tests.rs") == "chord"
    assert file_module("crates/topo/src/chord/mod.rs") == "chord"
    root_tests = edited_tokens(["crates/topo/src/tests.rs"], root, lambda _: "", lambda _: [])
    assert selected([("topo", "tests::made_by_macro"), ("topo", "made_by_macro")], root_tests) == [
        ("topo", "tests::made_by_macro")
    ], f"an edit to src/tests.rs selects the tests under `tests::` ({root_tests})"
    slow = [
        ("geom::all", "census::every_read_is_counted"),
        ("geom::all", "other::every_read_is_counted_elsewhere"),
        ("topo", "chord::tests::reads_its_golden"),
        ("topo", "at_the_root"),
    ]
    git_selftest(root)
    added = added_tokens(EDITED_DIFF, root)
    assert selected(slow, added) == [], "the edit spells no slow test's name"
    assert selected(slow, added | edited) == slow[0:1] + slow[2:3], (
        "the edited test and the golden's reader are selected, and nothing else"
    )


if __name__ == "__main__":
    args = sys.argv[1:]
    if args == ["--selftest"]:
        sys.exit(selftest())
    if len(args) == 2 and args[0] == "--new-since":
        sys.exit(new_since(args[1]))
    save = None
    if len(args) == 6 and args[4] == "--save":
        save, args = Path(args[5]), args[:4]
    if len(args) == 4:
        try:
            sys.exit(
                main(
                    args[0],
                    int(args[1]),
                    set(Path(args[2]).read_text().split()),
                    alone.nextest_runner(shlex.split(args[3])),
                    save,
                )
            )
        except Exception as exc:  # noqa: BLE001 — a checker that cannot read is a red, named
            _error("check-fast-set-times.py failed", f"{type(exc).__name__}: {exc}")
            sys.exit(2)
    sys.exit(
        "usage: check-fast-set-times.py --new-since <base> "
        "| <label> <rc> <tokens file> <cargo scope> [--save <dir>] | --selftest"
    )
