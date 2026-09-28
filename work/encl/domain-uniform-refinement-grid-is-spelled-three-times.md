---
id: domain-uniform-refinement-grid-is-spelled-three-times
kind: issue
title: A domain-uniform refinement grid that skips stated knots is spelled by hand three times
status: review
opened: 2026-09-28
priority: P4
cost: E
---


Filed by the sweep on `encl/equal-split-plan-chain`, which gave
"`equal_split_points`, then `refine_plan_homogeneous`" one home
(`geom_core::spline::algebra::equal_split_plan`). The second pass,
shaped at that sweep's blind spot (a schedule written out by hand
rather than through `equal_split_points`), found this sibling schedule
instead.

## What

"Cut the whole DOMAIN `[lo, hi]` into `N` equal pieces, skip a point
that is already a knot, refine there" is written out by hand three
times, each with its own constant and its own spelling of the skip:

* `geom_brep::props::quad::refine_dir` — `d0 + (d1 − d0)·k/QUAD2_REFINE_SPANS`,
  kept when `t > d0 && t < d1 && !kv.knots().contains(&t)`, then
  `refine_plan(kv, &vec![1.0; count], &add)` (which is
  `refine_plan_homogeneous` spelled out);
* `geom_brep::ssi::certify::refined` — `lo + (hi − lo)·i/SSI_CERT_SPANS`,
  kept when `kv.multiplicity_of(t).is_none()`, then `refine_knots`;
* `geom_brep::edge_nurbs::localized`'s inner `breaks` —
  `lo + (hi − lo)·i/PXN_WALL_SPANS`, kept when
  `kv.multiplicity_of(t).is_none()`, then `refine_knots_u`/`_v`.

The three do not agree on when to refine at all. `certify::refined`
and `localized::breaks` skip the whole grid once
`kv.control_count() >= N + kv.degree()` (the vector is already about as
fine as the grid); `refine_dir` has no such cut-off and inserts its
grid into any vector. That difference is behaviour, not spelling, so a
homing has to keep it as a caller's choice or rule on it; it must not
unify it silently.

The same range-uniform grid is also spelled for CUTS rather than for
refinement, twice in `props/quad.rs`, both already behind a sliver
guard (`SLIVER_CUT_ULPS`: a grid point within `8·ε` of the range's
width from a knot is dropped, and the knot stands): `knot_aligned_cuts`, the quadrature cell
cuts, and `bezier_blocks`' breaks, which then raise each break to full
multiplicity for per-span blocks.

A test re-spells the first (`geom-brep/tests/review_r1_rational_probes.rs`,
`diag_refine_half_circle`, "exactly what refine_dir does").

This is NOT the equal-split schedule: `equal_split_points` restarts its
grid at every knot, so a knot is a span end by construction; this grid
is blind to the knots and has to be told to skip them. Both skips are
the same exact-`f64`-equality test in two spellings, and exact equality
is what `refine-dir-hairline-knot-insertion` (NURBS) measures failing:
a stated knot one ulp off a grid point survives the skip and leaves a
hairline span that de Boor divides by.

## What a fix looks like

One `geom_core::spline::algebra` home for the domain-uniform schedule,
with one skip guard, beside `equal_split_points`. What that guard
should be (exact, or a sliver guard) is the hairline row's question,
not this one's; homing the schedule first means that answer lands once
instead of three times. `props::quad::knot_aligned_cuts` is the in-tree
precedent for that question: it spells the same grid for quadrature
cuts behind `SLIVER_CUT_ULPS`, and it and `bezier_blocks` are candidate
consumers of the same home. The owners differ (`props/quad.rs` is PROPS/QUAD,
`ssi/certify.rs` SSI, `edge_nurbs.rs` ISO), which is why the row sits
with the schedule's home rather than with any one of them.
