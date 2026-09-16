---
id: nodepick-name-doors-take-a-second-evaluation-unpaired
kind: issue
title: NodePick::patch_names and boundary_names take a second evaluation and check no pairing
status: open
opened: 2026-09-16
refs: [2723, 1098]
---

## What

Found by the review lane of PR 2723 (`edit/pair-apply-names`), whose
sweep names its own blind spot — "a door that takes the pair as a
method with the document or the evaluation on `&self`" — and then does
not sweep it. Two live members of that class:

- `NodePick::patch_names(&self, eval: &Evaluation<f64>)`
  (`crates/editor-core/src/resolve/pick.rs:1053`)
- `NodePick::boundary_names(&self, eval: &Evaluation<f64>)`
  (`crates/editor-core/src/resolve/pick.rs:1087`)

A `NodePick` is BUILT from an evaluation (`NodePick::build`,
`pick.rs:847`, which is where the type's "pairing true by
construction" claim comes from) and then handed a SECOND, unrelated
evaluation at each of these two doors. The index keeps `node`, `body`
and the mesh; it keeps no `DocumentId`, so nothing here can run
`ident::mispaired` even in principle, and the second evaluation is
paired with the index by caller convention alone. `PickEntry` in
`PickMemo` does carry `document` and does compare it
(`pick.rs:907`) — the memo knows the question; the name doors do not.

This is DI3's premise one layer out: node ids and output-body indices
are minted per document, so a twin recipe's evaluation answers the
lookups and returns ITS names, in patch order, with no refusal.

## Measured

Review probe `edit_pair_apply_names::
nodepick_patch_names_answers_out_of_a_twins_tables` on branch
`review/pair-rv` (a green documentation row: it asserts what happens
today). A square prism's index, handed a triangular prism's
evaluation:

```
PROBE NodePick::patch_names: 6 patches, 5 named by the twin, 3 answers differ
```

Five of six patches come back `Ok(name)` out of the twin's tables and
three of the six differ from the truth. No `HitTestError::Unnamed`,
no refusal — a confident wrong name, which is the class #1098 records
for raw pick targets and which this type's atomic-provenance door was
built to close.

## Where it stands

`crates/editor-core/src/resolve/pick.rs` is EDIT's (`work/edit/program.md`
paths). Outside the fence of
`pair-doors-outside-the-three-do-not-check-document-identity`, which
is written for `(document, evaluation)` doors and has moved to
`work/props/`: this pair is `(value derived from evaluation A,
evaluation B)`, so the stamp it would need is not the one DI3 puts on
the argument list. Whether the fix is a `DocumentId` on `NodePick`, a
typed refusal, or a signature that never takes the second evaluation
is open.
