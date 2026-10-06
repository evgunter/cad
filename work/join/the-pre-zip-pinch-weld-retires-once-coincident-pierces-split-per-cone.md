---
id: the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone
kind: issue
title: The pre-zip pinch weld (finish::weld_pinches) stays as the repair of an operand's coincident pierces; it retires once those split per cone (D10 ground)
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [a-pinch-no-kept-face-can-cross-refuses, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face]
---


## What

Found building `a-pinch-no-kept-face-can-cross-refuses` (branch
`join/pinch-one-vertex-per-cone-build`). The ruling (Ev, PR 4057) retires
the pinch welds. The post-zip pierce weld (`weld_pierce_copies`) is
retired there. The pre-zip one, `finish::weld_pinches`, cannot go yet.

It fires where two coincident edges of one operand pierce the other's
face at one point. That operand carries a contact of its own: a pinch
line where two of its lumps touch. Each pierce mints its own ring vertex
in the pierced face, so before any zip that face holds two vertices on
one point whose corners overlap: the operand's own fragment crosses
there. `weld_pinches` repairs it by fusing the two into one vertex, its
chord dividing a bow-tie into two faces. That vertex then goes through
`zip::split_cones` with the rest. The split can separate a vertex per
cone, but it cannot uncross two.

Measured with the weld removed, on that branch:
- **Declared ground (D10).** `topo::all union_flush_onto_edge_contact`'s
  three rows go from building to tier-3′ `UndeclaredContact`
  (`VertexVertex` and `EdgeEdgeOverlap` between the two pierce copies).
  The copies sit on two point keys, and the contact records reach them
  only through the weld rows (`ops::Descendants`).
- **Undeclared.** `editor-core::all union_pinch_member_order`'s four
  rows build, then the mesher refuses `PinchWedge`: one face's corners
  cross at the pinch.

## The shape to give

Mint an operand's coincident pierces on one point key with their corners
already per cone (each pierce's ring vertex holding the corners of its
own cone), so no repair weld is needed. Then retire `weld_pinches`,
`pinch_site` and `Joint::Hole`. The declared rows move only when D10
lifts, and their contact records then have to name both copies.
