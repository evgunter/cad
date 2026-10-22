---
id: a-boolean-edge-lying-in-a-curved-face-is-not-read
kind: issue
title: The Boolean's parting reads edges lying in plane faces only; an edge lying in a curved face keeps the rider form
status: open
opened: 2026-10-19
priority: P1
cost: H
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

`topo::boolean::slit::riders` reads an edge as lying in a face only
where the face is a plane (`lies_in`): a line edge whose midpoint the
plane face holds, or a conic in the face's plane. A keel resting along
a ruling of a cylinder floor, or an arc resting in a sphere or cylinder
face, is not read, so the Boolean keeps the rider form there: one edge
claiming the face's empty side, which tiers 3 and 3′ pass. Unmeasured:
no fixture of the class is built yet.

## Fix

Read a line edge along a ruling of a cylinder or cone face, and a conic
on a quadric face, as lying in it (the reduction's `LyingIn::Curved`
already decides the second at the operand), then part it as the plane
case does: the wall-side reading is first order on the face's normal at
the edge's midpoint, so it carries over.
