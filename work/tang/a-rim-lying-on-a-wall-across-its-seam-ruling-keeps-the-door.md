---
id: a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door
kind: issue
title: A rim lying on a wall across that wall's seam ruling keeps the crossing layer's door
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-06
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

## Review tier

SINGLE, FULL: the lying-on reduction's split decides which faces a rim
arc belongs to; one full review.

## Closed (2026-10-06, TANG)

`reduce::lying_on` asks a third question when certificates (a) and (b)
both decline: where the arc meets the face's boundary strictly inside
its span (`carrier_cross::boundary_crossing`, the declared arms'
interior question). A meeting is a `Pierce` that the sweep splits both
edges at, and each half is read again. A certified absence is
certificate (a)'s conclusion. The meetings come from every boundary
vertex and the closed-form meeting with every line or circle boundary
edge, so the split is not specific to seam rulings. Certificate (a)
was where the defect started: it reads the whole circle, which any
ruling of the wall meets.

The turned sunk dome builds every op in both member orders, undeclared,
at its closed form, tiers 3 and 3′. The rows cover both sink depths,
the two-face tube at 1/12, 1/8 and 1/5 of a turn, and a four-face
tube. On the four-face tube each semicircle crosses two rulings at
those turns, and at 0 and 1/4 its vertices sit on rulings
(`pi_seam_and_kiss_through_the_boolean.rs`,
`a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared`).

`t ∖ d` exposed a census gap. A v-f record on a curved face (the dome's
rim vertex inside a wall face) read stale at tier 3′, because the
census confirmed v-f records against planar faces only.
`census::confirm_vertex_on_face` now confirms it through the curved
containment door.

Filed: `a-line-edge-lying-on-a-wall-keeps-the-door` (P1, the same
lane for a ruling). Evidence added to
`a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings`.
