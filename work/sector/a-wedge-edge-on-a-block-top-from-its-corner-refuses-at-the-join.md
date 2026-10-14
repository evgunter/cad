---
id: a-wedge-edge-on-a-block-top-from-its-corner-refuses-at-the-join
kind: issue
title: A tilted wedge whose short edge lies on a block's top from the block's corner refuses at the join, with every reading definite
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Filed by CONTACT-9. Found at base as well as at head.

The pose: a block `[0,20]²×[-20,0]` and a parallelepiped on its corner
`(0,0,0)` with edges `a = (10, 1, -d)`, `b = (0.2, 1, 0)` and
`c = (0, 0, 1)`. Edge `b` lies on the block's top, from the corner
into the face, and `a` dips `d` into the block. `intersect` refuses
with `Join(UnpairedLooseEnds { count: 2 })` at `d = 500·ε` and at
`d = 5·10⁴·ε`. With 1 m edges every side reading is definite, so this
is not the levered-reading class CONTACT-9 fixed. The same wedge set
on the block's top away from the corner (`(5, 5, 0)`) answers
correctly.

So the defect is at the vertex pair, where the corner's three faces
meet an edge lying on one of them: the edge-sector event in
`recl_edges`, or the germ pairing it hands the join. It has not been
traced further.
