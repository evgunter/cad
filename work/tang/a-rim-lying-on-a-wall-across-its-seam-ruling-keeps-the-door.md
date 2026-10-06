---
id: a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door
kind: issue
title: A rim lying on a wall across that wall's seam ruling keeps the crossing layer's door
status: open
opened: 2026-10-02
---

## What

`dome_on_the_cap()` lowered into the tube by `dz` (`-1e-3` or `-0.3`),
undeclared, builds: its rim circle lies on the tube's wall, and the
tube's two wall faces are bounded by seam rulings at `±x`, where the
rim's own two vertices are. Turned a twelfth of a turn about the axis
first, it refuses `CurvedPierceUnsupported` in both member orders
(`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`a_dome_sunk_into_the_tube_builds_undeclared`, measured on branch
`tang/abutting-rim`, PR 3823).

## Why

Each rim semicircle now crosses a seam ruling mid-arc, so it passes from
one wall face to the other. Certificate (a) of `reduce::lying_on` finds
that crossing on the circle and correctly declines; certificate (b)
needs a chain of the partner's arcs, and the partner has none in that
plane. Nothing splits the arc at the ruling, so the door stays.

## Direction

A crossing of a lying-on arc with a partner boundary LINE in the arc's
plane is a point the reduction can split at, as it splits a chord at a
pierce: the line's one crossing with the plane, decided on the circle,
is the split point. The two halves then each lie inside one face.
