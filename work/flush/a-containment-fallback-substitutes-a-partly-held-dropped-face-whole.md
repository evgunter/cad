---
id: a-containment-fallback-substitutes-a-partly-held-dropped-face-whole
kind: issue
title: A containment fallback records a dropped face only part of which the kept face holds as substituted whole
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [5368]
---


## What

The section path cuts a dropped coincident face where its fate changes
(`crates/topo/src/boolean/fate.rs`, `cut`), so each
`BooleanNaming::substituted` row there is a fragment held whole. The
containment fallback cuts nothing (`fate::uncut`, written from
`ops.rs`'s `finish_fallback`): it records each coincident pair's dropped
face whole, oriented by which face its result keeps.

Where the dropped face is larger than the kept face holding it, the
row overstates: part of the face is substituted and the rest is simply
removed, two fates in one row. Witnesses, both `OperandA` fallbacks
(probed on PR 5368):

- `a ∖ lid`, `lid` = x −1..3, y −1..2, z 1..2 standing on `a` = x 0..2,
  y 0..1, z 0..1: the row says `lid`'s whole bottom is held by `a`'s
  top.
- `a ∩ big`, `big` = x −1..3, y −1..2, z −1..1: the row says `big`'s
  whole top is held by `a`'s top.

A naming reader of such a row would read the unheld part of the face as
held. Neither operand's dropped face is in the result, so no geometry
is wrong; the record is.

## Shape of a fix

Either cut the dropped face in the fallback too (the fallback keeps or
drops whole shells, so its dropped clone is free to divide), or record
the row as partial with the kept face's boundary, or write no row for a
dropped face that is not wholly held and record why. Pin both witnesses
beside `boolean_covered::a_containment_fallback_writes_each_row_from_the_face_its_result_holds`.

## A second order difference: the fallback records no discards

The section path records each discarded fragment with the kept faces it
borders (`BooleanNaming::discards`); the containment fallback records
none (`crates/topo/src/boolean/discard.rs`'s module docs). Where one
operand order takes the fallback and the other the section path (a box
sunk flush in a block, `block ∪ box` against `box ∪ block`; a hemisphere
and a small brick on its base, `hemi ∪ brick` against `brick ∪ hemi`),
the two results agree on the body and the substituted rows but not on
the discards: one order has none. Both swap comparators exclude this
explicitly, asserting the fallback side records no discard rather than
comparing (`topo` `boolean_covered::assert_alike`, `sweep`
`curved_coincident_fates::assert_alike`). A fix here would record the
fallback's discards too, or say why a whole-shell path owes none.
