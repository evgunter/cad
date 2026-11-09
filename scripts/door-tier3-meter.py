#!/usr/bin/env python3
"""Measure the boolean door: what each call costs, and what its gate costs and says.

Runs the given crates' suites (the `ci` nextest profile) with topo's
`door-tier3-meter` feature on, which records one line per call through
`topo::boolean_op_with` (`crates/topo/src/boolean/door_meter.rs`), and
prints one row per corpus: the door's summed time, the result gate's
(tier 3, then the census over the result's own contacts), and the
results the gate refuses.

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
    "door_us",
    "gate_us",
    "outcome",
    "verdict",
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
    gated = [r for r in rows if r["verdict"] != "-"]
    refused = [r for r in gated if r["verdict"] != "ok"]
    door = sum(int(r["door_us"]) for r in rows)
    gate = sum(int(r["gate_us"]) for r in rows)
    ratios = sorted(int(r["gate_us"]) / max(1, int(r["door_us"])) for r in gated)
    outcomes = Counter(r["outcome"] for r in rows)
    print(
        f"| {crate} | {len(rows)} | {len(gated)} | {door / 1e6:.2f} s | "
        f"{gate / 1e6:.2f} s | {gate / max(1, door):.0%} | "
        f"{statistics.median(ratios):.0%} / {ratios[int(0.9 * len(ratios))]:.0%} / "
        f"{ratios[-1]:.0%} | {len(refused)} |"
    )
    print(f"    outcomes: {dict(outcomes.most_common())}")
    for (test, op_name, verdict), n in Counter(
        (r["test"], r["op"], r["verdict"][:120]) for r in refused
    ).most_common():
        print(f"    gate refuses: {n} × {test} {op_name}: {verdict}")


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
        "| corpus | calls | gated results | door time | gate time | gate / door | "
        "gate / door per result, median / p90 / max | gate refuses |"
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
