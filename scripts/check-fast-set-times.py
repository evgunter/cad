#!/usr/bin/env python3
"""Fail the per-PR gate when a fast-set test is slow.

    check-fast-set-times.py --new-since <base> > new-tests.txt
        print the `pkg:ident` tokens of the Rust lines HEAD adds to <base>
    check-fast-set-times.py <label> <nextest rc> [new-tests.txt]
        read the `ci` profile's junit report and fail on a slow passing test
    check-fast-set-times.py --selftest

The `ci` profile's default filter holds back the slow set, so every test the
default-eps step runs is a fast-set test. Two bars apply to a test that
passed:

- NEW_BAR (0.5 s) to a test this diff adds, so a new test has room before
  runner noise carries it over the backstop;
- BAR (1.0 s), the backstop, to every other one.

A test at or over its bar gets an `::error` (the first ANNOTATED, then one
counting the rest: GitHub keeps 10 error annotations per step, and the step
also carries nextest-failures.py's and the runner's own) and a row in the
step summary naming its fix. A failing test is not timed: its own report is
nextest-failures.py's, which reads the same junit afterwards.

WHAT COUNTS AS NEW. `--new-since` takes `git diff --no-renames -U0 <base>
HEAD` (two-dot: the per-PR checkout is the test merge, whose first parent is
the base, and the base is fetched at depth 1 beside it) over the
`*.rs` files and collects every identifier on an added line, keyed by the
package whose Cargo.toml is nearest above the file, plus a `mod` token for
each module an added line declares. A test is new when its leaf name is one
of its package's identifiers, or a module on its path is one of its
package's added `mod`s. A renamed or moved test is new; so is an existing
test whose name an added line merely mentions, which only applies the
stricter bar. What this cannot see: a test whose name no added line spells
— a new invocation of a macro that names the test's module rather than the
test (`rows_at!(at_f32, f32)`), a module renamed through `#[path]`, or a
name a macro assembles (`paste!`). Those meet BAR.

The report is the one nextest-failures.py reads and deletes, so this runs
first. It exits 0 when nothing is slow, 1 when a test is, and 2 when it
cannot read a report nextest's rc 0 says exists.
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
import tomllib
import xml.etree.ElementTree as ET
from pathlib import Path

from slow_set import is_new

PROFILE = "ci"
BAR = 1.0
NEW_BAR = 0.5
ANNOTATED = 3
CONFIG = ".config/nextest.toml"
IDENT_RE = re.compile(r"\b[a-z_][A-Za-z0-9_]*\b")
MOD_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)")


def _escape_data(s: str) -> str:
    return s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")


def _escape_property(s: str) -> str:
    return _escape_data(s).replace(":", "%3A").replace(",", "%2C")


def _error(title: str, message: str) -> None:
    print(f"::error title={_escape_property(title)}::{_escape_data(message)}")


def _report_path() -> Path:
    with open(CONFIG, "rb") as f:
        name = tomllib.load(f)["profile"][PROFILE]["junit"]["path"]
    target = Path(os.environ.get("CARGO_TARGET_DIR") or "target")
    return target / "nextest" / PROFILE / name


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


def new_since(base: str) -> int:
    diff = subprocess.run(
        [
            "git",
            "diff",
            "--no-renames",
            "--no-color",
            "-U0",
            base,
            "HEAD",
            "--",
            "*.rs",
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    print(" ".join(sorted(added_tokens(diff, Path.cwd()))))
    return 0


# --- the report ----------------------------------------------------------------


def slow_rows(
    report: Path, tokens: set[str]
) -> tuple[int, list[tuple[float, str, str, bool]]]:
    """(passing tests timed, [(seconds, binary id, name, new)] over their bar)."""
    timed, rows = 0, []
    for case in ET.parse(report).getroot().iter("testcase"):  # noqa: S314 — nextest wrote it, this job
        if any(case.find(t) is not None for t in ("failure", "error", "skipped")):
            continue
        timed += 1
        secs = float(case.get("time") or 0)
        binary, name = case.get("classname") or "?", case.get("name") or "?"
        new = is_new(binary, name, tokens)
        if secs >= (NEW_BAR if new else BAR):
            rows.append((secs, binary, name, new))
    return timed, sorted(rows, reverse=True)


def fix(binary: str, name: str) -> str:
    return (
        f"make it faster, or add `binary_id({binary}) & test(={name})` "
        f"to the slow set in {CONFIG}"
    )


def bar_text(new: bool) -> str:
    return f"new in this diff: {NEW_BAR} s" if new else f"existing test: {BAR} s"


def report_rows(
    label: str, timed: int, rows: list[tuple[float, str, str, bool]]
) -> None:
    print(
        f"check-fast-set-times [{label}]: {timed} passing tests timed; "
        f"{len(rows)} at or over the bar ({NEW_BAR} s new in this diff, {BAR} s otherwise)"
    )
    for i, (secs, binary, name, new) in enumerate(rows):
        line = (
            f"{binary} {name} took {secs:.3f} s ({bar_text(new)}): {fix(binary, name)}"
        )
        if i < ANNOTATED:
            _error(f"{label}: slow fast-set test", line)
        else:
            print(f"  {line}")
    if len(rows) > ANNOTATED:
        _error(
            label,
            f"{len(rows) - ANNOTATED} more slow fast-set tests: see the step summary",
        )
    if rows and (path := os.environ.get("RUNNER_TEMP")):
        # The file the test job's last step prints, so the slow rows close the log.
        with open(Path(path) / "nextest-failures.txt", "a") as f:
            for secs, binary, name, new in rows:
                f.write(
                    f"SLOW [{label}] {binary} {name} {secs:.3f} s ({bar_text(new)}): {fix(binary, name)}\n"
                )
    if rows and (path := os.environ.get("GITHUB_STEP_SUMMARY")):
        with open(path, "a") as f:
            f.write(
                f"### Fast-set tests at or over the bar: {label}\n\n"
                "Make each faster, or add its entry to the slow set "
                f"(`{CONFIG}`, the `ci` profile's `default-filter`).\n\n"
                "| time | bar | entry |\n|---|---|---|\n"
            )
            for secs, binary, name, new in rows:
                f.write(
                    f"| {secs:.3f} s | {bar_text(new)} | `binary_id({binary}) & test(={name})` |\n"
                )
            f.write("\n")


def main(label: str, rc: int, tokens: set[str]) -> int:
    report = _report_path()
    if not report.exists():
        if rc == 0:
            _error(
                label,
                f"nextest exited 0 but wrote no junit report at {report}; no test was timed",
            )
            return 2
        return 0
    timed, rows = slow_rows(report, tokens)
    report_rows(label, timed, rows)
    return 1 if rows else 0


# --- selftest --------------------------------------------------------------------


FIXTURE = """<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="8" failures="1" errors="0">
  <testsuite name="geom::all" tests="4" failures="1">
    <testcase name="fit::under" classname="geom::all" time="0.400"/>
    <testcase name="fit::at_new_bar_old" classname="geom::all" time="0.600"/>
    <testcase name="fit::added_row" classname="geom::all" time="0.600"/>
    <testcase name="fit::slow_but_failing" classname="geom::all" time="3.000"><failure message="boom"/></testcase>
  </testsuite>
  <testsuite name="topo" tests="4">
    <testcase name="chord::tests::at_bar" classname="topo" time="1.000"/>
    <testcase name="chord::tests::over_bar" classname="topo" time="1.100"/>
    <testcase name="chord::tests::just_under" classname="topo" time="0.999"/>
    <testcase name="fresh::tests::inherits_its_module" classname="topo" time="0.500"/>
  </testsuite>
