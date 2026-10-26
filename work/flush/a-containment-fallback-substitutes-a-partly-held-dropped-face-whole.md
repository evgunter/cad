---
id: a-containment-fallback-substitutes-a-partly-held-dropped-face-whole
kind: issue
title: A containment fallback records a dropped face only part of which the kept face holds as substituted whole
status: closed
opened: 2026-10-20
priority: P2
cost: M
refs: [5368]
branch: flush/fallback-partial-rows
pr: 5706
closed: 2026-10-26
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

## Measured

On main `1f9b8156`, through the plain booleans (probe:
`topo` `boolean_covered::a_containment_fallback_cuts_a_dropped_face_held_in_part`
and `a_box_sunk_flush_in_a_block_unites_alike_in_either_order`, with
`src` at main):

- `a ∖ lid` and `a ∩ big` are `OperandA` fallbacks, each with one row
  holding the whole of `lid`'s bottom or `big`'s top, and no discard.
  `big ∩ a` takes the section path: the same row as a fragment, and six
  discards, `big`'s top among them bordering `a`'s top outline.
- `block ∪ box` (fallback) and `box ∪ block` (section) agree on the body
  and the row; the section order records `box`'s five other faces as
  discards, the fallback none. `box ∩ block` (fallback) records none
  where `block ∩ box` records six.
- No case of `sweep` `curved_coincident_fates` took the fallback in
  either order, so its exclusion compared nothing. A brick on the
  hemisphere's base, inside it, does: `hemi ∪ brick` and
  `brick ∩ hemi` are `OperandA`.

## Fix

The fallback cuts as the section path does (shape (a)).
`ops::fallback_fates`, called once the fallback's result is built and
gated, keeps the fates of each pair whose kept copy the result holds and
whose dropped copy it drops, cuts them through `fate::cut`, and records
every face of a dropped shell, the cut fragments among them, through
`discard::discard_row`. A dropped face borders no kept face of its own
operand, so its stretches are read through substituted fragments alone
(`discard::ChordsAcross`, whose own-vertex read moved from
`finish::discarded` to `discard::own_in_result`, called by both paths).
`fate::uncut` is gone. The swap comparators compare the fallback's
discards like any other.

The fallback reaches poses that the join shapes on the section path, so
`fate::cut` learned three of them:

- a closed kept edge whose one vertex is a lone vertex of the dropped
  face, or an existing vertex in one corner of it: a circular edge
  (`MefSite::Lone`, or `Chords` with an empty run);
- one that leaves and returns through two corners of the outer loop:
  a chord splitting that loop;
- a sector's piece read on a face that is one piece, which is that face,
  as `fragment_at` already reads it.

A closed chord carries the kept edge's restated description, since a
whole-turn scaffold circle reads as a null edge's.

Each dropped face is cut on its own. Where a face's cut meets one of
the cut's placement limits (`fate::Limit`, which names
each), that face alone is restored and recorded in
`BooleanNaming::uncut` (`UncutFace`), with neither a substituted nor a
discard row. Every other refusal of the cut propagates. The limit is
filed as `a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch`; that
no reader reads `uncut` is filed on BORDERS as
`no-reader-reads-a-fallbacks-uncut-faces`.

## Resolution

Closed by PR 5706 (2026-10-26). The containment fallback cuts each dropped coincident face through `fate::cut`, as the section path does, and records every face of a dropped shell through `discard::discard_row`, so the two operand orders agree on substituted rows and discards (the swap comparators now compare them).
- A face whose cut meets one of the cut's placement limits (`fate::Limit`, a typed refusal) alone is restored and recorded in `BooleanNaming::uncut`, with neither a substituted nor a discard row. Every other refusal of the cut, `JoinDesync`, `Escalated` and an Euler operator's among them, refuses the boolean.
- A ring the cut winds that reads in band refuses as its own decision, `BooleanDecision::CutRingWinding`, with a lever.

One FULL review: NOT-MERGEABLE-AS-IS on the first head (the fallback swallowed every refusal of the cut into `uncut`, and one refusal uncut every partly held face). The fix pass fixed both, which the check pass confirmed by execution (injected refusals propagate; the holed plate and lid uncut only the plate's top). No body moved: batteries identical, corpus BODY lines identical at 3 ε but the two `boss/holedlow ∩` refusal words, suites full at 3 ε.

Residue filed:
- `a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch` (FLUSH), the limits themselves;
- `no-reader-reads-a-fallbacks-uncut-faces` (BORDERS, P2, design).
