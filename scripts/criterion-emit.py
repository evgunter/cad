#!/usr/bin/env python3
"""Turn one `cargo bench` run into one entry of the criterion history.

`benches/` is the harness; this is the half that makes it a LANE rather
than a thing someone runs by hand. It reads criterion's own per-benchmark
`estimates.json`, keeps the statistics worth keeping, staples on the
environment block every measurement lane here carries, and writes the
single JSON file that `docs/perf-data/criterion/` accumulates.

WHY AN ENVIRONMENT BLOCK IS NOT OPTIONAL. `memories/perf-measurement-lane.md`
records what this repository learned the expensive way: a committed timing
is worth nothing if you cannot say which box produced it. Three refreshes
of the old rebuild-latency baseline disagreed by more than any real change
could explain, and `docs/PERF-SCAN-2026-08.md` had to label every
absolute-millisecond claim in the tree provisional as a result. Same
shape here, same defence: runner, core count, memory, toolchain,
RUSTFLAGS, every `CARGO_PROFILE_*` and the debug-assertions posture, on
every sample.

THE ROSTER IS PINNED, AND THAT IS THE ONE THING THIS SCRIPT FAILS ON. A
benchmark that is renamed, dropped or added changes the history's shape
silently otherwise — the new name simply starts a column of its own and
the old one stops, which reads in the trend as "that cost went away".
`--expect` names the rows this lane commits to; a mismatch is an error
with both sides printed, and the fix is to edit the roster in the same
diff that edits the benchmark. Nothing else here is fatal: this is a
reporting lane and it never fails on a number.

WHY IT READS `new/` AND NOT `base/`. Criterion keeps the previous run at
`base/` for its own change detection. That comparison is a LOCAL
convenience and is not this lane's mechanism: the trend is the committed
history, compared across entries whose environment blocks a reader can
see. A hosted run starts from an empty `target/criterion` anyway, so
`base/` there is either absent or a copy of `new/`.

Usage:
  criterion-emit.py --criterion-dir DIR --out FILE [--commit SHA]
                    [--runner TEXT] [--expect ROW]... [--method TEXT]
  criterion-emit.py --selftest
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import subprocess
import sys
import tempfile
import time
from pathlib import Path

# The rows this lane commits to, in the order the benchmark file defines
# them. Kept here rather than derived so that a benchmark disappearing is
# an ERROR and not a quietly shorter history — see the header.
DEFAULT_ROSTER = (
    "tessellate/washer/1e-4",
    "tessellate/washer/1e-6",
    "kernel/validate/tier23_washer",
    "kernel/mass_props/washer",
    "kernel/build/extrude",
    "kernel/boolean/two_bricks",
)

DEFAULT_METHOD = (
    "criterion 0.8.2 over benches/benches/kernel.rs; per-row sample counts "
    "set in that file; bench profile (release, debug-assertions OFF — the "
    "manifest says why); times in nanoseconds, `median` is the figure to "
    "read and `median_ci` the interval criterion resolved WITHIN this run. "
    "Cross-run spread is wider than that interval: treat a move under ~10% "
    "as noise unless consecutive entries agree."
)


def die(msg: str) -> None:
    print(f"criterion-emit: {msg}", file=sys.stderr)
    raise SystemExit(1)


def _read_json(path: Path):
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError) as exc:
        die(f"cannot read {path}: {exc}")


def collect(criterion_dir: Path) -> dict[str, dict]:
    """Every benchmark criterion just measured, keyed by its full id.

    A directory counts only when it holds BOTH `new/estimates.json` and
    `new/benchmark.json`: the id lives in the second, and a row whose id
    cannot be read is a row that would land in the history under its
    directory name — which is criterion's sanitised spelling, not the
    benchmark's name, and the two drift the moment a name grows a slash.
    """
    rows: dict[str, dict] = {}
    for estimates_path in sorted(criterion_dir.glob("**/new/estimates.json")):
        meta_path = estimates_path.parent / "benchmark.json"
        if not meta_path.is_file():
            continue
        meta = _read_json(meta_path)
        estimates = _read_json(estimates_path)
        full_id = meta.get("full_id")
        if not isinstance(full_id, str):
            die(f"{meta_path}: no `full_id`")
        sample_path = estimates_path.parent / "sample.json"
        samples = None
        if sample_path.is_file():
            iters = _read_json(sample_path).get("iters")
            if isinstance(iters, list):
                samples = len(iters)
        median = estimates.get("median", {})
        mean = estimates.get("mean", {})
        interval = median.get("confidence_interval", {})
        rows[full_id] = {
            "median_ns": median.get("point_estimate"),
            "median_ci_ns": [interval.get("lower_bound"), interval.get("upper_bound")],
            "mean_ns": mean.get("point_estimate"),
            "median_abs_dev_ns": estimates.get("median_abs_dev", {}).get("point_estimate"),
            "samples": samples,
        }
    return rows


def environment() -> dict:
    """The block without which none of the numbers above mean anything."""
    overrides = sorted(
        f"{k}={v}" for k, v in os.environ.items() if k.startswith("CARGO_PROFILE_")
    )
    mem_total_kb = None
    try:
        for line in Path("/proc/meminfo").read_text().splitlines():
            if line.startswith("MemTotal:"):
                mem_total_kb = int(line.split()[1])
                break
    except (OSError, ValueError, IndexError):
        pass
    # `<version>-<host>`, the spelling docs/perf-data/rebuild-latency/ already
    # uses for this field, so the two lanes' environment blocks compare.
    toolchain = ""
    try:
        out = subprocess.run(
            ["rustc", "-vV"], capture_output=True, text=True, timeout=60, check=False
        ).stdout
        host = ""
        version = ""
        for line in out.splitlines():
            if line.startswith("host:"):
                host = line.split(":", 1)[1].strip()
            elif line.startswith("rustc "):
                version = line.split(" ", 1)[1].split()[0]
        if version and host:
            toolchain = f"{version}-{host}"
    except (OSError, subprocess.SubprocessError, IndexError):
        pass
    return {
        "arch": platform.machine(),
        "os": platform.system().lower(),
        "nproc": os.cpu_count(),
        "mem_total_kb": mem_total_kb,
        "runner": os.environ.get("CRITERION_RUNNER", ""),
        "rustup_toolchain": toolchain,
        "rustflags": os.environ.get("RUSTFLAGS", ""),
        "cargo_profile_overrides": overrides,
        # Stated, not detected: `benches/Cargo.toml`'s `[profile.bench]`
        # sets it, and the manifest carries the measured 5-6.5x this
        # posture is avoiding. A detector here would be a second copy of
        # that decision, free to disagree with the one that holds.
        "debug_assertions": False,
    }


def build_entry(criterion_dir: Path, commit: str, roster: tuple[str, ...], method: str) -> dict:
    rows = collect(criterion_dir)
    missing = [r for r in roster if r not in rows]
    extra = [r for r in sorted(rows) if r not in roster]
    if missing or extra:
        die(
            "the benchmark roster moved.\n"
            f"  expected : {list(roster)}\n"
            f"  measured : {sorted(rows)}\n"
            f"  missing  : {missing}\n"
            f"  unexpected: {extra}\n"
            "Edit DEFAULT_ROSTER (or --expect) in the same diff that renamed "
            "the benchmark, so the history's columns stay readable."
        )
    return {
        "commit": commit,
        "measured_at_epoch_s": int(time.time()),
        "method": method,
        "environment": environment(),
        "benchmarks": {row: rows[row] for row in roster},
    }


def selftest() -> int:
    """A fixture criterion tree through `collect` and the roster check.

    Two cases, and the second is the one that matters: a renamed row must
    be an ERROR rather than a shorter history.
    """
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        def plant(full_id: str, directory: str, median: float) -> None:
            d = root / directory / "new"
            d.mkdir(parents=True)
            (d / "benchmark.json").write_text(json.dumps({"full_id": full_id}))
            (d / "estimates.json").write_text(
                json.dumps(
                    {
                        "median": {
                            "point_estimate": median,
                            "confidence_interval": {"lower_bound": median * 0.9,
                                                    "upper_bound": median * 1.1},
                        },
                        "mean": {"point_estimate": median},
                        "median_abs_dev": {"point_estimate": 1.0},
                    }
                )
            )
            (d / "sample.json").write_text(json.dumps({"iters": [1.0, 2.0, 3.0]}))

        plant("a/one", "a_one", 100.0)
        plant("a/two", "a_two", 200.0)

        rows = collect(root)
        if sorted(rows) != ["a/one", "a/two"]:
            failures.append(f"collect found {sorted(rows)}")
        if rows["a/one"]["median_ns"] != 100.0:
            failures.append("median not carried through")
        if rows["a/one"]["samples"] != 3:
            failures.append("sample count not carried through")
        lo, hi = rows["a/two"]["median_ci_ns"]
        if abs(lo - 180.0) > 1e-9 or abs(hi - 220.0) > 1e-9:
            failures.append(f"confidence interval not carried through: {lo}, {hi}")

        entry = build_entry(root, "deadbeef", ("a/one", "a/two"), "m")
        if list(entry["benchmarks"]) != ["a/one", "a/two"]:
            failures.append("roster order not preserved")
        if not entry["environment"]["arch"]:
            failures.append("environment block is empty")

        # The roster pin: a row the harness no longer emits must be fatal.
        # `die` writes its diagnosis to stderr, which is the point of it —
        # muffled HERE only so a passing selftest looks like one.
        with open(os.devnull, "w", encoding="utf-8") as devnull:
            saved, sys.stderr = sys.stderr, devnull
            try:
                build_entry(root, "deadbeef", ("a/one", "a/renamed"), "m")
            except SystemExit:
                pass
            else:
                failures.append("a moved roster did NOT fail")
            finally:
                sys.stderr = saved

    for f in failures:
        print(f"SELFTEST FAILED: {f}", file=sys.stderr)
    if failures:
        return 1
    print("criterion-emit selftest: ok")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(add_help=True)
    ap.add_argument("--criterion-dir", default="benches/target/criterion")
    ap.add_argument("--out")
    ap.add_argument("--commit", default="")
    ap.add_argument("--expect", action="append", default=None,
                    help="one roster row; repeatable. Defaults to DEFAULT_ROSTER.")
    ap.add_argument("--method", default=DEFAULT_METHOD)
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()

    if args.selftest:
        return selftest()
    if not args.out:
        die("--out is required (or pass --selftest)")

    criterion_dir = Path(args.criterion_dir)
    if not criterion_dir.is_dir():
        die(f"no criterion output at {criterion_dir} — did `cargo bench` run?")
    roster = tuple(args.expect) if args.expect else DEFAULT_ROSTER
    entry = build_entry(criterion_dir, args.commit, roster, args.method)
    Path(args.out).write_text(json.dumps(entry, indent=1, sort_keys=True) + "\n")
    print(f"criterion-emit: wrote {args.out} ({len(entry['benchmarks'])} rows)")
    for row, stats in entry["benchmarks"].items():
        print(f"  {row:34s} {stats['median_ns'] / 1e6:12.6f} ms")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
