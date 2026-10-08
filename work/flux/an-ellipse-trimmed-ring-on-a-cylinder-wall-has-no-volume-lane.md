---
id: an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane
kind: issue
title: A cylinder wall carrying a ring trimmed by ellipse arcs (a tilted bar through a pipe) has no volume lane: RingOnCurvedFace
status: open
opened: 2026-10-02
---


## What

`topo::mass_properties` refuses a cylinder wall that carries a RING
trimmed by an ellipse arc: `RingOnCurvedFace`
(`crates/topo/src/props.rs:1770`). The cylinder's closed form reads
rings (`geom_brep::props::curved_face_loops`, the chart Green form) but
only rims and rulings; the quadrature lane that reads ellipse trims
takes the outer loop only.

Witness (TANG, PR 3851, while scanning the ring lane's closure): the
unit pipe (`crates/sweep/tests/verbs_germarms.rs`'s `pipe`) and the
brick `x ∈ (−1.1, 1.1)`, `y ∈ (0.2, 0.7)`, `z ∈ (−0.15, 0.15)` turned
about the x axis through `(0, 0.45, 0)` by any of 0.3, 0.7, 1.0, 1.3
rad. The ∩ and bar ∖ pipe build and measure; ∪ and pipe ∖ bar build,
pass every tier, and refuse at the volume backstop with
`VolumeUnmeasured { source: RingOnCurvedFace }` (72 of 72 such ops over
the poses tried, thicknesses down to 1 mm, both tilts, both sides of
the axis).

## The shape of a fix

The quadrature lane's Green form over every loop, as the cylinder's
closed form now does: a ring's contribution is its own `∮` with its own
winding. The closed form's closure checks (`props_loop_closed`,
`props_chart_loops_closed`) are the premise to carry over.

## More witnesses (JOIN, PR 4008's fix pass)

Once the join pairs a conic's germs along the conic, two steep-ellipse
batteries reach this lane with a ring on the cylinder wall
(`crates/sweep/tests/pocket_ring_steep_ellipse.rs`):

- `steep_quad_prism_battery`, `k = 0.5`, sites `[200, 230, 300, 120]`,
  `ψ = 0`, against, ∪ in both orders. The prism's footprint on the
  wall's `[180°, 360°]` face is a window wholly inside it, a ring.
  Main built this ∪ AB sound only because the chord order paired the
  side plane's sites back to back (200↔230, 180↔280.4), which merged
  the face's outer loop and the ring into one loop. The gating row
  `steep_ellipse_poses_build_sound_or_refuse_typed` holds the pose as
  "sound or refused".
- `steep_plate_rod_battery` (a plate pierced by a tilted rod): 1 099
  of its 3 240 runs end here, 910 of them on main already. The 54 new
  ones are at θ ∈ {70, 78}°, ∪ in both orders and rod ∖ plate, which
  refused `RingHomingAmbiguous` before the fix.


## More witnesses (JOIN, PR 4038's pinch crossing)

PR 4038 crosses a pinch before the seam zips; on a cylinder's side
wall the crossing is one ring of the wall split in two (`kemr`), and
the result then refuses at the gate
`ResultInvalid { VolumeUncomputable { RingOnCurvedFace } }`. Main
refused the same lines earlier, `Euler(SelfLoopEdge)`.

- PR 4038's review r1 (`r1b_pinch_probes cyl`, on
  `join/pierce-pinch-families-review-r1`): 61 cube ∖ prism lines on a
  cylinder of axis y, r = 5 (Ltop 19, Lbot 10, notch327 32).
- Its review r2 (`r2_pinch_probes cyl`, on
  `join/pierce-pinch-families-review-r2`): 76 cube ∖ prism lines, e.g.
  `Ltop cyl fib33 psi=0.9 off cp S`.
