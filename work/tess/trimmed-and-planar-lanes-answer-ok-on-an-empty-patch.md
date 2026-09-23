---
id: trimmed-and-planar-lanes-answer-ok-on-an-empty-patch
kind: issue
title: mesh: nothing structural stands between a zero-area boundary polygon and Ok(empty patch) in the trimmed and planar lanes
status: open
opened: 2026-09-18
priority: P1
cost: D
---


TESS-1's sweep (discipline §5). The shape swept for: **a tessellation
lane that can return `Ok` with an empty patch for a face with a
non-empty loop.** TESS-1 closed the curved lane's zero-HEIGHT member on
its structural fact (no meridian traversal); the zero-WIDTH member is
`rim-free-loop-on-a-poleless-chart-meshes-as-a-hole`. This row is what
reading the other two lanes found. **Nothing here was executed** — it
is a reading, and its reach is unmeasured.

**`trimmed::tessellate_trimmed`.** One path to `Ok(patch)` (the retry
loop's exit). What stands between a degenerate trim polygon and an
empty emission is `polygon.len() < 3` (`MissingEntity`, "degenerate
trimmed boundary") and nothing else: a polygon of three or more entries
with no area gives the CDT no inner face, `triangles` stays empty,
`worst` stays `0.0 ≤ δ`, and the face answers `Ok`. The identified-edge
census beside it returns early on an empty identified set. Candidate
reach, not constructed: a NURBS face with a collapsed control-net edge
(a spline-stated dome) bounded by ONE closed iso loop — the trimmed
lane's twin of the rim-only cap, whose UV polygon is a segment at one
`v`. The cylinder chart has no such member with a finite face.

**`planar::tessellate_planar`.** One path to `Ok`. An EXACTLY zero
Newell sum makes `chart_frame`'s normal NaN (`Vec3::normalize` of zero
is all-NaN by contract), every projection NaN, and spade's `insert`
refuses — `Triangulation`, typed. A NEAR-zero sum (a sliver loop whose
area is rounding noise) normalizes to a noise direction and the lane
proceeds; whether `classify_faces` then keeps nothing (→ `Ok` empty) or
keeps slivers was not determinable by reading. Such a face is not
tier-3 valid as far as this lane knows, which was also not checked.

**What a fix has to respect.** TESS-1's spec ruled out deciding a
refusal on a triangle count or an area — those are consequences read
from floats. A lane-agnostic "zero triangles ⇒ refuse" is that. The
cross-face census (`tessellate::unpaired_chord_segment`) already catches
every member in builds with debug assertions, as a panic; the question
for the unit that takes this is whether each lane has a structural fact
of its own to refuse on, as the curved lane had.

What the reading could not see: spade's behaviour on collinear and
near-collinear constraint input; `planar::classify_faces` on a
self-touching outer loop; whether any `step-import` normalization leaves
a zero-area trim loop; NURBS faces with degenerate edges generally.

## Counter-reading of the trimmed candidate (TESS-1 fix pass, 2026-09-20)

A reviewer's reading, which this lane re-read and agrees with; **still
unexecuted**. The trimmed candidate above most plausibly does NOT reach
`Ok(empty)`. `trimmed::tessellate_trimmed`'s constraint pass classifies
every intermediate vertex of a realised constraint, and an
intermediate BOUNDARY vertex refuses `SelfTouchingTrimLoop`. A
zero-area UV polygon is collinear, so the constraint that closes it
runs back over every other boundary point of the loop: `realised.len()
> 1` with boundary intermediates, and the face refuses typed before any
triangle is kept. The message would be wrong for the cause (the loop
does not touch itself; it has no area), but it is a typed refusal and
not a hole.

That arm is also the difference between the lanes: `curved`'s
constraint pass (`exists_constraint` / `can_add_constraint` /
`add_constraint`) has no intermediate-vertex classification, which is
why a zero-height or zero-width curved polygon falls through to an
empty emission where the trimmed one would not. `planar`'s pass counts
crossings and has no such refusal either; its row above stands as
written.

So what this row still owes is an EXECUTION of the trimmed candidate —
if it refuses `SelfTouchingTrimLoop` as read, the trimmed half closes
with a note about the message, and the row narrows to `planar`.
