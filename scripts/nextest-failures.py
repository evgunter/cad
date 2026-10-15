#!/usr/bin/env python3
"""Name every failing row of the nextest run that just finished, where a reader
without the full job log can see it.

    cargo nextest run --profile ci ... || rc=$?
    scripts/nextest-failures.py "<label>" "$rc"

A test job prints one PASS line per test, about 14 000 per suite run, so a
FAIL line and its panic sit far above the 5000-line log tail a session can
read. For each failing test in the run's JUnit report (`[profile.ci.junit]` in
`.config/nextest.toml`) this writes the test's name, its panic message and any
`seed=` line it printed to three places:

- one `::error` annotation, which the check-run annotations API serves;
- the job's step summary (`$GITHUB_STEP_SUMMARY`);
- `$RUNNER_TEMP/nextest-failures.txt`, which the job's last step prints, so
  the failures close the log whatever ran after them.

The report is deleted once read: a later run that dies before writing its own
cannot be reported with this one's rows. A nonzero exit code with no failing
test in the report (a build error, no tests matched) is recorded as such.
This script exits 0; the caller's own exit code carries the verdict.
"""

import os
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

PANIC_LINES = 6


def _escape_data(s: str) -> str:
    return s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")


def _escape_property(s: str) -> str:
    return _escape_data(s).replace(":", "%3A").replace(",", "%2C")


def _panic(text: str) -> list[str]:
    """The panic's location line and message, without the backtrace."""
    out = []
    for line in text.strip().splitlines():
        if line.startswith("stack backtrace:") or len(out) == PANIC_LINES:
            break
        out.append(line)
    return out


def _failures(report: Path) -> list[tuple[str, list[str], list[str]]]:
    rows = []
    for case in ET.parse(report).getroot().iter("testcase"):  # noqa: S314 — nextest wrote it, this job
        bad = case.find("failure")
        if bad is None:
            bad = case.find("error")
        if bad is None:
            continue
        name = f"{case.get('classname')} {case.get('name')}"
        panic = _panic(bad.text or "") or [bad.get("message") or bad.get("type") or "failed"]
        captured = "\n".join((case.findtext(t) or "") for t in ("system-out", "system-err"))
        seeds = list(dict.fromkeys(ln.strip() for ln in captured.splitlines() if "seed=" in ln))
        rows.append((name, panic, seeds))
    return rows


def main() -> int:
    label, rc = sys.argv[1], int(sys.argv[2])
    target = Path(os.environ.get("CARGO_TARGET_DIR") or "target")
    report = target / "nextest" / "ci" / "junit.xml"
    rows = []
    if report.exists():
        rows = _failures(report)
        report.unlink()

    text, summary = [], []
    for name, panic, seeds in rows:
        detail = panic + seeds
        print(f"::error title={_escape_property(f'{label}: {name}')}::{_escape_data(chr(10).join(detail))}")
        text.append(f"FAIL [{label}] {name}")
        text.extend(f"    {ln}" for ln in detail)
        summary.append(f"- **{label}** `{name}`\n\n  ```\n" + "\n".join(f"  {ln}" for ln in detail) + "\n  ```")
    if rc != 0 and not rows:
        note = f"[{label}] nextest exited {rc} with no failing test in its report; read the step's log"
        print(f"::error title={_escape_property(label)}::{_escape_data(note)}")
        text.append(note)
        summary.append(f"- {note}")
    if not text:
        return 0

    if path := os.environ.get("RUNNER_TEMP"):
        with open(Path(path) / "nextest-failures.txt", "a") as f:
            f.write("\n".join(text) + "\n")
    if path := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(path, "a") as f:
            f.write(f"### Failing tests: {label}\n\n" + "\n".join(summary) + "\n\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
