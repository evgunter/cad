---
id: axis-coincident-lap-trips-the-planar-join-invariant
kind: issue
title: A box lap whose plane CONTAINS the cylinder axis reaches the all-planar join lane's conic guard through the public subtract door
status: closed
opened: 2026-09-09
priority: P0
cost: H
parent: CONTACT-2
closed: 2026-09-26
---


Found while measuring which box cuts a cylinder accepts (a demo-coverage
survey of log-cabin joinery). Every partial box cut refuses
`CurvedSectorSideUnsupported` — the frontier
`work/bool/slab-cut-cylinder-refuses-sector-side` (#1455) already
carries — **except one pose**, which refuses differently and with an
internal invariant's words.

## Measured

A cylinder r = 0.5 running `z ∈ [0, 4]` (an extruded circle), minus a
box extruded from `z = 3` for `1.5` (so it runs off the far end and has
exactly ONE interior end wall). `Tol::witness()`, `topo::subtract`
through the public door.

| cutter `y` range | cut plane vs the axis | refusal |
|---|---|---|
| `[0.0, 1.0]` | **contains the axis** | `Join(SectionInvariant { face: FaceKey(1v1), what: "the all-planar join lane reached a conic run edge (the operand gate promises every carrier planar)" })` |
| `[0.2, 1.0]` | off the axis | `CurvedSectorSideUnsupported` |
| `[0.35, 1.0]` | off the axis, shallow | `CurvedSectorSideUnsupported` |

The full-length flat (`z` spanning the whole rod) builds in every one of
those `y` poses, so the pose alone is not the problem — it is the pose
together with the interior end wall.

## Why it is a row

The payload is not a frontier's. `crates/topo/src/chord_join.rs:1512`
raises it from the `JoinLane::Planar` arm, and its own sentence says the
lane reached something *the operand gate promises cannot arrive*:

> the all-planar join lane reached a conic run edge (the operand gate
> promises every carrier planar)

So a legal input through a public door reaches a site whose message
asserts the site is unreachable. One of two things is wrong and the
tree does not say which: the gate's promise does not hold for this
pose, or the lane selection picks `Planar` for a body whose run edges
are not. Either way the honest refusal for the shape is the sector-side
one its off-axis siblings get — this pose gets an invariant instead,
and an invariant that fires is not a frontier a consumer can read.

The pose is not exotic: a plane through the axis is exactly how a
half-lap joint is cut, and the `z = 3` wall's section against the
cylinder is a plain circular arc.

## What the taker owes

Decide which half is wrong, then either make the gate's promise true
for this pose or route it to `CurvedSectorSideUnsupported` with its
siblings — and a row on the pose either way, since nothing in tree
reaches this arm today.

Refs `work/bool/slab-cut-cylinder-refuses-sector-side` (#1455).

## Second pose (2026-09-25, `band/ruled-d-hole-ring-crease`)

The same payload from a different operand: the block `[−1, 1]² × [0, 1]`
minus a D-profile rod (chord `x = 0.3`, major arc r = 0.5 about the
origin) extruded for `1.0` from `z = −0.5` — a blind D pocket entering
through the BOTTOM face — refuses `Join(SectionInvariant { what: "the
all-planar join lane reached a conic run edge (the operand gate promises
every carrier planar)" })`. The same pocket from the top refuses
`JoinDesync` instead; both are rowed at
`work/zip/blind-d-pocket-subtract-refuses-with-join-internal-words`.

## Closed

By CONTACT-2 (PR 3250). The gate's promise was the false half: the
Planar lane now measures a conic against its section plane, and no
pose reaches the invariant. What each pose does now is in
`crates/sweep/tests/axis_lap.rs`. The axis lap's remaining refusal is
ZIP's `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`,
and the top-entry D pocket is ZIP's `blind-d-pocket-subtract-refuses-with-join-internal-words`.
