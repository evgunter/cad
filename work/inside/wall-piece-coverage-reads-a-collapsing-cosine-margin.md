---
id: wall-piece-coverage-reads-a-collapsing-cosine-margin
kind: issue
title: A narrow wall piece's coverage is read on the cosine-window margin, which collapses to the zero band for a hit definitely clear of both its ends
status: closed
opened: 2026-10-10
priority: P2
cost: M
closed: 2026-10-24
---


Found by CLEAVE's `point-in-solid-mints-outside-its-period-and-nappe-rows`
(mint C15), which routed the arm through the nonzero gate. This row is the
cause that unit left in place.

## What

`crates/topo/src/boolean/solid_contain.rs` `point_on_chart_wall` decides
whether a piece covers a hit's ruling, when neither of the piece's end
junctions is active, on `chart_azimuth_margin`: `r·(cos Δ − cos h)`, `Δ`
the hit's angle from the piece's mid-azimuth and `h` its half-width. Near
an end that margin is about `−r·sin h·δ` for a hit `δ` past it, so on a
narrow piece it reads Zero for a hit definitely clear of the end. That is
the audit's ledger row F8 (`docs/predicate-dimension-audit.md`, "window/
cosine family ... own unit").

Measured at ε = 1e-9, 1e-6 and 1e-12 at the default K
(`wall_section_rows::a_narrow_windows_collapsed_margin_keeps_its_reading`):
on windows 0.02, 0.05 and 0.08 rad wide, a hit `2Kε` past or short of the
window's end is definitely clear of that end's junction (its sine offset is
`2Kε`), yet the window margin decides Zero. The arm now refuses there as
`PointInSolidError::NarrowWindowUnread`, carrying the nonzero gate's
tagged rejection (`bool_wall_trim`, margin within ε) and ending on this
kernel limit; before, it minted `INVALID`. Either way a hit the face
definitely holds or definitely misses is refused, and a ray that meets it
is set aside.

## What would close it

Read coverage on the margin the junction loop already decided: the sine
offset of the hit's ruling from each end (`axis·(m̂ × r̂)·r`, signed) is
definite for a non-active end, and its sign says which side of that end the
hit is on. One fact decided once (the mints design's item 7), and no cosine
collapse. The face-level window check in `point_on_wall_in_face` and
`wall_hit` reads the same construction and collapses the same way on a
narrow face; whether those want the linearized margin too is the F8 unit's
question.


## Closed by CHART (2026-10-24)

The code this row is about is deleted. On branch `chart/ringed-curved-doors`
(`curved-face-containment-answers-none-on-a-ringed-face`), cylinder and cone
faces are read from their boundary edges (`topo::boolean::ruled_region`) by
both point doors, with no chart window, outline class or period guard.
