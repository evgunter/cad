#!/usr/bin/env python3
"""Check that every eps row of the nightly's full suite reported every test the
build's listing says it runs.

    scripts/full-suite-coverage.py <listing.json> <dir> <leg>...
    scripts/full-suite-coverage.py --selftest

<listing.json> is `cargo nextest list --workspace --ignore-default-filter
--message-format json` over the build the legs ran: the expected set is every
test it marks `filter-match: matches` (every test but the `#[ignore]`d ones),
named `<binary-id> <test>` as junit's `classname` and `name` spell it. <dir>
holds one directory per `full-suite-junit-<leg>` artifact, each with the
junit.xml its leg wrote; <leg> names each expected leg (`default`,
`1E-12-1of3`, ...), and legs sharing a prefix before `-<k>of<n>` are one row.

Exit 1, naming the tests in the step summary and as `::error` annotations,
when a row reports fewer tests than expected (a leg that hit its budget or
died) or a test the listing does not hold, when an expected leg left no
report or an unexpected one did, or when two shards reported the same test.
"""

import json
import os
import re
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

SHOWN = 10
LEG = re.compile(r"^(?P<row>.+?)(?:-(?P<k>\d+)of(?P<n>\d+))?$")


def expected(listing: Path) -> set[str]:
    suites = json.loads(listing.read_text())["rust-suites"]
    return {
        f"{binary} {name}"
        for binary, suite in suites.items()
        for name, case in suite.get("testcases", {}).items()
        if case.get("filter-match", {}).get("status") == "matches"
    }


def _tests(report: Path) -> set[str]:
    root = ET.parse(report).getroot()  # noqa: S314 — a junit this workflow wrote
    return {f"{c.get('classname')} {c.get('name')}" for c in root.iter("testcase")}


def _some(names: set[str]) -> str:
    shown = sorted(names)[:SHOWN]
    return "; ".join(shown) + (f"; and {len(names) - SHOWN} more" if len(names) > SHOWN else "")


def check(want: set[str], base: Path, legs: list[str]) -> tuple[list[str], list[str]]:
    """(summary lines, errors) for the expected `legs` under `base`."""
    errors: list[str] = []
    if not want:
        errors.append("the listing names no test to expect")
    found: dict[str, set[str]] = {}
    for leg in legs:
        report = base / f"full-suite-junit-{leg}" / "junit.xml"
        if report.is_file():
            found[leg] = _tests(report)
        else:
            errors.append(f"{leg}: no junit report (the leg was cancelled, timed out or died before writing one)")
    for extra in sorted(d.name for d in base.glob("full-suite-junit-*") if d.name.removeprefix("full-suite-junit-") not in legs):
        errors.append(f"{extra}: a leg this check was not told to expect; add it to the list the workflow passes")
    rows: dict[str, list[str]] = {}
    for leg in legs:
        rows.setdefault(LEG.match(leg)["row"], []).append(leg)
    summary = [f"expected: {len(want)} tests (the build's listing)", "", "| row | legs reported | tests | missing |", "|---|---|---|---|"]
    for row, members in rows.items():
        seen: set[str] = set()
        for leg in members:
            got = found.get(leg, set())
            if twice := seen & got:
                errors.append(f"{row}: {len(twice)} tests reported by more than one shard: {_some(twice)}")
            seen |= got
        missing, unlisted = want - seen, seen - want
        summary.append(f"| {row} | {sum(leg in found for leg in members)}/{len(members)} | {len(seen)} | {len(missing)} |")
        if missing:
            errors.append(f"{row}: {len(missing)} of {len(want)} tests not reported: {_some(missing)}")
        if unlisted:
            errors.append(f"{row}: {len(unlisted)} reported tests the listing does not hold: {_some(unlisted)}")
    return summary, errors


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        return selftest()
    if len(argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    summary, errors = check(expected(Path(argv[0])), Path(argv[1]), argv[2:])
    text = ["### full suite: tests reported per eps row", "", *summary, ""]
    text += [f"- **{e}**" for e in errors] or ["every row reported every listed test"]
    print("\n".join(text))
    if path := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(path, "a") as f:
            f.write("\n".join(text) + "\n")
    for e in errors:
        print(f"::error title=full suite coverage::{e}")
    return 1 if errors else 0


def selftest() -> int:
    def junit(names: list[str]) -> str:
        cases = "".join(f'<testcase classname="b" name="{n}"/>' for n in names)
        return f'<testsuites><testsuite name="b">{cases}</testsuite></testsuites>'

    def listing(names: list[str], ignored: list[str]) -> str:
        cases = {n: {"filter-match": {"status": "matches"}} for n in names}
        cases |= {n: {"filter-match": {"status": "mismatch", "reason": "ignored"}} for n in ignored}
        return json.dumps({"rust-suites": {"b": {"testcases": cases}}})

    def run(layout: dict[str, list[str]], legs: list[str]) -> list[str]:
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "list.json").write_text(listing(["x", "y", "z"], ["w"]))
            for leg, names in layout.items():
                (Path(d) / f"full-suite-junit-{leg}").mkdir()
                (Path(d) / f"full-suite-junit-{leg}" / "junit.xml").write_text(junit(names))
            return check(expected(Path(d) / "list.json"), Path(d), legs)[1]

    legs = ["default", "1E-12-1of2", "1E-12-2of2"]
    full = {"default": ["x", "y", "z"], "1E-12-1of2": ["x"], "1E-12-2of2": ["y", "z"]}
    cases = {
        "a complete set, one row sharded": (full, 0),
        "a test no leg reports": ({"default": ["x", "y"], "1E-12-1of2": ["x"], "1E-12-2of2": ["y"]}, 2),
        "a shard that stopped short": (full | {"1E-12-2of2": ["y"]}, 1),
        "a shard with no report": ({"default": ["x", "y", "z"], "1E-12-1of2": ["x"]}, 2),
        "two shards reporting one test": (full | {"1E-12-1of2": ["x", "y"]}, 1),
        "an ignored test reported": (full | {"default": ["x", "y", "z", "w"]}, 1),
        "a leg nobody expected": (full | {"1E-06": ["x", "y", "z"]}, 1),
    }
    bad = []
    for what, (layout, want) in cases.items():
        got = run(layout, legs)
        if len(got) != want:
            bad.append(f"{what}: {len(got)} errors, want {want}: {got}")
    if bad:
        print("full-suite-coverage selftest FAILED:\n  " + "\n  ".join(bad), file=sys.stderr)
        return 1
    print(f"full-suite-coverage selftest OK: {len(cases)} cases")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
