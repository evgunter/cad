---
id: a-profile-segment-is-an-ellipse-arc
kind: unit
title: A profile segment can be an ellipse arc: stored, checked at validate, lifted to Curve3::Ellipse, every pair with another segment and every sweep refusing typed
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
---



Unit 2 of `an-ovoid-cannot-be-authored` (Ev, PR 7289), part 1's
storage: "an ellipse-arc segment in the profile, its own segment kind,
lifting to the existing `Curve3::Ellipse`, as an arc lifts to
`Circle`". The format clause it extends is `crates/profile/README.md`
"The profile loop: format and tangency" (moved from DESIGN `[topology]`).

## What is true when it is done

- A 2-D ellipse-arc carrier type in `geom-core` beside `Arc2`
  (`crates/geom-core/src/arc.rs`): centre, a unit principal direction,
  the two semi-axes, and a signed sweep in the eccentric anomaly,
  `0 < |Δ| ≤ 2π`. It is shared by the profile and the B-rep sketch
  segment, as the arc carrier is.
- `profile::Segment` (`crates/profile/src/lib.rs`) gains `Ellipse(..)`;
  `SegmentDirections` gains the ellipse's arm (its end tangents);
  `SegmentKind` (`crates/profile/src/validate.rs`) gains `Ellipse { .., turn }`.
- Validation checks the redundant form of a table's ellipse segment as
  it checks an arc's (`seg::build_seg`, `classify`, `check_carrier`,
  `check_directions`): both vertices on the carrier, the sweep agreeing
  with them mod 2π, the recorded directions agreeing; each condition
  checked at validate for a table and never twice. A table ellipse whose
  semi-axes are within the band of each other refuses typed with the
  recourse "write `arc_to`" (the ruling: a factor within the band of 1
  refuses; two kinds of carrier, so the boundary refuses as the
  near-horn spindle's does). A full-turn ellipse is one segment at one
  vertex.
- The junction reads an ellipse leg's turn at its own lever
  (`seg::junction_turn`, `heading_on`, an `arc_lever` analogue for the
  ellipse).
- **Every pair with an ellipse segment refuses typed** in the pair
  pass (`seg::pair_contacts`), naming that the ellipse's pair rows are
  the planar reader's and unbuilt
  (`profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`).
  So a one-segment full-turn ellipse loop validates, and any loop with
  an ellipse and another segment refuses, typed.
- The lift: `geom_brep::mapped::SketchSegment` (`crates/geom-brep/src/mapped.rs`)
  gains the ellipse arm and lifts it to `Curve3::Ellipse` through
  `Curve3::ellipse`'s deciding door (`crates/geom/src/curves.rs`).
- Every consumer of `SegmentKind`/`SketchSegment` handles the arm
  (counted 2026-11-22, about 55 non-test sites in 20 files outside
  `profile`, and about 47 inside it): sweep's `swept.rs`, `extrude.rs`,
  `revolve/axis.rs`, `revolve/partial.rs`, `revolve/tube.rs`, `loft.rs`,
  `loft/stacking.rs`, `skin.rs`; topo's `validate.rs`, `offset_axial.rs`,
  `split.rs`, `replace_face.rs`, `loop_winding.rs`,
  `loop_winding/spline_rows.rs`; geom-brep's `certify.rs`,
  `props/loop_area.rs`; editor-core's `names/emit_sweep.rs`,
  `mate/reach.rs`; the viewer's `sketch.rs`, `pane/create.rs`; pncad's
  `profile.rs`; the tour's `skinned.rs`, `rocker.rs`. **Each sweep
  refuses an ellipse segment typed (row 2)**: the revolve naming
  `the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis`
  (centred) or SPLINEWALL's `the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits`
  (off the axis), extrude, loft and sweep naming
  `extrude-loft-and-sweep-mint-the-wall-of-an-ellipse-or-spline-segment`.
  The viewer draws an ellipse segment's sketch (`viewer/src/sketch.rs`'s
  flatten).
- The format clause in `crates/profile/README.md` says the third
  carrier: "a line, an arc, or an ellipse arc (centre, principal
  frame, two semi-axes and signed sweep in the eccentric anomaly)". This
  is the approved change's re-word and lands with it.

## Check first

1. BOXFINISH's open PR 7384 ("a validated segment holds its directions
   in its kind's arm") reshapes `SegmentKind` and `SegmentDirections`.
   Build on main as it stands when this unit starts; if 7384 is still
   open, coordinate on BOXFINISH's log rather than reshaping twice.
2. The format clause's "the sweep as one `4·atan` of a quantity
   algebraic in the authored data, never an angle difference" must
   hold for the ellipse's stored sweep. For an ellipse drawn by
   `Stretched`, the eccentric anomaly is the un-stretched circle's
   angle, so the circle's `4·atan` carries over. For a TABLE ellipse
   (authored as data), check that the sweep is stored as authored and
   only checked. If the clause cannot hold for one of the two doors,
   **stop and report**: that is a format question for Ev.

## Leaves for later

- The leg: `ellipse-to-draws-the-stretched-circle-from-a-point-tip`.
- The pair rows: `profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`.
- Containment of one loop in another when a loop holds an ellipse
  (`seg::ray_crossings`, `validate::point_in_loop`) refuses typed here
  and is the planar build's ray driver.
- The fillet beside an ellipse is an absent verb (the ruling), so
  nothing in `fillet_select.rs` or `path/arc_fillet.rs` learns the
  ellipse.

## Rows and ground

- Changes no other row. SPLINEWALL's spline segment adds a sibling
  variant to the same enum after this unit
  (`a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it`).
- Crosses LATHE and STRUT (`sweep/src/swept.rs`, `extrude.rs`,
  `revolve/*`), CARVEREST (`swept.rs`), RESTFRONT and ONEDOOR
  (`topo/src/validate.rs`), CHROME, DOORS and VGEOM (`viewer/src/sketch.rs`),
  REFS (`names/emit_sweep.rs`), ONECOPY (`geom-core`'s arc home). Run
  `territory --base origin/main`; announce on PROFILE's log (the
  vocabulary is PROFILE's) and LATHE's.
- Runs beside `a-spheroid-is-a-surface-kind-that-nothing-mints-yet`.
- Review: **single (FULL)**. The consistency conditions are the
  format clause's and the refusal at the band is a margined verdict;
  one full read can check both. Difficulty **L**.
