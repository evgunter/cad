---
id: a-carried-vertex-on-face-row-at-a-pinch-is-unprobed
kind: issue
title: No fixture carries a vertex-on-face row whose vertex sits at a pinch, where the remap's strict rule drops a fused vertex's rest
status: parked
opened: 2026-10-03
priority: P3
cost: M
blocked_on: [intent-stage4-is-built]
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
