#!/usr/bin/env python3
"""Name every failing row of the nextest run that just finished, where a reader
without the full job log can see it.

    cargo nextest run --profile ci ... || rc=$?
    scripts/nextest-failures.py "<label>" "$rc"

A test job prints one PASS line per test, about 14 000 per suite run, so a
FAIL line and its panic sit far above the 5000-line log tail a session can
read. For each failing test in the run's JUnit report this writes the test's
name, its panic message and its first SEED_LINES `seed=` lines to:

- the job's step summary (`$GITHUB_STEP_SUMMARY`);
- `$RUNNER_TEMP/nextest-failures.txt`, which the job's last step prints, so
  the failures close the log whatever ran after them;
- an `::error` annotation, which the check-run annotations API serves, for
  the first ANNOTATED tests, then one annotation counting the rest. GitHub
  keeps 10 error annotations per step and drops the rest silently, and
  nightly.yml's ε loop runs three invocations in one step: three times
  (ANNOTATED + 1), plus the runner's own exit-code annotation, is 10.

The report's path comes from `[profile.<PROFILE>.junit]` in
`.config/nextest.toml`. The report is deleted once read: a later run that dies
before writing its own cannot be reported with this one's rows. This script
exits 0 whatever goes wrong inside it, so a caller under `bash -e` keeps
nextest's exit code as the step's verdict.
"""

import os
import sys
import tomllib
import xml.etree.ElementTree as ET
from pathlib import Path

PROFILE = "ci"
PANIC_LINES = 6
SEED_LINES = 4
ANNOTATED = 2


def _escape_data(s: str) -> str:
    return s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")


def _escape_property(s: str) -> str:
    return _escape_data(s).replace(":", "%3A").replace(",", "%2C")


def _report_path() -> Path:
    with open(".config/nextest.toml", "rb") as f:
        name = tomllib.load(f)["profile"][PROFILE]["junit"]["path"]
    target = Path(os.environ.get("CARGO_TARGET_DIR") or "target")
    return target / "nextest" / PROFILE / name


def _panic(text: str) -> list[str]:
    """The panic's location line and message, without the backtrace."""
    out = []
    for line in text.strip().splitlines():
        if line.startswith("stack backtrace:") or len(out) == PANIC_LINES:
            break
        out.append(line)
    return out


def _seeds(captured: str) -> list[str]:
    seeds = list(dict.fromkeys(ln.strip() for ln in captured.splitlines() if "seed=" in ln))
    if len(seeds) <= SEED_LINES:
        return seeds
    return [*seeds[:SEED_LINES], f"... and {len(seeds) - SEED_LINES} more seed= lines"]


def _failures(report: Path) -> list[tuple[str, list[str]]]:
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
        rows.append((name, panic + _seeds(captured)))
    return rows


def _error(title: str, message: str) -> None:
    print(f"::error title={_escape_property(title)}::{_escape_data(message)}")


def main(label: str, rc: int) -> None:
    report = _report_path()
    had_report = report.exists()
    rows = []
    if had_report:
        rows = _failures(report)
        report.unlink()

    text, summary = [], []
    for i, (name, detail) in enumerate(rows):
        if i < ANNOTATED:
            _error(f"{label}: {name}", "\n".join(detail))
        text.append(f"FAIL [{label}] {name}")
        text.extend(f"    {ln}" for ln in detail)
        summary.append(f"- **{label}** `{name}`\n\n  ```\n" + "\n".join(f"  {ln}" for ln in detail) + "\n  ```")
    if len(rows) > ANNOTATED:
        _error(label, f"{len(rows) - ANNOTATED} more failing tests: see the step summary / the job's last lines")
    if rc != 0 and not rows:
        if had_report:
            note = f"[{label}] nextest exited {rc} with no failing test in its report; read the step's log"
        else:
            note = f"[{label}] nextest exited {rc} with no junit report at {report}; read the step's log"
        _error(label, note)
        text.append(note)
        summary.append(f"- {note}")
    if not text:
        return

    if path := os.environ.get("RUNNER_TEMP"):
        with open(Path(path) / "nextest-failures.txt", "a") as f:
            f.write("\n".join(text) + "\n")
    if path := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(path, "a") as f:
            f.write(f"### Failing tests: {label}\n\n" + "\n".join(summary) + "\n\n")


if __name__ == "__main__":
    try:
        main(sys.argv[1], int(sys.argv[2]))
    except Exception as exc:  # noqa: BLE001 — the step's verdict is nextest's rc, never this script's
        print(f"::warning title=nextest-failures.py failed::{_escape_data(f'{type(exc).__name__}: {exc}')}")
    sys.exit(0)
