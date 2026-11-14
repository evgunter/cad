#!/usr/bin/env python3
"""Name the test binary whose listing blocks or fails.

    cargo test --no-run --message-format json-render-diagnostics ... > build.json
    scripts/list-test-binaries.py build.json [--timeout SECONDS]

`cargo nextest list` and `cargo nextest run` list every test binary
(`<bin> --list --format terse`, then again with `--ignored`) before running
anything, and print nothing while they do, so a listing that blocks does not
say which binary it is. The nightly runs nextest's own listing under a bound
and calls this only when that listing failed: it runs the same two listings
binary by binary, one at a time, each from its package's directory (nextest's
working directory for a test binary) with `CARGO_MANIFEST_DIR` and
`CARGO_PKG_NAME` set, each bounded, and names the package and target of every
one that did not finish or failed.

What it cannot reproduce: the rest of the environment nextest gives a binary
(its own variables, its dynamic-library path), and nextest listing several
binaries at once. So when nextest's listing blocked and every binary lists
here, this says so, and exits 1 all the same.
"""

import json
import os
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

TIMEOUT = 120


def binaries(path: str) -> list[tuple[str, Path, str, str]]:
    """(executable, package dir, package name, `package kind target`) per test artifact."""
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
            pkg_dir = Path(msg["manifest_path"]).parent
            pkg = msg["package_id"].rsplit("#", 1)[-1].split("@")[0] if "@" in msg["package_id"] else pkg_dir.name
            out[msg["executable"]] = (pkg_dir, pkg, f"{pkg} {msg['target']['kind'][0]} {msg['target']['name']}")
    return [(exe, *rest) for exe, rest in sorted(out.items())]


def list_once(exe: str, cwd: Path, pkg: str, extra: list[str], timeout: int) -> str | None:
    """None when the listing finished cleanly, else what went wrong."""
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(cwd), CARGO_PKG_NAME=pkg)
    with tempfile.TemporaryFile() as err:
        # Its own process group, and no pipe to wait on: a grandchild that
        # keeps a pipe open cannot hold this past the bound.
        proc = subprocess.Popen([exe, "--list", "--format", "terse", *extra], cwd=cwd, env=env,
                                stdout=subprocess.DEVNULL, stderr=err, start_new_session=True)
        try:
            rc = proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
            return f"did not finish in {timeout} s"
        if rc != 0:
            err.seek(0)
            return f"exited {rc}: " + err.read().decode(errors="replace")[-400:]
    return None


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
    for exe, cwd, pkg, what in found:
        for extra in ([], ["--ignored"]):
            start = time.monotonic()
            if problem := list_once(exe, cwd, pkg, extra, timeout):
                bad.append(f"{what} ({exe}): `--list{' --ignored' if extra else ''}` {problem}")
                break
            if (took := time.monotonic() - start) > 5:
                print(f"slow listing: {what}: {took:.1f} s")
    for b in bad:
        print(f"::error title=a test binary's listing blocks or fails::{b}")
    if not bad:
        print(f"::error title=the listing did not reproduce::every one of {len(found)} test binaries "
              "lists here, one at a time from its package directory; what blocked nextest's listing "
              "is outside what this can reproduce (its own environment, or listing binaries in parallel)")
    print(f"list-test-binaries: {len(found)} binaries listed, {len(bad)} did not list")
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
