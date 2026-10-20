---
id: a-joined-set-over-two-tied-operand-edges-loses-its-operand
kind: issue
title: A pair boolean's joined edge over two operand edges one tie names takes a bare From name with no operand, and its vertex refuses
status: open
opened: 2026-10-20
---



Filed by `wire-emis-topo` (the `emit_topo.rs` third of
`names-emission-raises-panic-or-take-their-true-ending`).

## What

`name_boolean_edges` names a joined edge that lies along several operand
edges by `set_name`, which hands each member's `From`-wrapped name to
`join_names::joined_name`. That dedups the names, and one name left is
returned bare. Where an operand's table ties two of the members (two
placements of one prototype), the set collapses to one `From` name, but
its `EdgeGroup` keeps `root: None, side: None`, as a set's does.
`name_boolean_vertices` then reads a `From`-headed edge as a carried
one, finds no operand, and refuses `Emission("a carried edge's group
names no operand")` (`crates/editor-core/src/names/emit_topo.rs`). The
body is sound and the input legal, so the refusal is a defect.
Unmeasured: no test builds the shape.

## Next

Build the witness (a joined edge over two tied edges of one operand),
then decide whether a set whose names tie is a set (`Merged` of one
name twice) or the one tied edge it spells, and give the group the
fields that reading needs.
