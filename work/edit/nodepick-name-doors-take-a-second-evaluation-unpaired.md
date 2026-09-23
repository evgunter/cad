---
id: nodepick-name-doors-take-a-second-evaluation-unpaired
kind: issue
title: NodePick::patch_names and boundary_names take a second evaluation and check no pairing
status: closed
opened: 2026-09-16
refs: [2723, 1098]
branch: edit/nodepick-pairing
pr: 2773
closed: 2026-09-16
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
is now the memoised pick's own stamp, so the two cannot drift. That
comparison is the memo's own DI3 refusal, said so in `PickMemo`'s doc
and pinned by `the_memo_refuses_a_prior_of_another_document`.

`patch_names` and `boundary_names` return
`Result<Vec<Result<StableName, HitTestError>>, HitTestError>` and refuse
`HitTestError::EvaluationOfAnotherDocument { expected, found }` through
`ident::mispaired`, before any table is read. The refusal is of the
CALL — one fact about the arguments, outside the vector — and the
per-entity `Unnamed` lane keeps its own meaning.

Item 3's sweep also reached `pick_face`, which could mispair and now
cannot: `PickTarget` carries the document, `NodePick::target` stamps it,
and `pick_face` refuses the same arm for the first offending target
BEFORE the standing loop (a twin mints the same node ids, so standing
admits it; `the_pairing_refusal_wins_over_standing` pins the order
against a document in which the node does not exist at all).

`PickTarget`'s fields are PRIVATE, with two mints: `NodePick::target`,
where every half comes from one tessellation, and `PickTarget::new`,
where a caller declares them over a mesh index of its own. A minted
target cannot be taken apart and re-stamped — neither type hands its
`MeshPick` out — so the review's forgery
(`PickTarget { document: twin, ..pick.target() }`) is a compile error,
with a `compile_fail,E0451` row on the type. What remains is the raw
path, where all four halves are the caller's claim uniformly; the node
half stays #1098's residual raw-assembly class, still witnessed by the
ignored row
`gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`.

`ident::Mispaired` projects through ONE `impl From<Mispaired>` per error
type, at the error type (`ProductError`, `ChecksError`, `EditError`,
`HitTestError`, `MateFault`); the eight sites that spelled the field
mapping by hand are one `.into()` each. `pick.rs`'s local helper is
`mispairing` — named for what it returns — over that `From`.

A2a's door list and its closing paragraphs re-worded for the three new
doors, for what the stamp does not decide, and for the rule's boundary:
it binds only where BOTH halves carry an identity, so a raw arena key or
a bare name is outside it rather than an unchecked door. DI3 unedited.

Rows, in `crates/editor-core/tests/edit_pair_apply_names.rs`: the
`pair-rv` probe becomes `the_name_doors_refuse_a_twins_evaluation` (both
doors, the shape assertion, and the premise asserted on the TWIN — its
own index answers a full name set that is not the square's),
`pick_face_refuses_a_target_of_another_document`, and
`a_later_evaluation_of_the_same_document_is_admitted`, which now asserts
what the later run ANSWERS: the same names slot for slot (a
program-anchored name does not move when a parameter does — measured),
and, when the same parameter is driven to a degenerate value, the call
still admitted with every slot `NodeFailed`. Lane `nodepick-rv`'s five
probes are adopted authorship-preserving and become rows:
`the_memo_refuses_a_prior_of_another_document`,
`the_pairing_refusal_wins_over_standing`,
`a_raw_target_is_a_claim_in_every_half`,
`the_pairing_arm_renders_both_documents` (both ids' `hex()`, as the
`product` twin asserts) and `what_the_admitted_later_evaluation_answers`
(the memo misses and rebuilds on the moved key, so a caller through
`PickMemo` never pairs a stale index).

Mutants: `mispairing -> None` reds five rows (two before the fix pass);
the memo's document comparison -> `true` reds exactly one, where it red
none before. Python mirrors in `crates/pncad-py/tests/test_picking.py`
(`TestThePickIndexPairsWithItsDocument`), whose admitted row now edits
the INDEXED node as the Rust row does.

Not done here: `face_name` / `edge_name` / `vertex_name` (`resolve/hit.rs`)
take a raw arena key beside the evaluation and carry no provenance at
all, so there is no stamped value for `ident::mispaired` to run on —
the raw-key class, not this one. Not taken from the review: splitting
`pick.rs` (a TINT-shaped row if anyone wants it) and a public
`PickTarget::document()` accessor (no caller, and one would re-open the
mint the private fields closed).

## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2773 after one opus style review (MERGEABLE:
three MINOR, two NOTE, ten style findings, every one taken in the fix
pass). `NodePick` carries its document's identity from the evaluation
that built it; `patch_names`, `boundary_names` and `pick_face` refuse a
mispaired evaluation through `ident::mispaired` before reading it, in
`HitTestError`'s own arm; the memo's document comparison has its first
row; `PickTarget` cannot be re-stamped (its fields are private, its two
mints read the document off the evaluation); the six-then-eight hand
copies of the mispairing projection are `From<Mispaired>` impls. A2a's
list gains the three doors and its rule gains the qualifier it always
meant — both halves must carry a stamp — recorded as a description
moved by the change, with `git log -S` finding no ratification of the
sentence replaced. Residue in its own file:
`pick-face-raw-target-path-survives-only-for-rows` (the raw target
path stays for four rows and is a claim in every half, measured).
