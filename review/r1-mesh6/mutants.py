#!/usr/bin/env python3
"""Mutation battery for the issue 897 censuses: apply one mutant, run the
named unit rows, restore.

Adopted from the R1 review of PR 1545 and RETARGETED at the fix-pass head,
which moved the shared rule and the identified-vertex census into `walk.rs`;
the reviewer's anchors named their pre-refactor homes and no longer applied.
M8 is new — the `n > 3` mutant the reviewer found SURVIVING, now killed by
`three_uses_of_an_identified_edge_is_already_the_defect`. Lane path is derived
from this file's location, and the body is spelled to pass this repo's ruff
gate; the method is the reviewer's.

Files are restored from `git show HEAD:<path>`, so the tree is byte-identical
afterwards. Usage: mutants.py [name ...]   (default: all)
"""

import os
import subprocess
import sys
from pathlib import Path

LANE = Path(__file__).resolve().parents[2]
CURVED = "crates/mesh/src/curved.rs"
WALK = "crates/mesh/src/walk.rs"
TESS = "crates/mesh/src/tessellate.rs"

MUTANTS = {
    "M1_no_uv_repeat": (
        WALK,
        "if seen.insert(id, (u, v)).is_some_and(|p| p != (u, v)) {",
        "if seen.insert(id, (u, v)).is_some_and(|_p| false) {",
    ),
    "M2_threshold_4": (
        WALK,
        "uses.iter().find(|&(_, &n)| n > 2)",
        "uses.iter().find(|&(_, &n)| n > 4)",
    ),
    "M3_drop_pole_keep": (
        CURVED,
        "    identified.extend(polygon.iter().filter(|e| e.pole).map(|e| e.id));\n",
        "",
    ),
    "M4_old_pole_filter": (
        WALK,
        "if identified.contains(&a) || identified.contains(&b) {",
        "if false {",
    ),
    "M5_chord_gt2": (
        TESS,
        "uses.iter().find(|&(_, &n)| n != 2)",
        "uses.iter().find(|&(_, &n)| n > 2)",
    ),
    "M6_no_mark": (
        TESS,
        "            if a < shared_below\n                && b < shared_below\n                && let Some(n)",
        "            if let Some(n)",
    ),
    "M7_bits_compare": (
        WALK,
        "if seen.insert(id, (u, v)).is_some_and(|p| p != (u, v)) {",
        "if seen.insert(id, (u, v)).is_some_and(|p| "
        "(p.0.to_bits(), p.1.to_bits()) != (u.to_bits(), v.to_bits())) {",
    ),
    "M8_threshold_3": (
        WALK,
        "uses.iter().find(|&(_, &n)| n > 2)",
        "uses.iter().find(|&(_, &n)| n > 3)",
    ),
}

BATTERY = [
    "local-scripts/with-build-slot.sh",
    "--",
    "cargo",
    "test",
    "-p",
    "mesh",
    "--lib",
    "--",
]

ROWS = [
    "identified_ids",
    "a_seam_vertex",
    "three_uses_of_an_identified_edge",
    "the_full_2pi",
    "a_repeat_at_the_same",
    "unpaired_chord",
    "a_boundary_the_second",
    "a_shared_chord",
    "a_seam_edge_traversed",
    "a_segment_no_face",
    "ids_at_or_above",
]


def restore(rel: str) -> None:
    blob = subprocess.run(
        ["git", "show", f"HEAD:{rel}"],
        capture_output=True,
        text=True,
        check=True,
        cwd=LANE,
    ).stdout
    (LANE / rel).write_text(blob)


def main() -> None:
    os.chdir(LANE)
    env = dict(os.environ, CARGO_TARGET_DIR=str(LANE / "target"))
    for name in sys.argv[1:] or MUTANTS:
        rel, old, new = MUTANTS[name]
        restore(rel)
        src = (LANE / rel).read_text()
        assert src.count(old) == 1, (name, src.count(old))
        (LANE / rel).write_text(src.replace(old, new))
        proc = subprocess.run(
            [*BATTERY, *ROWS],
            env=env,
            capture_output=True,
            text=True,
            check=False,
        )
        out = proc.stdout + proc.stderr
        red = [
            line.split()[1]
            for line in out.splitlines()
            if line.startswith("test ") and line.rstrip().endswith("FAILED")
        ]
        broken = "COMPILE-ERROR" if "error[" in out or "could not compile" in out else ""
        print(f"{name}: exit={proc.returncode} {broken} red_rows={red or 'NONE'}")
        sys.stdout.flush()
        restore(rel)
    print("restored; git status:")
    subprocess.run(["git", "status", "--short", "crates/mesh/src"], check=False)


if __name__ == "__main__":
    main()
