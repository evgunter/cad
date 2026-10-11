---
id: an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap
kind: issue
title: A triangle prism's apex edge in the partner's face plane builds and fails tier 3' on one EdgeFaceOverlap the boolean neither refuses nor records
status: dispatched
opened: 2026-10-11
priority: P0
cost: M
refs: [a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts, 4650]
branch: flush/apex-edge-in-face-plane
---


Split out of `a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts`
(its "Also in `join1_delta_arc_battery`" section), whose corner-pair
runs PR 4650 fixes and these it does not.

## What

`join1_delta_probes.rs` `join1_delta_arc_battery`, the poses
`ARC tri {half, quarter, shallow} d=(0.0, 0.5) zb=(0.5, 1.5) decl={false, true} U`:
six unions build at the volume with tier 2, the certificate and a legal
operand, and fail tier 3′. The row they came from read one
`UndeclaredContact { EdgeFaceOverlap }` at `(0.0, 0.5, 1.0)`: the
triangle's apex edge `x = 0, y = 0.5` lies in the partner's face plane
`y = 0.5`. (That row named `tri tri` where this measurement fails
`tri quarter`; `tri tri` is `SOUND` here.)

## Measured (2026-10-11)

The same six lines fail tier 3′ on main (merge base `8c8f3b4c`), on
PR 4650 (`flush/corner-ruling-tier3`, which lands vertex-on-face records
on the piece of a divided face that holds the vertex), and on PR 4604's
head `29f98d7a` (CONTACTHOLD, which records every non-incident cell
pair). None of the three moves them, so this is not a record carried
to the wrong piece. Read, not measured: one `EdgeFaceOverlap` and no
`VertexOnFace` suggests the apex edge's ends are not in the face's
interior, so no rest is minted at all, and nothing records the edge
lying in the face.

## Owed

Read one pose's reduction contacts: whether any decision places the
apex edge on the face, and where the overlap's bounds lie. Then decide
whether the op owes an edge-face record (4604's currency) or a
vertex-on-edge record at the bounds.
