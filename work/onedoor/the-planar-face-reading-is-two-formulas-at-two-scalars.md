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

## A needle channel's `f64` shell volume reads 8% short (GREEN, 2026-11-14)

Added by GREEN's default-eps lane (`green/default-eps`) from nightly
43371638371 (main `e36675c1`), default ε row. This row is the fix row
for the pin below.

`topo::all shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
(slow set, `crates/topo/tests/shell_role_reader.rs`) was red at ε 1e-9
on main `b363f5fa`. With the volume assertion turned into a print, four
of the five runs of the `Pose::Moved` channel 10.5ε across (upright
fill face) miss the 5% the row holds, and nothing else in the test
does:

| run | `w²` | `classify_shells` enclosure volume | off |
|---|---|---|---|
| `brick ∖ slab ∪ fill` (the void) | −1.1025e-16 | −1.0141836835207821e-16 | −8.0% |
| `slab ∖ fill` | 1.1025e-16 | 1.0080466258290497e-16 | −8.6% |
| `slab ∩ (brick ∖ fill)` | 1.1025e-16 | 1.0080466258290497e-16 | −8.6% |
| `(brick ∖ fill) ∩ slab` | 1.1025e-16 | 1.0081101210732788e-16 | −8.6% |

`fill ∪ brick ∖ slab` reads within 5%. Every run's corners are the
channel's to `w/1000` (`assert_corners` passes), so the bodies are
built right and the number read off them is what moved. The absolute
error is about 9.5e-18 m³ on operands a metre long.

**Bisect** (first-parent, by running the test, `a46ac5820` good, the
merge of PR 6812 that last edited the row): first bad `9e1484043`, the
merge of PR 6505 (`onedoor/one-surface-record`), with its parent
`e996c0049` green. That PR moved the `f64` plane term to the fan about
the face's anchor (`[geom-brep.face-anchored.plane]`), the formula this
row is about. Which face's term carries the error was not measured.

**The pin.** `shell_role_reader.rs`'s `MOVED_THIN_READS` holds those
four runs to the values above within 1%, so it turns red when the read
moves, toward `w²` or away. The wanted reading is `w²` within the 5%
the other rows hold; the pin goes when it is.

Reproduce (red before the pin):

    cargo nextest run -p topo --test all -E 'test(=shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void)'

`work/restfront/the-lane-free-sign-guard-has-no-witness-of-a-misrounded-sum.md`
wants a body whose face-anchored `f64` sum rounds past its volume. This
one rounds by 8.6% of it, not past it; it may be a lead for that row.
