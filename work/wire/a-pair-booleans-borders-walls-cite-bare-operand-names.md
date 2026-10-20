---
id: a-pair-booleans-borders-walls-cite-bare-operand-names
kind: issue
title: A pair boolean's Borders walls cite the operands' bare face names, so two walls of one name from the two operands spell one set
status: open
opened: 2026-10-20
priority: P2
cost: M
---


## Finding

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean`'s `wall`
closure: a face piece's `Fragment(Borders)` wall is the merged parent's
`Merged` name or `operand_face_name(..).name`, the operand's BARE face
name, and `obstacles.split_corner` reads the same closure for a corner's
copies. Since `wire/pair-seam-name-order` a pair boolean's seam sides
and crossings cite the face as the boolean's own table names it
(`From { read, of }`), so the read says which operand a side came from;
a wall still does not.

Where the two operands hold the same names (a `Transform` copy, any
placement of one prototype), two pieces of one parent bordered one by
`from`'s face `X` and one by `tool`'s face `X` spell one
`Borders([X])` and tie. N3 (`crates/editor-core/src/names/README.md`)
says a `Borders` wall cites its parent and the seam sentence beside it
says a side is named as this boolean's own table names it; whether the
wall takes the same spelling is the question, and it moves every pair
boolean's `Borders` name. N3's "a `Borders` wall cites its parent"
arguably binds it already: a wall's parent, as this boolean's own table
names it, is the `From` the face came in through, as a seam side's is. A union's walls are member-space names
already (the collapse descends the fold's `From`).

Measured: `pair_seam_name_order::a_subtract_of_a_transform_copy_names_each_crossing_apart`
publishes no tied row, so that fixture does not reach a wall
collision. No witness built.

Signed (WIRE lane `wire-seamorder`).
