---
id: measure-what-an-exact-nurbs-wall-of-revolution-does-today
kind: unit
title: Measure, on hosted CI, what a body with an exact rational NURBS wall of revolution does today at import and rest, measure, mesh, export and the boolean
status: open
opened: 2026-11-22
priority: P1
cost: M
parent: an-ovoid-cannot-be-authored
---



Part 3's first unit (`an-ovoid-cannot-be-authored`, Ev PR 7289). No
body with a NURBS wall of revolution exists in the tree
(`crates/sweep/src/revolve/surfaces.rs`'s `wall_surface` has only
plane, cylinder, cone, sphere, torus and spindle arms), and neither
designer measured what one does: their local builds never finished.
This unit is the measurement. It builds nothing that merges.

## What is done

1. A throwaway branch (`splinewall/ovoid-probe`) carries the designer's
   unrun probe, `crates/step-import/tests/ovoid_probe.rs`, taken from
   the branch `splinewall/probe-nurbs-wall-of-revolution` (pushed, not
   for merge), wired into `crates/step-import/tests/all.rs`. Its PR is
   marked ready, so the gate runs it, and is closed unmerged once read.
2. The probe builds a quarter-turn sphere wedge through the public
   revolve, writes it as STEP, scales every point's `y` by `a`, swaps
   `SPHERICAL_SURFACE` for the exact rational biquadratic net (two
   meridian spans × one turn span) and the two meridian circles for
   its boundary rows, and re-imports. It runs at `a = 1` (the analytic
   control, which should promote to `Sphere`) and `a = 1.6`. It prints
   one line per stage: IMPORT (tier 3, face kinds), MEASURE (against
   `π·a/3`), TESSELLATE (`check_mesh`, signed volume), EXPORT, and BOOL
   (∪, ∖, ∩ against a brick across the wall at the equator, a slab over
   the north pole, and a brick inside).
3. Add a **full-turn** variant (four quarter spans in `u`, so the chart
   is closed in the angle), because the quarter wedge never reaches
   SPLICE's closed-seam rows, and a revolve door mints full turns.
4. The result, every stage at both aspects and both turn extents, is
   appended to this row and to `the-egg-and-the-elliptic-prism-meet-the-bar`,
   and each refusal is attributed to the row that names it or filed
   where it lands.

## Check first

1. **The probe's output must reach the CI log.** nextest shows a passing
   test's output only with `success-output` (`.config/nextest.toml`
   sets `failure-output` only), so end the census with a deliberate
   `panic!` carrying the lines, or set `success-output` on the
   throwaway branch.
2. **The probe swaps entities by number** (`#46`, `#14`, `#35`, `#15`,
   `#36`, `#51`, read off one export of the wedge). Confirm on today's
   main that the wedge's export still numbers them so (`dump_sphere_wedge`
   prints it); if the numbering moved, the swap writes a different file
   with no error. Fix the numbers, or find the entities by type.
3. Import splits the net at its C⁰ knot (the double knot at the
   equator) into two walls, so this is not exactly what a revolve door
   would mint; say so beside each result.

If a stage panics rather than refusing, that is a finding on the
owning program (a kernel bug), filed there with the probe as its
reproduction.

## What it settles

What part 3 costs on the spline lanes. Each stage where `a = 1.6`
refuses or goes wrong, and `a = 1` does not, is a cost a
`Spheroid` kind does not pay and the NURBS walls of part 3 do. The
three SPLICE rows that name the weak points:
`a-closed-charts-seam-continuity-has-no-home` (P3),
`a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole` (P2),
`a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`
(P2, parked). And CHARTTAIL's `a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint`
(P1) for the poles. Main's `[surfaces]` now says a NURBS net's
collapsed boundary row "is a chart singularity at that point, decided on
the stored values" (PR 7287); `chart_singularity::spline_collapsed_row`
still decides "no point of the row decided apart" at the band, so the
probe measures the code before that ruling's build.

## Rows and ground

- Closes nothing; its result re-cuts `the-egg-and-the-elliptic-prism-meet-the-bar`.
- Touches only the throwaway branch. No announcement owed; the results
  go on SPLICE's and CHARTTAIL's logs where they reach those rows.
- Runs at once, beside OVOID's first two units.
- Review: **orchestrator's read**: nothing merges, and the reading is
  the substance. Difficulty **S**.
