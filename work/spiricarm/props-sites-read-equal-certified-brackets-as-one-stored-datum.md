---
id: props-sites-read-equal-certified-brackets-as-one-stored-datum
kind: issue
title: Props sites read two equal certified brackets as one stored datum without requiring a point
status: open
opened: 2026-11-11
priority: P2
cost: M
---


## Finding

Filed from PR 6846's review (the endpoint census fix pass). The class only; nothing here is
fixed.

Some props sites decide that two values are "the same stored datum" by comparing two
certified brackets end for end, without requiring either to be a point:

- `ProjectedEdgeQ::of_image`'s torus arm
  (`crates/geom-brep/src/props/quad/projected_edge.rs:51`): the image's major radius against
  the chart's, `major.is_certified() && own.is_certified()`, then `lo == lo` and `hi == hi`.
- `topo`'s spiric wall entry, its `same` closure
  (`crates/topo/src/props/quad_lane.rs:861`): the edge's and the image's radii against the
  torus's, `is_certified()` on both, then `lo == lo` and `hi == hi`.

The same decision is spelled with a point required elsewhere:

- `section_bounds`' cylinder pair, its `same` closure
  (`crates/geom-brep/src/intersect/section_bounds.rs:471`): `x.lo() == x.hi()` before the
  end-for-end comparison;
- `chart_region`'s `exact_pair` (`crates/topo/src/chart_region.rs:2492`): both point
  brackets, equal, finite. (The review cited this as an instance; at the time of filing it
  requires a point, so it belongs with the contrast.)

and `Interval::same_enclosure` (`crates/geom-core/src/interval.rs:264`) states that bracket
identity is "never a coincidence decision".

Two non-point brackets with equal ends can hold different true values. Whether such a
mismatch, inside one bracket, can reach a soundness claim at either site (a term read about
one torus's core where the image's inverse names another) is the open question.

## Repair

Settle whether the bracket-equality spelling is sound at each site: either require a point
(as `section_bounds` and `chart_region` do), or argue at the site why a mismatch inside the
bracket cannot move what the lane certifies. Sweep for the shape (`lo() == … lo()` beside
`hi() == … hi()` with no `lo() == hi()`), not the symbol.
