---
id: the-planar-face-reading-is-two-formulas-at-two-scalars
kind: issue
title: a plane face is read by the fan at f64 and by plane distance plus a tilt slack at Interval: two formulas for one flux
status: open
opened: 2026-11-11
priority: P2
---


Filed by the onedoor fix-3 lane on PR 6505 (rv3 review n5, S10).

## What

`[geom-brep.face-anchored.plane]` reads one flux at two scalars, but by
two formulas. `geom_brep::props::planar_face` (f64) takes the fan
`at·A⃗` about the face's anchor (`props::face_anchor`).
`topo`'s `props::quad_lane::planar_face_about` (Interval) takes the
plane-distance form `plane_flux` — the anchor's distance from the
centre along the stored normal times `A⃗·n̂` — plus `tilt_slack`, a
symmetric bound on the part across the normal. The interval encloses
the fan, so the clause holds as an enclosure, but the two lanes do not
evaluate one expression.

## Evidence

- The fix-2 lane tried the f64 lane on the plane-distance form, as its
  brief asked. R1's slotted plate (`crates/topo/tests/a_verdict_moves_only_the_faces_it_read.rs`,
  A∪P) then reads its volume 33ε off the exact value: a face moved onto
  a record in band of its own stands off that record, and the stored
  normal's distance reads the record, not the face's own boundary.
- `quad_lane::planar_face_about`'s doc rejects the fan at Interval for
  conditioning: `(anchor − centre)·A⃗` rounds every component of `A⃗`
  at the lever `|anchor − centre|`, which swamps a thin body's volume
  (`work/tally/the-corner-fan-volume-of-an-oblique-needle-cancels-the-bodys-extent-cubed.md`).

## What one formula would need

A form both scalars can take that (a) reads the face's own boundary,
never the record's normal or origin, so a face standing off its record
reads its own fan (what the f64 lane needs), and (b) keeps the width of
a long thin face's term at the face's own flux, not its lever times its
area (what the interval lane needs). One candidate: the fan's normal
part read along the loops' own unit normal `A⃗/|A⃗|` in place of the
stored one, with `tilt_slack` levered by the in-plane offset only; it
needs a certified normalisation at Interval and a check that its f64
value keeps the exact rows (`crates/topo/tests/review_m3_pr55.rs`).
