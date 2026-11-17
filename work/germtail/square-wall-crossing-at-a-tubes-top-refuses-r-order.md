---
id: square-wall-crossing-at-a-tubes-top-refuses-r-order
kind: issue
title: The square-wall arm refuses R-order where the wall crosses the tube at its top or bottom parallel, though the section is regular there
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [radial-hole-through-a-tube-has-no-section-arm, 4530]
---

## What

`section_cert.rs` `square_folds` and `across_strip` refuse
`Section::Unordered` (R-order) when a fold of the wall's trace `K` on a
tube circle, or a crossing of `K` with a strip edge `z = ±r`, lies
within the band of the circle's top or bottom point `(±R, ±r)`
(predicates `section_torus_square_wall_fold_half` and
`section_torus_square_wall_edge_over`). The carriers cross
transversally there unless `K` is tangent to the tube circle (which
the `_tube` margin refuses R-tan first), so the section is regular and
its topology is stable under every pose the band admits: the refusal
is the arm declining to pick a side, not a pinch.

Measured on PR 4530's fix pass (1,200 poses of the search's family
"a strip crossing at a tube circle's top", offsets log-uniform from
`1e-12` to `0.5` of `min(r, rc)`, seed `0x5eed`): 244 refuse R-order at
ε = 1e-9, 408 at 1e-6, 96 at 1e-12; every pose within the escalation
band of a top point refuses (1,200 of 1,200 at 1e-9).

## The fix it wants

Answer by a side the band admits, the way `reaches_edge` answers a
strip-edge tangency beyond the parallel: read every near-top crossing
as standing just outside the parallel (`|y| > R`), which is one
perturbation of the pose (K moved so each such crossing steps outward)
when the near-top crossings are realizable together. What needs
proving is that joint realizability: up to four top and bottom points
can lie near `K` at once, and a reading no single nearby pose produces
could pair the folds wrongly. A fold and the strip crossing beside it
agree to second order in their offset from the top point (`K` against
`T` there: `z − r ≈ k (y − y_e)` against `−(y − σR)²/2r`), which is
the start of that argument.

Rows: `section_cert_square_rows.rs`
`a_crossing_at_a_tubes_top_refuses_r_order` pins today's refusal and
turns into the answered row.

## A seed of the square search that reds on this family (2026-11-17, SECTARMS)

`topo`'s `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
drew `CAD_FUZZ_SEED=0x2d854e95cb708d44` at `CAD_FUZZ_EFFORT=1` on branch
`sectarms/periodic-loop-carrier` and failed with one mismatch, which
replays there:

    a strip crossing at a tube circle's top: R 0.015043003400221869,
    r 0.007531328765283513, (e, z0, rc) (0.01590952267596522,
    -0.005535154178584106, 0.0021761347361288),
    tilt 0.00000000000644107827255586: R-tan at
    Degeneracy { tangent: 0.00024737568798874343, top: 1.4351601400341973e-9 }:
    Tangent("section_torus_square_wall_fold_half")

The arm answers `Section::Tangent` from the classification, before any
section is marched or fitted, so that branch's change (a closed marched
loop's carrier) is not on its path. Not replayed on main: read, not run.
