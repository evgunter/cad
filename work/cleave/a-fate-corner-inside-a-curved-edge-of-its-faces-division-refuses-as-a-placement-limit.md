---
id: a-fate-corner-inside-a-curved-edge-of-its-faces-division-refuses-as-a-placement-limit
kind: issue
title: A fate cut's pierced corner inside a curved edge of its kept face's division refuses as a placement limit; the side reading takes lines only
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
---


Found by the fix of
`a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece`
(branch `cleave/fate-cut-end-on-a-division-edge`). Unmeasured: no
witness yet.

## What

`fate::piece_beside` reads which piece of a kept face holds a dropped
corner whose vertex lies inside an edge of the face's division: the
edge's half-edge in each piece, read as a straight corner at the point
(`corner_holds`), its direction the edge's chord. That holds for a line
only. On any other edge (a circle, an ellipse, a spline) the tangent at
the point is not the chord, and `piece_beside` refuses
`fate::Limit::BesideNoPiece` before reading.

A pose that should reach it: a lump whose base circle on the flush
plane passes through another lump's corner at no vertex of the circle
(a cylinder standing beside the cube `[0, 1]³` with its base circle
through `(1, 1, 1)`), and a partner flush with both, in the op that
keeps the partner's face over the corner.

## Owed

A witness at all three ε. Then read the edge's tangent at the point
(its parameter there, `Curve3::deriv`, and the reach to each end,
`geom_brep::edge_reach_bounds` from that parameter) and pass it to the
same straight-corner reading, or show the pose cannot arise.
