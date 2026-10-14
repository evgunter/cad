---
id: a-carried-vertex-on-face-row-at-a-pinch-is-unprobed
kind: issue
title: No fixture carries a vertex-on-face row whose vertex sits at a pinch, where the remap's strict rule drops a fused vertex's rest
status: open
opened: 2026-10-03
priority: P3
cost: M
---



## What

`remap_carried` (`crates/topo/src/boolean/ops.rs`) carries an operand's
own v-v rows through the same chase as the reduction's rows. Its
vertex-on-face rows (`carried_a.vf` / `carried_b.vf`) instead go
through `vert_strict`, which drops a row whose vertex took part in a
zip fusion. That is the rest-consumed rule `remap_contacts` states for
discovered v-on-f rows.

`fuse/corrupt-operand-edge-contact` made `remap_contacts` group v-v rows
that share a key. At a pinch (an operand's own contact leaving two
vertices at one point), the other operand's vertex fuses into one of
the two and keeps touching the other. A carried v-on-f row naming the
pinch vertex that fuses would drop. Whether a contact then survives at
that point unrecorded is unmeasured.

## What is and is not exercised

Measured on that branch by making the two loops panic when non-empty,
over `--profile ci` for topo, editor-core, sweep, viewer and pncad
(7488 tests). Two tests reached them:
- `editor-core` `docm7_union_declare::a_same_member_declared_pair_is_a_carried_record_at_its_step`;
- `topo` `boolean::ops::tests::a_cycling_absorption_row_refuses_where_a_dead_end_drops`.

Since stage 4 B2 (`intent/contact-records-cite-their-decision`) the
first no longer reaches them: its declared pair names a contact the
operand records nothing for, so the node refuses
`DeclaredContactUnbacked`, and the test is
`a_same_member_declared_pair_with_no_record_refuses_at_every_door`.
The carried v-on-f loops are reached instead by `topo`
`records_cite_their_decision::a_carried_vertex_on_face_record_cites_its_operands_record`
(a cube on its corner, carried into a union with a far box). No
document-level fixture declares a backed v-on-f pair: `kiss_carry`
carries a v-v pair only.

Neither puts the row's vertex at a pinch. The pinch fixtures in
`crates/topo/tests/union_flush_onto_edge_contact.rs` carry v-v rows
only.

## Owed

A fixture where an operand carries a `Rest` v-on-f row whose vertex is
one of a pinch's two vertices, and the next boolean fuses that vertex,
e.g. the pinch of that suite resting on a face, with a flush partner
folded onto it. Then read 3′ on the result. A fix, if one is owed,
lands where the v-v group rule did.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: asks whether carried declared Rest v-on-f rows survive remap_carried; those declared-contact rows retire at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.

## 2026-10-13: re-scoped to the one carry (FUSE, carry-rows-after-4820)

PR 4820 retired `remap_carried` and `vert_strict`. An operand's carried
vertex-on-face row now takes this route:

1. `boolean::ops::carried` places it through what the op made of each
   input cell (`operand_fate`), by the census's record confirm
   (`records::landings`).
2. The landing is restated in the operand's clone keys.
3. `carry_rows` reads it with the reduction's rows. Its vertex-on-face
   loop still skips a row whose vertex a zip fused (`desc.fused`), the
   rest-consumed rule.

The question stands. At a pinch, the other operand's vertex fuses into
one of the pinch's two vertices and keeps touching the other. A carried
vertex-on-face row naming the vertex that fused is skipped, so does a
contact at that point survive unrecorded? Still unprobed on main
`0d2117d0`: no fixture puts a carried vertex-on-face row's vertex at a
pinch.
`records_cite_their_decision::a_carried_vertex_on_face_record_cites_its_operands_record`
(a cube on its corner, no pinch) passes tier 3′.

Owed, unchanged: the fixture this row describes, carried through
`BooleanInputs`, and its tier-3′ verdict.
