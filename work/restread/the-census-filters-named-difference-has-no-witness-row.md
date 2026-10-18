---
id: the-census-filters-named-difference-has-no-witness-row
kind: issue
title: The census filter's one named difference from the exact sweep (a pruned pair's carrier-stage escalation is not raised) has no row left that shows it
status: open
opened: 2026-10-18
priority: P3
cost: E
---

## What

The census's box filter differs from the exact sweep in one named
way: a pair the padded boxes prove apart is decided apart, so a
carrier-stage escalation the exact sweep would raise about it is not
raised (`crates/topo/README.md`, the census paragraph; `Trees`'s
contract in `crates/topo/src/census.rs`). The row
`census::tests::a_carrier_stage_outcome_on_separated_entities_is_the_filters_to_answer`
pinned that difference on two unit cubes, the second at
`(10, 5·10⁻⁹, 0)`. Its last remaining exact-sweep escalation was
`curve_pair_pose_offset`, from `geom_brep::curve_pair::meet` reading
two segments on one line far apart by their ends against the lines.
Branch `restread/meet-apart-on-one-line` fixed that, so the exact
sweep now decides those cubes as well. The row was re-baselined to
`separated_cubes_on_in_band_edge_lines_read_apart_in_both_sweeps`.

A scratch sweep on that branch found no pose where the exact sweep
escalates. It tried only six axis-aligned separated-cube poses
(offsets in band along y, z, and both, at faces' planes and at edge
lines). It did not try the members `Trees`'s doc names: `contfp`'s
`RayExhausted` refusal and ray-walk escalations on a vertex coplanar
with a far face, and its `Uncrossable` refusal within the reach of a
spline edge. Those are the first places to look for a witness. Until
one is found, no row shows the filter answering a question that the
exact sweep cannot.

## The shape to give

A row whose exact sweep still raises a carrier-stage outcome on
separated entities, if one exists (a curved pair, perhaps:
`work/cpair/the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`
measures two such escalations in `meet`). Or, if RESTREAD's direction
holds and the exact sweep comes to decide every such pair, the README
paragraph can say so, and this difference no longer needs naming.
