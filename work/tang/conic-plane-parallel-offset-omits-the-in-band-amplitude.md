---
id: conic-plane-parallel-offset-omits-the-in-band-amplitude
kind: issue
title: a conic read parallel to a face plane serves its offset without the in-band amplitude beside it
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`split_conic_plane_parallel` (`crates/topo/src/splitting/classify.rs:507`)
routes a conic whose amplitude (its tilt times its radius) reads Zero
to `ConicPlaneMeet::Parallel { offset }`. `topo::boolean::reduce` then
serves "the edge lies in the face plane" on
`bool_conic_face_plane_offset` Zero (`boolean/reduce.rs:1318`) and
"clear" on `bool_arc_plane_side` Positive (`reduce.rs:3163`), neither
with the in-band amplitude beside the offset.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.
