#!/usr/bin/env python3
"""List every test binary a cargo build produced, each under its own bound.

    cargo test --no-run --message-format json-render-diagnostics ... > build.json
    scripts/list-test-binaries.py build.json [--timeout SECONDS]

`cargo nextest run` lists each test binary (`<bin> --list --format terse`,
then again with `--ignored`) before it runs anything, and prints nothing
while it does. A listing that blocks therefore reads as a hung build or a
hung suite. This runs the same two listings binary by binary, each bounded,
and names every binary whose listing did not finish or failed, with the
package and target it was built from. Exit 1 when any did.
"""

import json
import subprocess
import sys
import time
from pathlib import Path

TIMEOUT = 120


def binaries(path: str) -> list[tuple[str, str]]:
    """(executable, `package target`) for every test artifact in cargo's JSON."""
    out = {}
    with open(path) as f:
        for line in f:
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-artifact" or not msg.get("executable"):
                continue
            if not msg.get("profile", {}).get("test"):
                continue
            pkg = Path(msg["manifest_path"]).parent.name
            out[msg["executable"]] = f"{pkg} {msg['target']['kind'][0]} {msg['target']['name']}"
    return sorted(out.items())


def main(argv: list[str]) -> int:
    if not argv:
        print(__doc__, file=sys.stderr)
        return 2
    timeout = int(argv[argv.index("--timeout") + 1]) if "--timeout" in argv else TIMEOUT
    found = binaries(argv[0])
    if not found:
        print(f"list-test-binaries: {argv[0]} names no test binary (did the build run?)", file=sys.stderr)
        return 1
    bad = []
    for exe, what in found:
        for extra in ([], ["--ignored"]):
            start = time.monotonic()
            try:
                done = subprocess.run([exe, "--list", "--format", "terse", *extra],
                                      capture_output=True, timeout=timeout, check=False)
            except subprocess.TimeoutExpired:
                bad.append(f"{what} ({exe}): `--list{' --ignored' if extra else ''}` did not finish in {timeout} s")
                break
            took = time.monotonic() - start
            if done.returncode != 0:
                bad.append(f"{what} ({exe}): `--list{' --ignored' if extra else ''}` exited {done.returncode}: "
                           + done.stderr.decode(errors="replace")[-400:])
                break
            if took > 5:
                print(f"slow listing: {what}: {took:.1f} s")
    for b in bad:
        print(f"::error title=a test binary's listing blocks::{b}")
    print(f"list-test-binaries: {len(found)} binaries listed, {len(bad)} did not list")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
