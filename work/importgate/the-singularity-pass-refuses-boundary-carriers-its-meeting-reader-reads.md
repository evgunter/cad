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
boundary carrier is a line or a circle (`foreign_carrier`: the
normalization "cuts only faces bounded by lines and circles, and does
not yet cut this one"). That limit was the pass's own crossing reader's.
`first_hit` now reads each meridian against the boundary through
`curve_pair::meet`, whose table reads ellipse pairs in closed form and
spline and spiric pairs by the certified walk, so the gate is now only
the pass's scope.

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

- 2026-11-25 — from `lathe/import-wrap-tip` (LATHE): the pinned row no longer pins this refusal. Open CASCADE's oblique cone states its apex as the tip of its seam, the wrap tip, which import now adopts as stated; it imports and is re-pinned as `an_obliquely_cut_open_cascade_cone_imports_as_stated`. The gate itself is unchanged and still refuses a face holding its pole or apex inside an ellipse or spline boundary. No committed row reaches that gate now; the cone with no seam is the witness to build. `cut` has gone: the interior disposition now `splice`s one wrap edge in.
