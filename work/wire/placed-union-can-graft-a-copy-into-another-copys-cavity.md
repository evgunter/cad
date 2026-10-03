---
id: placed-union-can-graft-a-copy-into-another-copys-cavity
kind: issue
title: Placed union grafts every copy onto the prototype's solids, so a copy inside another copy's cavity lands as an island under one solid (unmeasured)
status: closed
closed: 2026-10-03
opened: 2026-10-02
priority: P3
cost: M
---

Found by the island-filing sweep (FUSE,
`subtract-of-a-hollow-operand-files-the-island-under-one-solid`), not
measured.

`crates/editor-core/src/eval/wire.rs`, the placed union: placement 0
mints the destination solids and every later placement is grafted ONTO
them (`topo::graft_disjoint_all_onto_keyed`), after
`topo::Separation::certify` has shown the copies' padded face boxes
apart. Face boxes apart does not mean outside each other's cavities. A
prototype whose one solid holds a hollow box and, beside it, a small
cube (a disjoint union) can be placed so that copy 2's cube lands
inside copy 1's cavity while every face box stays clear. The fused
body then holds an `Outer` inside a `Void` of the same solid, which the
boolean now files as a solid of its own
(`crates/topo/src/boolean/islands.rs`) and tier 3 admits.

First a row: build that prototype and placement and read the grouping.
If it lands as predicted, the placed union wants the same filing (or a
refusal), and the choice is the owner's.

## Closed (PR 3891)

Retired by the same PR that filed it: `graft_disjoint_all_onto_keyed`
is gone, and the placed union mints one solid per copy
(`crates/editor-core/src/eval/wire.rs`, `wire_placed_union`), so no
copy lands under another copy's solid.
