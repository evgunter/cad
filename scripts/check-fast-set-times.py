#!/usr/bin/env python3
"""Fail the per-PR gate when a fast-set test is slow.

    check-fast-set-times.py --new-since <base> > new-tests.txt
        print the `pkg:ident` tokens of the Rust lines HEAD adds to <base>,
        and the `pkg:fn:` / `pkg:file:` tokens of the files it edits
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

WHAT COUNTS AS EDITED is slow_set.py's reading, not this bar's: for every
Rust file the diff edits and HEAD holds, `pkg:fn:<name>` for each `fn` it
defines and `pkg:file:<module>` for the module it stands for (`file_module`).
A Rust file whose string literals spell the file name of a non-Rust file the
diff edits counts as edited too. Those tokens select slow-set tests
(`slow_set.is_edited`); they never make a test new here.

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
from collections.abc import Callable
from pathlib import Path, PurePosixPath

from slow_set import is_new, selected

PROFILE = "ci"
BAR = 1.0
NEW_BAR = 0.5
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
