---
id: the-carved-balls-clearance-row-is-vacuous-on-main
kind: issue
title: tilted_sphere_pair_k_rows' clearance row is red on main under --features probe: no bool_circle_curved_clearance is asked, so the row calls itself vacuous
status: open
opened: 2026-10-03
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
