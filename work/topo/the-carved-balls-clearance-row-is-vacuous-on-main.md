---
id: the-carved-balls-clearance-row-is-vacuous-on-main
kind: issue
title: tilted_sphere_pair_k_rows' clearance row is red on main under --features probe: no bool_circle_curved_clearance is asked, so the row calls itself vacuous
status: closed
opened: 2026-10-03
pr: 3957
branch: cleave/all-features-row
closed: 2026-10-03
---


## What

`crates/sweep/tests/tilted_sphere_pair_k_rows.rs`,
`a_carved_balls_meridian_fragments_record_no_clearance_charge`, fails on
`main` (measured at `c21be5b5`'s successor, 2026-10-03, by TANG while
validating PR 3851) and identically on `tang/pierce-ring`:

    no clearance was asked: the row would be vacuous

Every op builds, but no `bool_circle_curved_clearance` sample is
recorded for any of the three carved-ball poses, so the row's own
non-vacuity guard (line 93) fires. The row closed
`work/topo/circle-clearance-records-its-charge-on-an-arc-ending-on-the-carrier.md`;
a later change on `main` stopped the poses from asking the clearance at
all. The suite runs only under `--features probe`, so the hosted CI does
not see it (`work/ciw/tests-red-under-all-features-never-run-by-ci.md`).

The fix is either a pose that still asks the clearance, or the row
re-stated for what the boolean now does instead; which one is the
owner's call.

## Measured and fixed (CLEAVE, 2026-10-03)

The poses still ask the clearance; it records under
`bool_conic_curved_clearance` since #3805 renamed it, and the row
filtered on the old name. The row now names the new one and passes at
ε 1e-6, 1e-9 and 1e-12. The same finding is
`work/cleave/a-sweep-row-fails-under-all-features-on-main.md`, which
carries the measurement.
