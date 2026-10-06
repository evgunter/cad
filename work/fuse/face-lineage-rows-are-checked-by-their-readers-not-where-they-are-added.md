---
id: face-lineage-rows-are-checked-by-their-readers-not-where-they-are-added
kind: issue
title: The boolean's face lineage rows (Descendants.faces, face_fragments_*, merge_groups) are checked by their readers, not where they are added — the move Fusions made for vertices
status: open
opened: 2026-10-06
priority: P1
cost: M
refs: [survivor-folds-a-corrupt-fusion-list-onto-a-dead-key-outside-the-contact-remap, join-desync-is-the-catch-all-for-cyclic-lineage-records, 4116]
---

Found by the review of PR 4116 (ZIP, `zip/survivor-and-recourse`), as
the class that PR closed only for vertices.

## The finding

PR 4116 made a vertex fusion list a `zip::Fusions`: `Fusions::push`
and `Fusions::extend` refuse a row that fuses a key into itself or
names a key an earlier row killed, so no reader ever meets a corrupt
vertex list. The boolean's face lineage rows did not get the same
move. They are plain `Vec`/`BTreeMap` rows, any row is accepted where
it is added, and each reader checks for a cycle as it chases:

- `Descendants.faces` (`boolean/ops.rs`), written by
  `Descendants::absorb_faces` and `Descendants::absorb_merge`, refused
  for a cycle at its reader `Descendants::live_face` ("a face's
  absorption rows are cyclic");
- `BooleanNaming::face_fragments_a` / `face_fragments_b`, read through
  `lineage_root` in `discard.rs` (`stretches`: "a face's fragment
  lineage is cyclic") and by editor-core's naming
  (`NamingError::FragmentLineage`, `names/emit.rs`);
- `BooleanNaming::merge_groups`, read by editor-core's `names/emit_topo.rs`
  and `names/emit_union.rs`, whose rows nothing checks for a face
  absorbed twice or a kept face that is also absorbed.

So a corrupt face row is caught (or not) by whichever reader happens
to chase it, once per reader, and a reader that does not chase it
cannot tell.

## What is owed

A face lineage type in the shape of `Fusions`: rows validated where
they are added (no face absorbed or divided twice, no row naming a
face an earlier row retired, no self-row), so a reader's chase is
total. Then the readers' cycle refusals can go: the list cannot
cycle. Measure first which
rows are actually written in an order the check can hold at append
time: `merge_groups`' crossings and `merge_coplanar_faces`' groups are
built in two places (`boolean_op_recut`, the REST lane in `rest.rs`).

Related: `join-desync-is-the-catch-all-for-cyclic-lineage-records`
(the refusals' variant), which this would mostly retire.
