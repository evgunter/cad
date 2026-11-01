---
id: a-closed-edge-lying-in-a-face-is-never-read-by-the-parting
kind: issue
title: The Boolean's parting skips every closed edge, so a full circle lying in a face keeps the rider form
status: open
opened: 2026-10-26
priority: P1
cost: E
---



Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`.
P1: D1-wrong topology shipped silently, the class of the two rows that
branch closes.

## What

`topo::boolean::slit::riders` skips an edge whose two ends are one
vertex (`if a == b { continue; }`), so a closed edge, a full circle a
full revolve or a one-segment rim mints, is never asked of any face,
plane or curved. Where such a circle lies in a face with both its faces
going into the face's material, the Boolean ships one edge there, and
tiers 3 and 3′ pass it.

Witness (the review of PR 5793): the block `[−2, 2] × [0, 1] × [−2, 2]`
less the full revolve about `y` of the quadrilateral
`(.5, .5), (1, .5), (1, 1), (.6, .6)`. Its top vertex `(1, 1)` sweeps a
closed circle of radius 1 lying in the block's top face `y = 1`, the
revolve's two faces at it both leaving it into the block's material.
`A ∖ B` builds F 9, E 17, V 11, two shells, with one closed rim edge
lying in the plane top face, on main and on this branch, at every ε.
The open-arc forms of the class (a rim split into arcs) refuse `Curved`
(`sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`), so the closed
form alone ships.

## Fix

Ask a closed edge of every face as an open one is asked (its ends are
one vertex, so `both` is the faces that vertex lies on, and the carrier
and the records decide the rest); a circle then refuses `Curved` where
its walls go into the face's material, as the open arcs do, and the
witness above is pinned refusing.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)
