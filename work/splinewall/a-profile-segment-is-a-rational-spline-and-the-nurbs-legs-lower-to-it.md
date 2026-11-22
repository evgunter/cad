---
id: a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it
kind: unit
title: A profile segment can be a rational spline, and the NURBS legs PATHS-DESIGN specifies (VQ7) lower to it
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
blocked_on: [a-profile-segment-is-an-ellipse-arc, profile-validation-reads-its-pairs-through-the-plane-instance]
---



Part 3's profile half (`an-ovoid-cannot-be-authored`, Ev PR 7289):
"the spline leg PATHS-DESIGN already specifies (the profile README's
VQ7)". The legs are ruled: `docs/PATHS-DESIGN.md` §2 Legs, "NURBS
legs" (`nurbs_in_place(len1, ctrl, weights, knots)` on a `Directed` tip
with `P0` and `P1` implied; `nurbs(curve)` placed rigidly;
`nurbs_reversed`, `nurbs_mirrored`; the sharp-seam and tangent-seam
closing forms, round 13), and its §3 table rows. Both doors are
heading-taking. A corner fillet beside a NURBS leg is an absent verb.
What VQ7 records is only that the segment layer has nothing to lower
them to; this unit is that layer and the legs.

## What is true when it is done

- `profile::Segment`, `SegmentKind` and `SegmentDirections` gain the
  rational-spline arm (a 2-D rational spline carrier: clamped,
  `w > 0`, the PR 3 invariants), beside the ellipse arm OVOID's
  `a-profile-segment-is-an-ellipse-arc` added. Vertices stay verbatim
  and authoritative; a spline's end positions and tangents are
  intrinsic, so its end is a directed point.
- The four leg verbs and the two closing forms lower to it
  (`crates/profile/src/path.rs`, `path/program.rs`, `path/verbs.rs`),
  `lift` lifts it back, and the program form carries it everywhere the
  vocabulary is exhaustive (the set `ellipse-to-draws-the-stretched-circle-from-a-point-tip`
  lists: editor-core, the viewer, pncad, pncad-py).
- Validation pairs a spline with anything at certified subdivision,
  read through the planar meeting reader (PR 7290; the reader's table
  routes a spline pair to `Method::Walk`).
- Every sweep refuses a spline segment typed (row 2), naming the unit
  that builds its wall, until
  `the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits` and
  `extrude-loft-and-sweep-mint-the-wall-of-an-ellipse-or-spline-segment`
  land; whichever lands later wires the spline to the wall.
- VQ7 and the legs' "specified here, not built" in PATHS-DESIGN become
  present tense (the approved change's re-word).

## Check first

1. Where the 2-D carrier lives. `geom-core`'s `spline` module holds the
   knot machinery but no curve type; the NURBS curve is `geom`'s
   (`crates/geom/src/curves/nurbs.rs`), which profile reaches only
   through the planar build's new `geom-brep` dependency. Whether the
   carrier is a new `geom-core` type or `geom`'s curve at `z = 0` is a
   home question the format clause's "the arc carrier is one type
   shared by the profile and the B-rep sketch segment" bears on: report
   the options, do not pick.
2. That the planar reader's `Method::Walk` for a spline pair certifies
   a profile's pairs (crossing, touch, clear) at every lane scalar. If
   it does not, the pair rows are that reader's to build, and this unit
   stops.

## Rows and ground

- Changes PROFILE's VQ7 note in `crates/profile/README.md`.
- Crosses PROFILE (the lattice), NURBS (`geom-core/src/spline/*`,
  `geom/src/curves/nurbs.rs`), SELFREF and DOCTAIL (`program.rs`), the
  viewer and Python programs. Announce on PROFILE's log and the away
  channel (persist).
- Review: **dual**: a new carrier kind in a ratified format and four
  ruled verbs. Difficulty **L**.
