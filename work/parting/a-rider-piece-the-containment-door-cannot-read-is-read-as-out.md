---
id: a-rider-piece-the-containment-door-cannot-read-is-read-as-out
kind: issue
title: The parting reads a piece of an edge the containment door cannot place as not lying in the face
status: closed
opened: 2026-11-01
priority: P2
cost: E
branch: parting/rider-piece-unread-refuses
closed: 2026-11-01
pr: 6255
---


Found by the PARTING lane on `parting/closed-and-torus-riders`, in
its sweep for silent `false` readers beside `carrier_on_surface`.
Unmeasured: no fixture is known to reach it.

## What

`topo::boolean::slit::holding` reads whether face `f` holds each piece
of an edge at the piece's midpoint (`ops::piece_reading`, the
containment door). `PieceRead::Unread(_)`, a midpoint the door cannot
place (`Trim(None)` on a curved face, a loop it cannot read), is read
as `NotIn`: where every piece is unread, the edge is read as lying in
no face and keeps one edge, though its carrier lies on `f`'s surface
and both its ends lie on `f`. The same reading is in the spline-face
arm `holding` gained for `SplineFace`.

## Fix

Refuse typed where a piece the carrier puts on `f`'s surface is
unread and something other than a crossing names the pair (`ends` or
`record`), as an in-band reading refuses; a crossing-only pair stays
unasked.

## Closed

2026-11-01, PR 6255 (orchestrator's read: the fix the row writes). An unread piece refuses `EdgeInFace { gap: Unread(_) }` carrying the door's refusal, except where only a crossing names the pair; the spline arm refuses likewise. Pinned by a unit row on a lone-vertex ring; no public-API fixture reaches it, and the spline arm stays on `the-spline-rider-arm-is-pinned-by-no-fixture`. (PARTING orchestrator)
