#!/usr/bin/env python3
"""Check that every eps row of the nightly's full suite reported every test.

    scripts/full-suite-coverage.py <dir> <row>...
    scripts/full-suite-coverage.py --selftest

<dir> holds one directory per `full-suite-junit-<row>[-<k>of<n>]` artifact,
each with the junit.xml its leg wrote; <row> names each expected leg the same
way (`default`, `1E-12-1of3`, ...). Every row runs the same archive under the
same filter, so each must report the same set of tests. The reference set is
the union over every leg; a row's shards must be disjoint and together report
all of it.

A leg that hit its time budget, was cancelled or crashed reports fewer tests
or none, and nothing in its own log counts what it never started. This names,
per row, how many tests are missing and the first few, in the step summary
and as `::error` annotations, and exits 1 when any row is short, any expected
leg left no report, a leg it was not told to expect reported, or two shards
reported the same test.
"""

import os
import re
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

SHOWN = 10
LEG = re.compile(r"^(?P<row>.+?)(?:-(?P<k>\d+)of(?P<n>\d+))?$")


def _tests(report: Path) -> set[str]:
    root = ET.parse(report).getroot()  # noqa: S314 — a junit this workflow wrote
    return {f"{c.get('classname')} {c.get('name')}" for c in root.iter("testcase")}


def check(base: Path, legs: list[str]) -> tuple[list[str], list[str]]:
    """(summary lines, errors) for the expected `legs` under `base`."""
    errors: list[str] = []
    found: dict[str, set[str]] = {}
    for leg in legs:
        report = base / f"full-suite-junit-{leg}" / "junit.xml"
        if report.is_file():
            found[leg] = _tests(report)
        else:
            errors.append(f"{leg}: no junit report (the leg was cancelled, timed out or died before writing one)")
    for extra in sorted(d.name for d in base.glob("full-suite-junit-*") if d.name.removeprefix("full-suite-junit-") not in legs):
        errors.append(f"{extra}: a leg this check was not told to expect; add it to the list the workflow passes")
    reference = set().union(*found.values()) if found else set()
    rows: dict[str, list[str]] = {}
    for leg in legs:
        rows.setdefault(LEG.match(leg)["row"], []).append(leg)
    summary = [f"reference: {len(reference)} tests (the union over every leg)", "", "| row | legs reported | tests | missing |", "|---|---|---|---|"]
    for row, members in rows.items():
        seen: set[str] = set()
        for leg in members:
            got = found.get(leg, set())
            twice = seen & got
            if twice:
                errors.append(f"{row}: {len(twice)} tests reported by more than one shard, e.g. {sorted(twice)[0]}")
            seen |= got
        missing = sorted(reference - seen)
        reported = sum(leg in found for leg in members)
        summary.append(f"| {row} | {reported}/{len(members)} | {len(seen)} | {len(missing)} |")
        if missing:
            shown = "; ".join(missing[:SHOWN]) + (f"; and {len(missing) - SHOWN} more" if len(missing) > SHOWN else "")
            errors.append(f"{row}: {len(missing)} of {len(reference)} tests not reported: {shown}")
    return summary, errors


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        return selftest()
    if len(argv) < 2:
        print(__doc__, file=sys.stderr)
        return 2
    summary, errors = check(Path(argv[0]), argv[1:])
    text = ["### full suite: tests reported per eps row", "", *summary, ""]
    text += [f"- **{e}**" for e in errors] or ["every row reported every test"]
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

    def run(layout: dict[str, list[str]], legs: list[str]) -> list[str]:
        with tempfile.TemporaryDirectory() as d:
            for leg, names in layout.items():
                (Path(d) / f"full-suite-junit-{leg}").mkdir()
                (Path(d) / f"full-suite-junit-{leg}" / "junit.xml").write_text(junit(names))
            return check(Path(d), legs)[1]

    legs = ["default", "1E-12-1of2", "1E-12-2of2"]
    cases = {
        "a complete set, one row sharded": ({"default": ["x", "y", "z"], "1E-12-1of2": ["x"], "1E-12-2of2": ["y", "z"]}, 0),
        "a shard that stopped short": ({"default": ["x", "y", "z"], "1E-12-1of2": ["x"], "1E-12-2of2": ["y"]}, 1),
        "a shard with no report": ({"default": ["x", "y", "z"], "1E-12-1of2": ["x"]}, 2),
        "two shards reporting one test": ({"default": ["x", "y"], "1E-12-1of2": ["x", "y"], "1E-12-2of2": ["y"]}, 1),
        "an unsharded row short of a sharded one": ({"default": ["x"], "1E-12-1of2": ["x"], "1E-12-2of2": ["y"]}, 1),
        "a leg nobody expected": ({"default": ["x"], "1E-12-1of2": ["x"], "1E-12-2of2": [], "1E-06": ["x"]}, 1),
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
