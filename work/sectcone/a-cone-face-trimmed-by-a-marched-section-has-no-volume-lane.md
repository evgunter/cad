---
id: a-cone-face-trimmed-by-a-marched-section-has-no-volume-lane
kind: issue
title: A cone face trimmed by a marched section curve has no volume lane: the finish refuses VolumeUncomputable (Unimplemented)
status: open
opened: 2026-11-24
priority: P1
cost: M
refs: [the-join-reads-the-cone-pairs-on-the-value]
---


Found by the join's cone-pair unit (`the-join-reads-the-cone-pairs-on-the-value`, PR 7735).

## Measured

`crates/sweep/tests/reach_cone_root_lane.rs`
`every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed`, the tilted
rod (`r = 0.12`, `h = 0.5`, tilted `0.3` about `z`, centred at
`(−0.75, 0.5, 0)`) across the narrowing frustum. The cone × oblique cylinder
pair is carried and the join builds the topology, but the finish refuses:
`∪` returns `ResultInvalid { NotFinished [VolumeUncomputable { source:
Face { face, source: Unimplemented } }] }`. The row pins every op that
refuses this way (`volume_unread`) and holds every other op to its truth.

## Where

- `topo` `props.rs` `takes_quadrature` keeps a cone face on its closed form
  unless a rim is spiric or a rational spline (`curved_rim`), so a cone face
  trimmed by a marched (polynomial NURBS, or periodic) section curve goes to
  `geom_brep::props::cone_face_closed_form`.
- That closed form refuses the carrier there: `props/curved.rs`
  `require_on_cone` and `cone_edge_length` answer `PropsError::Unimplemented`
  on `Curve3::Nurbs | Periodic`, and `loop_area.rs` `loop_vector_area`
  answers it on `Periodic`.
- The quadrature lane's cone cap (`props/quad_lane.rs` `rim_face`, then
  `spiric::cone_cap_rounds`) reads a cone face's flux about the apex as
  `A·A⃗`, `A⃗ = ½∮(x − A) × dx`. That is exact only for edges that lie on the
  cone, and it charges nothing for an edge that does not. Routing the marched
  edge there would certify a bracket with no pad for the fit's distance
  from the cone, so it is not done here.

## What a fix supplies

A cone face's flux and area with a fitted edge: the vector area of the
edge (polynomial NURBS exactly, periodic by its pieces), padded by the
edge's certified distance from the cone (its record's envelope, as the
cylinder lane pads its pcurves), together with the closed form's nappe
and winding decisions for a curve that is not a conic. Every cone pair the
value marches (cone × cylinder, cone × cone, cone × sphere) leaves such a
face, so every Boolean that builds one of them stops here at its finish.

