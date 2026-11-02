---
id: a-clearance-seat-on-an-undrawn-body-certifies-and-draws-a-leg-to-nothing
kind: issue
title: A clearance body seat on a hidden or consumed body certifies and draws its witness leg to an unlit face, and the row does not say so
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the correctness review of PR 6357 (m3).

A clearance body seat on a body the picture does not show — a copy the
View pane hides, or a construction a later operation consumed — is
certified as written, and its witness leg is drawn to a face that
lights nothing (`viewer::marks::witness` lights a face only through
`drawn_patch`). The verdict is about the author's own pick, so nothing
is wrong in it, but the row does not say that the seat is not in the
picture, and the leg ends in empty space.

## Repair shape

The row says the seat is not drawn (and why: hidden, or consumed by
the node that took it), and the witness draws its leg only to a face
the picture draws — or says which end it could not place.
