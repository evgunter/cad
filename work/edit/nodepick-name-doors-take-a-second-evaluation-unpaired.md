---
id: nodepick-name-doors-take-a-second-evaluation-unpaired
kind: issue
title: NodePick::patch_names and boundary_names take a second evaluation and check no pairing
status: review
opened: 2026-09-16
refs: [2723, 1098]
branch: edit/nodepick-pairing
pr: 0
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

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/nodepick-pairing`. The shape is A2a's (`crates/editor-core/ASSEMBLY.md`):
a door that takes a value OF one document plus an evaluation refuses a
mismatch typed, before reading anything of the evaluation, through the
one predicate `ident::mispaired`. `PickMemo` already does this for
the index (`PickEntry.document`, compared at the memo lookup); the
name doors do not. DI3 (`crates/editor-core/IDENTITY.md`) stamps the
document half only — the version half is decided by content keys —
and this unit keeps that line.

1. `NodePick` carries `document: DocumentId`, stamped from the
   building evaluation (`eval.document`) at every constructor
   (`build`, `build_with`, `build_all`, `build_all_with`).
2. `patch_names` and `boundary_names` refuse a mispaired evaluation
   FIRST, typed, in the door's own vocabulary (a `HitTestError` arm
   mirroring `ProductError::EvaluationOfAnotherDocument { expected,
   found }`), through `ident::mispaired(self.document, eval.document)`.
   A mispairing is one refusal of the CALL, not a refusal per patch:
   say what the signature becomes and why, and measure the callers
   (`crates/viewer`, `crates/pncad`, `crates/pncad-py`) before choosing
   — a per-row `Err` that repeats one fact `n` times is the shape to
   argue against.
3. Sweep `resolve/pick.rs` for every other door that pairs a
   `NodePick`-derived value with an evaluation: `pick_face` takes
   `PickTarget`s built from one evaluation and an `eval` argument and
   checks node standing but not the document; `entity_name`'s callers.
   Every such door refuses the same way, or the PR body says why it
   cannot mispair (the row's "by caller convention alone" is not a
   reason).
4. A2a's list of pairing doors gains the `NodePick` doors, and its
   closing paragraph, which names them as the open row, is re-worded:
   that is a description moved by an approved change, not a second
   decision (CLAUDE.md, the merge-only rules), and lands here with the
   sentence in the PR body saying so. DI3 needs no edit.
5. Rows: the review probe
   `nodepick_patch_names_answers_out_of_a_twins_tables` on
   `origin/review/pair-rv` (a square prism's index handed a triangular
   prism's evaluation; five of six named by the twin, three wrong)
   becomes the refusal row — adopt it authorship-preserving and turn
   its "asserts what happens today" into the refusal, for
   `patch_names`, `boundary_names` and every door item 3 finds. A
   same-document later evaluation (after an edit that re-tessellates
   the node) is ADMITTED by this unit — that is DI3's line, the
   content key's business — and one row says so, so the boundary is
   pinned rather than implied. A mutant that drops the stamp
   comparison reds every refusal row.

## Built (2026-09-16)

`NodePick` carries `document: DocumentId`, stamped from `eval.document`
at `build` and `build_with` (`build_all` / `build_all_with` enumerate
through them, so all four constructors stamp). `PickMemo`'s
`PickEntry.document` field is gone: the memo's document half of the key
is now the memoised pick's own stamp, so the two cannot drift.

`patch_names` and `boundary_names` return
`Result<Vec<Result<StableName, HitTestError>>, HitTestError>` and refuse
`HitTestError::EvaluationOfAnotherDocument { expected, found }` through
`ident::mispaired`, before any table is read. The refusal is of the
CALL — one fact about the arguments, outside the vector — and the
per-entity `Unnamed` lane keeps its own meaning.

Item 3's sweep also reached `pick_face`, which could mispair and now
cannot: `PickTarget` gained a public `document` field, `NodePick::target`
stamps it, and `pick_face` refuses the same arm for the first offending
target BEFORE the standing loop (a twin mints the same node ids, so
standing admits it). The node half of `PickTarget`'s provenance contract
is untouched — arena keys collide across sibling nodes of ONE document
and there is nothing to compare — and stays #1098's residual raw-assembly
class, still witnessed by the ignored row
`gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`.

A2a's door list and its closing paragraph re-worded for the three new
doors and for what the stamp does not decide (the description an
approved change moved, not a second decision). DI3 unedited.

Rows, in `crates/editor-core/tests/edit_pair_apply_names.rs`: the
`pair-rv` probe becomes `the_name_doors_refuse_a_twins_evaluation`
(both doors, plus the shape assertion that a per-slot spelling would
fail), `pick_face_refuses_a_target_of_another_document`, and
`a_later_evaluation_of_the_same_document_is_admitted` (the prism's own
extrusion distance edited between the runs, content key asserted moved).
A mutant that drops the stamp comparison reds the first two and leaves
the third green. Python mirrors in
`crates/pncad-py/tests/test_picking.py`
(`TestThePickIndexPairsWithItsDocument`, five rows including the premise
that the twin's tables would have answered).

Not done here: `face_name` / `edge_name` / `vertex_name` (`resolve/hit.rs`)
take a raw arena key beside the evaluation and carry no provenance at
all, so there is no stamped value for `ident::mispaired` to run on —
the raw-key class, not this one.
