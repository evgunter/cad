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

## More of the class, from a sweep by type (the review of PR 7344, 2026-11-22)

The first sweep grepped for the lift by name. A sweep for the READ (a
raw test of an interval for point-ness, exact zero, or a whole number,
on data that came from the body's scalar) finds these beside the three
above. **Read in code, none measured.**

Exact structure asked of a lifted value, where a value exact only as a
form reads as inexact (the shape of `clearance.rs`'s `iso_of`):

- `crates/geom-brep/src/props/quad/rotational.rs` `HarmChan::is_flat`:
  `exact_point(x) == Some(0.0)` on the harmonic coefficients
  `quad_lane::chan` lifted. A channel flat only as a form takes the
  harmonic rule instead of the flat one: wider, not wrong.
- `crates/geom-brep/src/props/rational_rim.rs` `rational_edge_q_split`:
  the carrier interval's ends and the held knots must be point
  brackets (`bracket::exact_point`), else `QuadratureUnsupported`
  ("carrier interval ends are not exact parameters"). A span end that is
  a boxed parameter refuses the face.
- `crates/topo/src/clearance.rs` `iso_bounded_domain`: a free-form
  face's knot domain must be point brackets ("knot domain is not exact
  structure").
- `crates/topo/src/injective.rs` `iso_params`: a harmonic's moving
  channel must have no cosine or sine term, read as
  `exactly(h.pa[moving], 0.0) && exactly(h.pb[moving], 0.0)`; `None`
  otherwise, and its caller takes the wider reading. `Trig::of` in the
  same file is arithmetic on what it is handed and decides nothing.

A whole number read off an enclosure on the value channel, which over
a wide box is no longer isolated (not a re-ask: nothing at the body's
scalar reads it):

- `crates/geom-brep/src/props/quad/rotational.rs` `branch_turns`: the
  turn less the ends' difference must lie within a quarter turn of one
  whole number.
- `crates/geom-brep/src/props/quad/projected_edge.rs` `turns_name`
  (through `latitude_name`): within `1e-12·(1 + |k|)` turns of a whole
  number.
- `crates/geom-brep/src/props/closure.rs` `interval_loop_turns`: each
  junction's gap as an enclosure, decided through
  `require_junction_closed` at the lane's scalar on a margin read off
  that enclosure. This is the reading
  `work/nearpoint/a-cones-chart-junction-gap-is-wide-over-a-box-on-its-axial-position.md`
  measured refusing at the walk's own scalar through the closed form's
  twin.

A lift handed in as a closure, so no lift name appears at the read:

- `crates/geom-brep/src/curve_face/composite.rs` `crossings`
  (`cross: impl Fn(T) -> Interval`): the span's ends crossed and a
  divisor collapsed to its upper end after a raw
  `is_certified && hi > 0` test, an interval-only premise
  (`FaceDegenerate`); no twin at the body's scalar found.
- `crates/geom-brep/src/curve_face/spline_face.rs`: takes the same
  closure; what it decides on the crossed data was not read to the end.

**What this second pattern still cannot find:** a read of exact
structure spelled without `exact_point` / `exactly` / `lo() == hi()`
(a `width()` compared with zero, a `to_bits` compare, a `match` on a
decoration), and every crate outside `geom-brep` and `topo`.
