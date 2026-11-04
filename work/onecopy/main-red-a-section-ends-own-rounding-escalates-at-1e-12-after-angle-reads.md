---
id: main-red-a-section-ends-own-rounding-escalates-at-1e-12-after-angle-reads
kind: issue
title: Main red at 1e-12: section_end_poses' own-rounding row escalates its map residual since PR 6482 (angle-reads)
status: open
opened: 2026-11-04
priority: P0
cost: E
---


Filed by the PCERT orchestrator. PCERT's spline-chart incidence lane (PR 6498) found and bisected it.

## What

At ε 1e-12 (`CAD_TOLERANCE_EPS=1e-12`, CI's `ci` profile), `sweep::all section_end_poses::a_section_ends_own_rounding_is_a_kernel_limitation` fails at `crates/sweep/tests/section_end_poses.rs:398`:

    (0.3, 0, 1e-5, 0.3): Some(Pcurves(Certify { half_edge: HalfEdgeKey(35v1),
      error: Escalated { check: MapResidual, sample: 2, cause: Indeterminate {
      margin: MarginDiag(Value(3.6379788291145696e-12, ..)), band: Band { zero: 1e-12, escalate: 1e-11 },
      predicate: Some("pcurve_map_residual") } } }))

## Bisect (`git bisect --first-parent` on main)

- good: `e9eaae65`;
- first bad: `a6f0b5e9`, "Merge pull request #6482 from evgunter/onecopy/angle-reads";
- the panic is byte-identical on main `e931c7d3` alone, with no PCERT code.

Every PR that runs sweep at 1e-12 is red on it, beside `main-red-the-needle-refuses-its-meet-at-1e-12-after-meet-pose-order`.

## Owed

Root-cause why 6482's anchored angle read moves this section end's map residual to `3.6e-12` (in band at 1e-12) where it was decided before, fix it, and confirm the row at all three ε.
