---
id: the-cone-cap-lane-charges-no-pad-for-a-fitted-edge-off-the-cone
kind: issue
title: The cone cap quadrature lane reads a fitted edge as lying on the cone and charges nothing for its distance from it; a periodic edge has no vector area
status: open
opened: 2026-11-24
priority: P2
cost: M
refs: [the-join-reads-the-cone-pairs-on-the-value, a-cone-against-a-ball-or-a-cone-refuses-at-the-cone-face]
---

Found by the join's cone-pair unit (`the-join-reads-the-cone-pairs-on-the-value`, PR 7735), and filed with the same id by PR 7733, which opened the lane to cone faces a spline bounds. Whichever of the two lands second reconciles the text.

## Measured

`crates/sweep/tests/a_cone_with_a_ball_or_a_cone_in_every_order.rs` (both
fixtures) and `crates/sweep/tests/reach_cone_root_lane.rs`'s tilted rod:
the join builds, and the finish refused `VolumeUncomputable { Face {
source: Unimplemented } }` from the cone's closed form
(`geom_brep::props` `curved.rs` `require_on_cone`, `cone_edge_length`), which
reads lines and conics only. PR 7733 sends a cone face bounded by any
trim that closed form does not read (`topo` `props.rs` `takes_quadrature`)
to the cone cap lane (`props/quad_lane.rs` `rim_face`,
`spiric::cone_cap_rounds`). Before the lane reads it,
`geom_brep::props::require_cone_cap` decides the closed form's gates where
they apply: lines and conics on the cone with their spans decided, and
every edge's slant bounded on one nappe (a spline's by its control hull,
a spiric's by its extent).

## The residue

- **No pad for the fit.** The cap lane reads a cone face's flux about the
  apex as `A·A⃗`, `A⃗ = ½∮(x − A) × dx`, and its area as `|â·A⃗|/sin α`. Both
  hold only for edges that lie on the cone. A marched section curve lies on
  the cone only to within its fit's envelope, and the lane charges nothing
  for that distance, where the NURBS-patch lane pads its pcurves by their
  envelopes (`quad_lane.rs`, `Σ L·e`). `require_cone_cap` decides no
  spline's or spiric's incidence. So the bracket the lane certifies for
  such a face is not proved to hold the face's true flux.
- **A periodic edge.** A cone face trimmed by a `Curve3::Periodic` carrier
  takes the cap lane and refuses `Unimplemented` there: `cap_about` reads it
  through `loop_area.rs` `loop_vector_area`, which refuses `Periodic`.

## What a fix supplies

Pad the cone cap's flux and area by each fitted edge's certified distance
from the cone (its record's envelope) times its length bound and the apex
lever. Read a periodic edge's vector area by its clamped pieces, as
`SegmentQ::of` does.
