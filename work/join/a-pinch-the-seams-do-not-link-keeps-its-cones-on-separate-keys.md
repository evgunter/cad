---
id: a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys
kind: issue
title: A pinch whose keys no seam ties keeps its cones on separate point keys
status: parked
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
refs: [the-cone-vertices-at-a-pinch-share-the-pierce-points-key]
opened: 2026-10-07
---

Found by PR 4207's review (the FULL lane, M2). PR 4207 puts a pinch's
cone vertices on one point key where the seam correspondence ties their
keys (`zip::point_classes`, `zip::share_points`). This row is the rest:
pinches whose keys no seam record ties.

## What

- **`touch`: a point-touch union ships its pinch on two keys.** Two
  corners touching only at `v` union to one body pinched at `v`. No
  section curve runs through a touch at a point, so no null-pair record
  names the two operands' vertices there, and each keeps its own
  operand's point key. The result is `SOUND` only through the
  `VertexVertex` contact the union returns. Reused as an operand
  without that contact, the result is `dbl`'s shape. That is why
  `dbl`'s operands arrive with two keys.
- **`dbl3`: an operand pinched on three keys stays refused.** Three
  corners joined at `v` by two point-touch unions, then a cube whose
  face holds `v`. Of 72 lines, 40 stay `BAD` on tier 3′
  `UndeclaredContact { VertexVertex }` at `v`, byte-identical to main;
  `share_points` does not reach them. The likely cause, not traced line
  by line: the cube's seams pair only some of the cones' vertices at
  `v`, so a key whose vertex no seam reaches stays outside the class.
  Of the rest, 24 refuse `JoinDesync` and 8 are `SOUND` with one cone.

## Why it waits on D10

Nothing in the records ties these keys. Only two things could:
- a **declared contact**: the touch union's returned `VertexVertex`
  contact, carried with the body as intent;
- a **coincidence read**: comparing the vertices' positions.

The first is D10's question: one way to say intent. The second is
D10's undeclared-coincidence ground. So linking these keys waits on
`d10-one-way-to-say-intent-is-unbuilt`.

## Measured

PR 4207's review probes (branch
`join/pinch-cones-share-a-point-key-review`,
`REVIEW-4207-probes/r1_4139_probes_r4207.rs`, sets `touch` and `dbl3`
with the `R4_KEYS` column), release, main `3e9d1a96` vs PR 4207's head:
- `touch`: 84 lines, byte-identical; 28 of them (both orders) ship
  the pinch on two keys.
- `dbl3`: 72 lines, byte-identical, as above.


## Measured (branch `join/pinch-cones-split-at-insertion`)

`join_pierce_runs_sweep::pinch_runs_battery`'s pinched operand is a
point-touch union of two cubes, and the class reaches it. Read by
`pierce_point_finding` at the notch's corner, release, on PR 4207's head
`fb8c7cbb` with this branch's change: 468 `SOUND` bodies hold their
two vertices at `v` on two point keys. That is `ab U`, `ba U` and
`ba S`, 156 each. None of them is among the 217 lines that branch
builds. Those 217 share one key.
