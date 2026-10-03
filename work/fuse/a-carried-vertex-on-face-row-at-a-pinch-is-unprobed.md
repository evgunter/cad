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
(7488 tests). Two tests reach them:
- `editor-core` `docm7_union_declare::a_same_member_declared_pair_is_a_carried_record_at_its_step`;
- `topo` `boolean::ops::tests::a_cycling_absorption_row_refuses_where_a_dead_end_drops`.

Neither puts the row's vertex at a pinch. The pinch fixtures in
`crates/topo/tests/union_flush_onto_edge_contact.rs` carry v-v rows
only.

## Owed

A fixture where an operand carries a `Rest` v-on-f row whose vertex is
one of a pinch's two vertices, and the next boolean fuses that vertex,
e.g. the pinch of that suite resting on a face, with a flush partner
folded onto it. Then read 3′ on the result. A fix, if one is owed,
lands where the v-v group rule did.
