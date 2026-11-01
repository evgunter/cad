---
id: cylinder-chart-trim-reads-a-walk-it-cannot-take-as-a-corrupt-face
kind: issue
title: cylinder_chart_trim answers CorruptFace for a window walk it cannot take, on a valid face
status: closed
opened: 2026-10-12
priority: P3
cost: M
refs: [cone-trim-reads-an-in-band-image-walk-as-a-corrupt-face]
closed: 2026-10-24
---


Found by PR 4875's review (MINOR-3), by reading.

`crates/topo/src/boolean/solid_contain.rs` `cylinder_chart_trim` reads
the face's azimuth window through `chord_join::face_azimuth_window`.
After 4875 an escalation of that walk stays `Escalated`. Every other
refusal still answers `PointInSolidError::CorruptFace`: a run edge with
no closed-form image, a fitted image, a vertex off the carrier, or a
joint whose images name no common branch (`SectionInvariant`). Those are
statements about the face's class, not corruption. On a valid body,
"one of its faces is broken" is the wrong message.

It is not a local fix, for two reasons:
- `PointInSolidError` has no cylinder class variant beside
  `PartialSphereFace` / `PartialConeFace` / `PartialTorusFace`.
- `boolean::contain`'s cylinder placement reads `CorruptFace` from this
  function as the honest remainder (`CurvedPlacement::Trim(None)`), so a
  new variant moves that reader too.

What would close it: a typed class answer for a cylinder window the walk
cannot take, read by `contain` as its remainder, with `CorruptFace` kept
for `Corrupt` / `UnpairedLooseEnds` / a non-cycle (the distinction
`cone_trimmed_window` and the windowed-face reader draw).


## Closed by CHART (2026-10-24)

The code this row is about is deleted. On branch `chart/ringed-curved-doors`
(`curved-face-containment-answers-none-on-a-ringed-face`), cylinder and cone
faces are read from their boundary edges (`topo::boolean::ruled_region`) by
both point doors, with no chart window, outline class or period guard.
