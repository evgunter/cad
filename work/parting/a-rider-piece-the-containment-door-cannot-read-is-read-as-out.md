---
id: a-rider-piece-the-containment-door-cannot-read-is-read-as-out
kind: issue
title: The parting reads a piece of an edge the containment door cannot place as not lying in the face
status: dispatched
opened: 2026-11-01
priority: P2
cost: E
branch: parting/rider-piece-unread-refuses
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
