---
id: domain-grid-homing-residue
kind: issue
title: What the domain-uniform grid homing left: knot_aligned_cuts, the grid-plus-sliver predicate spelled twice, hand-spelled interior filters
status: dispatched
opened: 2026-09-28
priority: P4
cost: E
---

Filed by `encl/domain-uniform-grid` (PR 3348), which homed the
domain-uniform refinement grid as
`geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`
and moved the sliver clearance to `algebra::SLIVER_CLEARANCE_ULPS`.
What that unit declined or left, so it is not lost with the PR:

## `knot_aligned_cuts` is not homed

`geom_brep::props::quad::knot_aligned_cuts` spells the same
`lo + (hi − lo)·i/pieces` grid behind the same clearance, but it is
not `domain_grid_points`: its range is the trim rectangle's `[lo, hi]`,
not a knot vector's domain; its `knots` are a raw slice that may be a
derivative's (`derivative_knot_slice`); a grid point must also clear
the range ENDS (`lo`, `hi` are in its mandatory set); and the same
predicate is shared with `block_edges` so blocks and cells agree by
construction. Homing it means a range-and-slice primitive under the
knot-vector one, which is a design step, not a substitution.

## The grid-plus-sliver predicate is spelled twice

* `algebra.rs:652-661` (`domain_grid_points`: `sliver` from
  `(hi − lo).abs()·ulps·ε`, then `(t − k).abs() > sliver` over the
  interior knots);
* `quad.rs:2795-2800` (`knot_aligned_cuts`' `clear`: the same
  expression against its mandatory set, ends included);

and the grid expression a third time at `quad.rs:2706-2713`
(`block_edges`, `lo + (hi − lo)·b/QUAD2_HULL_BLOCKS`, ends included).

## Hand-spelled interior-knot filters

`kv.knots().iter().copied().filter(|k| *k > lo && *k < hi)` where the
typed door `KnotVector::interior_knots` (`knots.rs:823`) says the same
thing, in `props/quad.rs`:

* ~1059 (`interior`, against `kv.domain()`: the door's exact case);
* ~3231 and ~3661 (the `interior` closures, against a caller's
  `(lo, hi)`: the door's case only when that range is the domain;
  otherwise a range filter the door does not express);
* ~4049 (`bezier_blocks`' `breaks`, against `kv.domain()`).

`domain_grid_points` itself reads `interior_knots`. `quad.rs:1706`
filters a raw slice inline and `quad.rs:2775` a raw slice
(`knot_aligned_cuts`), which the door cannot serve.
