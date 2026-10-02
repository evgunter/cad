#!/usr/bin/env python3
"""Measure tier 3 at the boolean door: what it would cost and refuse.

Runs the `topo` and `sweep` suites (the `ci` nextest profile) with
topo's `door-tier3-meter` feature on, which records one line per result
the door builds (`crates/topo/src/boolean/door_meter.rs`), and prints
the table `work/reach/boolean-door-tier-3-waits-on-the-description-gap.md`
reports.

    python3 scripts/door-tier3-meter.py [--crates topo sweep] [--records DIR [--skip-run]]

Set `CAD_TOLERANCE_EPS` and `CARGO_TARGET_DIR` as for any suite run.
"""

import argparse
import os
import statistics
import subprocess
import sys
import tempfile
from collections import Counter

FIELDS = [
    "test",
    "op",
    "scalar",
    "faces",
    "op_us",
    "tier3_us",
    "backstop_us",
    "backstop_ok",
    "verdict",
    "a_ok",
    "b_ok",
]


def run(crate: str, path: str) -> None:
    # The meter's sink is compiled in (`env!`), so the path rides on the
    # build: a fresh file per run, which a concurrent run cannot share.
    env = dict(os.environ, CAD_DOOR_TIER3_METER_OUT=path)
    subprocess.run(
        [
            "cargo",
            "nextest",
            "run",
            "-p",
            crate,
            "--profile",
            "ci",
            "--no-fail-fast",
            "--features",
            "topo/door-tier3-meter",
        ],
        check=False,
        env=env,
    )


def summarize(crate: str, path: str) -> None:
    try:
        with open(path, encoding="utf-8") as f:
            rows = [
                dict(zip(FIELDS, line.rstrip("\n").split("\t"), strict=True))
                for line in f
            ]
    except FileNotFoundError:
        print(f"{crate}: the meter recorded nothing at {path}", file=sys.stderr)
        sys.exit(1)
    refused = [r for r in rows if r["verdict"] != "ok"]
    shipped = [r for r in refused if r["backstop_ok"] == "true"]
    clean_operands = [r for r in shipped if r["a_ok"] == "true" and r["b_ok"] == "true"]
    op = sum(int(r["op_us"]) for r in rows)
    t3 = sum(int(r["tier3_us"]) for r in rows)
    bs = sum(int(r["backstop_us"]) for r in rows)
    ratios = sorted(int(r["tier3_us"]) / max(1, int(r["op_us"])) for r in rows)
    print(
        f"| {crate} | {len(rows)} | {len(refused)} | {len(shipped)} | "
        f"{len(clean_operands)} | {t3 / op:.0%} | "
        f"{statistics.median(ratios):.0%} / {ratios[int(0.9 * len(ratios))]:.0%} / "
        f"{ratios[-1]:.0%} | {bs / op:.0%} |"
    )
    for (test, op_name), n in Counter(
        (r["test"], r["op"]) for r in shipped
    ).most_common():
        print(f"    shipped, tier 3 refuses: {n} × {test} {op_name}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--crates", nargs="+", default=["topo", "sweep"])
    parser.add_argument(
        "--skip-run", action="store_true", help="summarize --records without running"
    )
    parser.add_argument(
        "--records",
        help="the directory the per-crate records go to (default: a fresh one)",
    )
    args = parser.parse_args()
    if args.skip_run and not args.records:
        parser.error("--skip-run reads an earlier run's --records directory")
    out = args.records or tempfile.mkdtemp(prefix="door-tier3-meter-")
    print(
        "| corpus | results | tier 3 refuses | of those, shipped | shipped with "
        "tier-3-clean operands | tier 3 / op time | median / p90 / max | backstop / op |"
    )
    print("|---|---|---|---|---|---|---|---|")
    for crate in args.crates:
        path = os.path.join(out, f"{crate}.tsv")
        if not args.skip_run:
            if os.path.exists(path):
                os.remove(path)
            run(crate, path)
        summarize(crate, path)


if __name__ == "__main__":
    main()
