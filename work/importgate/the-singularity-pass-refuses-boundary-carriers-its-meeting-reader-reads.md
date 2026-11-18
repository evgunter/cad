---
id: the-singularity-pass-refuses-boundary-carriers-its-meeting-reader-reads
kind: issue
title: the singularity pass refuses a face bounded by an ellipse or a spline, a limit of the meeting reader it no longer has
status: open
opened: 2026-11-18
priority: P3
cost: M
---


Found by `importgate/singularity-meet`, which moved the pass's meetings
onto `geom_brep::curve_pair::meet`.

## What

`crates/step-import/src/normalize/singularity.rs`, `require_read_carriers`,
refuses a sphere or cone face holding its pole or apex unless every
boundary carrier is a line or a circle (`foreign_carrier`: "a carrier
the chart-singularity normalization reads no crossing of (it reads lines
and circles)"). That was the limit of the pass's own crossing reader.
`first_hit` now reads each meridian against the boundary through
`curve_pair::meet`, whose table reads ellipse pairs in closed form and
spline and spiric pairs by the certified walk, so the reason the gate
gives is no longer the pass's.

Pinned on the refusal today:
`crates/step-import/tests/review_pole_vertex_probes.rs`,
`an_obliquely_cut_open_cascade_cone_refuses_typed` (an Open CASCADE cone
cut by an oblique plane: an ellipse rim).

## Fix

Lift the gate where the rest of the disposition holds for the carrier:
`split_at_point` re-derives a split edge's interval through
`crate::geometry::endpoint_params`, and `mint_spoke` and `cut` take the
landing as a vertex, so each needs reading for an ellipse or spline edge
before the refusal goes. Re-pin the oblique cone on what it imports.
