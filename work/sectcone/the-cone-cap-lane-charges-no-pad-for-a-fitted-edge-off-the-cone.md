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

Found by the join's cone-pair unit (`the-join-reads-the-cone-pairs-on-the-value`, PR 7735).

## Measured

`crates/sweep/tests/reach_cone_root_lane.rs`
`every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed`, the tilted
rod across the narrowing frustum, on main before PR 7733: the cone ×
oblique cylinder pair is carried and the join builds, and the finish
refuses `∪` with `VolumeUncomputable { Face { source: Unimplemented } }`.
That came from the cone's closed form (`props/curved.rs` `require_on_cone`,
`cone_edge_length`), which reads lines and conics only. PR 7733
(`sectcone/cone-face-refusal`) sends a cone face bounded by a spiric or any
spline to the cone cap quadrature lane (`topo` `props.rs`
`takes_quadrature`, then `props/quad_lane.rs` `rim_face` and
`spiric::cone_cap_rounds`).

## The residue

- **No pad for the fit.** The cap lane reads a cone face's flux about the
  apex as `A·A⃗`, `A⃗ = ½∮(x − A) × dx`, and its area as `|â·A⃗|/sin α`. Both
  hold only for edges that lie on the cone. A marched section curve lies on
  the cone only to within its fit's envelope, and the lane charges nothing
  for that distance, where the cylinder lane pads its pcurves by their
  envelopes (`quad_lane.rs`, the NURBS-patch lane's `Σ L·e`). The
  closed form decided incidence (`require_on_cone`), and the cap lane
  decides none. So the bracket it certifies for such a face is not proved
  to hold the face's true flux.
- **No nappe or winding decision for a spline edge.** The closed form
  refused a spline because it "could cross the apex between its ends"
  (`cone_loops`), and the cap lane's area reading assumes one nappe.
- **A periodic edge.** `takes_quadrature`'s cone arm matches
  `Spiric | Nurbs`, so a cone face trimmed by a `Curve3::Periodic` carrier
  stays on the closed form and refuses `Unimplemented` there, and
  `loop_area.rs` `loop_vector_area` refuses `Periodic` as well.

## What a fix supplies

Pad the cone cap's flux and area by each fitted edge's certified distance
from the cone (its record's envelope) times its length bound and the
apex lever. Decide the nappe of a spline edge from its control hull's
slant. Read a periodic edge's vector area by its pieces.
