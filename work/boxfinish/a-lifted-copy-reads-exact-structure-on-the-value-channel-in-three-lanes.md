---
id: a-lifted-copy-reads-exact-structure-on-the-value-channel-in-three-lanes
kind: issue
title: Three interval lanes read exact structure off a lifted copy's value channel: a clearance row's held axis, a trim rectangle's held value and way, and a spiric or spline cycle's winding
status: open
opened: 2026-11-22
---


Found by the volume-premises lane's sweep (branch
`boxfinish/volume-premises`, 2026-11-22) for the shape of
`the-volume-readings-interval-re-derivation-re-asks-a-closed-forms-premises-on-the-value-channel`:
a decision taken at the body's scalar and taken again, or only, on the
value channel of a copy lifted through `Interval::from_certified`.
**Read in code, none measured**: no fixture reaches these over a box
today, since no door builds a curved body over one.

- **`crates/topo/src/clearance.rs` `iso_of`, `monotone_line`,
  `wall_of`.** Each reads which chart axis an image holds, or whether a
  channel is constant, as `injective::exactly(lift(x), 0.0)` (and `±1`
  for a spiric's sense): a point zero on the value channel, through no
  funnel. The same structural zero is read at the body's scalar by
  `geom_brep::Pcurve::held_line` (`geom_core::is_exact_zero`) and by the
  mint that chose the variant. A zero that is exact only as a form reads
  as not held: `Row::Bulging`, a wider row, or with
  `injective::whole_turn` no enclosure at all ("a face whose boundary
  carries no sound enclosure"). The nearest to this unit's shape: a
  re-ask that can only lose.
- **`crates/topo/src/props/quad_lane.rs` `trim_rectangle`'s
  `exact_value` and `way_along`.** Each held line's value and rate are
  lifted and asked "is a certified point" and "lies wholly one side of
  zero", raw. The axis and the winding were read at the body's scalar
  (`pcurves::Lift::runs`, `lift.winding()`); point-ness and the rate's
  sign are asked only here. Over a box a held value exact as a form
  refuses "held value is not exact structure". Not a literal re-ask.
- **`crates/topo/src/loop_winding.rs` `enclosed_winding_with`.** A cycle
  with a spiric or spline edge has its winding
  (`planar_winding_enclosure_refine`, `bool_ring_run_winding`) decided
  on the lifted copy alone; `winding_of_steps` routes to it instead of
  the sum at the body's scalar, so nothing is asked twice, and nothing
  is asked of the form either.

What a fix has to settle for each is whether the reading may be taken
at the body's scalar and carried (as the volume reading's curved faces
now are), which the first admits most plainly.

**Not this shape** (the same sweep): `quad_lane::planar_face_about` and
`polygon_face_about` decide nothing; `cut_face_rounds`' decisions exist
only in the lane; `cell_region`'s gap decisions have no twin at the
body's scalar and keep the cell where undecided.