</testsuites>
"""

DIFF = """diff --git a/crates/geom/tests/all/fit.rs b/crates/geom/tests/all/fit.rs
--- a/crates/geom/tests/all/fit.rs
+++ b/crates/geom/tests/all/fit.rs
@@ -10,0 +11,2 @@
+#[test]
+fn added_row() {
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
        report = root / "junit.xml"
        report.write_text(FIXTURE)
        timed, rows = slow_rows(report, tokens)
        got = {name: new for _, _, name, new in rows}
        assert timed == 7, f"the failing test is not timed ({timed})"
        assert got == {
            "fit::added_row": True,  # new, 0.6 s: over NEW_BAR
            "chord::tests::at_bar": False,  # existing, 1.0 s: at BAR
            "chord::tests::over_bar": False,  # existing, 1.1 s
            "fresh::tests::inherits_its_module": True,  # new module, 0.5 s: at NEW_BAR
        }, got
        assert "fit::at_new_bar_old" not in got, (
            "an existing test at 0.6 s is under BAR"
        )
        assert "chord::tests::just_under" not in got, "0.999 s is under BAR"
        assert "fit::slow_but_failing" not in got, (
            "a failing test's report is nextest-failures.py's"
        )
        assert [r[0] for r in rows] == sorted((r[0] for r in rows), reverse=True), (
            "slowest first"
        )
        assert "binary_id(geom::all) & test(=fit::added_row)" in fix(
            "geom::all", "fit::added_row"
        )
    print("check-fast-set-times: selftest ok")
    return 0


if __name__ == "__main__":
    args = sys.argv[1:]
    if args == ["--selftest"]:
        sys.exit(selftest())
    if len(args) == 2 and args[0] == "--new-since":
        sys.exit(new_since(args[1]))
    if len(args) in (2, 3):
        try:
            sys.exit(
                main(
                    args[0],
                    int(args[1]),
                    set(Path(args[2]).read_text().split() if len(args) == 3 else ()),
                )
            )
        except Exception as exc:  # noqa: BLE001 — a checker that cannot read is a red, named
            _error("check-fast-set-times.py failed", f"{type(exc).__name__}: {exc}")
            sys.exit(2)
    sys.exit(
        "usage: check-fast-set-times.py --new-since <base> | <label> <rc> [<tokens file>] | --selftest"
    )
