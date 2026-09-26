---
id: a-third-spelling-of-cut-every-span-into-splits-pieces
kind: issue
title: A third spelling of "cut every nonempty span into splits pieces", and the concept's home is below both crates
status: dispatched
opened: 2026-09-22
priority: P1
cost: D
branch: encl/equal-split-points-home
---


Filed by TESS-2 from a blinded review of its head. On ENCL's slate
because `crates/geom-brep/src/patch_bound.rs` is ENCL's path and two of
the three spellings are in `crates/geom-brep/`.

## What

"Cut every nonempty span of a knot vector into `splits` equal pieces,
skipping any point floating point collapses onto a span end" is now
written three times:

* `geom_brep::patch_bound::split_points` — the shipped one, whose doc
  already records the near-twin below and says the unification is Track
  R's ground (C-m/D30, gated behind #723);
* `geom_brep::props::quad`'s `knot_aligned_cuts` — the near-twin that
  doc names, with its own sliver guard;
* and, since TESS-2, `geom_core::spline::algebra`'s test module, twice
  (`the_ring_applier_stays_in_step_and_near_the_described_hull` and
  `the_convex_form_bulges_by_the_ratios_own_rounding`) plus
  `geom_core::spline::net`'s test module once, because `geom-core` is
  BELOW `geom-brep` and cannot call the shipped spelling at all.

That last point is the new information and it is the reason this row is
not just "unify two callers". The concept's natural home is
`geom_core::spline` — beside `algebra::refine_plan`, which is what every
caller feeds it to — and from there `patch_bound` and `props::quad` can
both reach it. As long as it lives in `geom-brep`, any `geom-core` row
that needs a refinement schedule writes a fourth copy.

## What a fix looks like

A `geom_core::spline::algebra::equal_split_points(kv, splits)` (or a
method on `KnotVector`), with the sliver rule and the "refinement is a
tightening, never a correctness condition" sentence that both shipped
spellings already carry. `patch_bound::split_points` becomes a re-export
under the name its consumers use; `props::quad`'s sliver guard is the
part that needs a reading, since it is a subdivision of a parameter
RANGE rather than of a whole vector.

Track R's #723 gate is about consolidating the two `geom-brep`
spellings. This row is narrower and is not gated by it: moving the
concept DOWN is what unblocks `geom-core`'s own callers, and it can land
before any decision about `props::quad`'s guard.
