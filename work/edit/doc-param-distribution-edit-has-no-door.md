---
id: doc-param-distribution-edit-has-no-door
kind: issue
title: No DocEdit annotates a standing document parameter — SetDocParam would drop the notation
status: closed
opened: 2026-09-16
pr: 2774
branch: edit/doc-param-distribution
closed: 2026-09-16
---



## Finding

**The third field of a `DocParam::Continuous` declaration has no
carry-forward door, and the create-or-replace route through it drops
the notation.** `DocParam` carries `dim`, `value`, `display_unit` and
`distribution` (`crates/editor-core/src/doc.rs`). Two of those now have
a narrow edit that carries the rest of the declaration forward:
`DocEdit::SetDocParamValue` through `DocParam::with_value`, and
`DocEdit::SetDocParamUnit` through `DocParam::with_display_unit`
(`crates/editor-core/src/edit.rs`). `distribution` has neither.

So adding or changing an E1/E2 annotation on a standing parameter goes
through `DocEdit::SetDocParam` — create-or-replace — and the authoring
spelling for an annotated parameter is
`DocParam::continuous_with(dim, value, distribution)`, which writes
`display_unit: UnitSym::canonical_for(dim)` (`doc.rs`). A parameter
authored in millimetres therefore reverts to metres the moment anyone
annotates it, with no refusal and no diagnostic. That is the
`B-DISTRIBUTIONS` trap in the mirror: the same door, the same silence,
the other field.

The only spelling that keeps both is the raw
`DocParam::Continuous { .. }` struct literal, which is the trap the
charter names.

## What is needed

The shape is written by the two doors that already exist:
`DocParam::with_distribution(&self, Option<Distribution>) -> Option<Self>`,
exhaustive on both arms, `None` for a `Count` (a structural parameter
is fixed under any error analysis — E11.3 — which is why the `Count`
arm has no field to hang one on), running the same `Distribution::check`
the edit door already runs; and a `DocEdit` arm routing through it,
refusing typed on an undeclared name (the existing
`EditError::DocParamNotDeclared` — the sentence is the same) and on a
`Count`. Rows: the notation survives an annotation edit (RED today
through `SetDocParam` with `continuous_with`), each refusal by name,
and replay/round-trip.

## The question it carries

Whether CLEARING an annotation is the same door with `None` or a
second edit. `SetAppearanceMeta`/`ClearAppearanceMeta` are two arms;
`with_value` has no clearing case to rule on. Whoever takes this should
state the reading rather than assume it.

## Home

`work/edit/` — `doc.rs` and `edit.rs` are EDIT's `paths`.

## Found by

The sweep of `edit/doc-param-unit` (the unit-only door), whose pattern
was "a field of the `DocParam` declaration a narrow edit can move";
`display_unit` was the unit's subject and `distribution` is the one
remaining hit.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/doc-param-distribution`. The mirror of the unit door
(`SetDocParamUnit` through `DocParam::with_display_unit`, PR #2732)
over the third field.

1. `DocParam::with_distribution(&self, Option<Distribution>) ->
   Result<Self, DistributionRefusal>` — `Result`, as the unit door's
   is, because there are two refusals: a `Count` (a structural
   parameter is fixed under any error analysis, E11.3; the arm has no
   field) and a distribution `Distribution::check` refuses. Exhaustive
   on both arms; carries `dim`, `value`, `display_unit` forward
   untouched.
2. `DocEdit::SetDocParamDistribution { name, distribution:
   Option<Distribution> }` routing through it, refusing typed on an
   undeclared name (`EditError::DocParamNotDeclared` with a new
   `CarryForwardDoor` arm) and on the two refusals above, each in the
   edit door's vocabulary.
3. **The clearing question, ruled here:** ONE door, `None` clears.
   The field is `Option<Distribution>` and "no annotation" is a VALUE
   of the declaration (E1/E2: absent means no error analysis applies),
   so writing `None` is the same carry-forward edit as writing `Some`.
   `SetAppearanceMeta`/`ClearAppearanceMeta` are two arms because a
   meta entry is a row in a map, where clearing removes the row rather
   than writing a value; that is a different shape, and the door's doc
   says so in one sentence. If the tree contradicts this reading,
   report it before building on it.
4. Rows, red first where they can be: the notation survives an
   annotation edit (RED today through `SetDocParam` with
   `continuous_with`, the row's finding — write it first); the value
   survives; each refusal by name (undeclared; `Count`; a distribution
   `check` refuses); clearing keeps unit and value; replay and
   round-trip of set, change and clear; the F6 census for the new
   refusal; a mutant that routes the new arm through `SetDocParam` reds
   the first row.
5. The `pncad-py` follow-through (the `DocEdit` payload, `.pyi`, tags,
   binding census) is LIB's, mechanical, taken here as the unit door
   took it; say so in the PR body.

## Built (2026-09-16)

Branch `edit/doc-param-distribution`.

- `DocParam::with_distribution(&self, Option<Distribution>) ->
  Result<Self, DistributionRefusal>` in `doc.rs`, exhaustive on both
  arms, carrying `dim`, `value` and `display_unit` forward. Its two
  refusals are `DistributionRefusal::CountHasNoAnnotation` and
  `::Invalid { fault }` from the same `Distribution::check` the
  persistence doors run.
- `DocEdit::SetDocParamDistribution { name, distribution }` in
  `edit.rs`, routing through it, with `CarryForwardDoor::Annotation`
  ("an annotation edit") and a new
  `EditError::DocParamCountHasNoDistribution`. The E2 fault maps
  through `distribution_fault_error`, extracted from `write_doc_param`
  so both doors report one answer.
- The clearing question is ruled as the spec ruled it: ONE door,
  `None` clears, and the reason is written once in
  `with_distribution`'s rustdoc. Nothing in the tree contradicted it.
- `persist::check`'s `edit_non_finite` learned the new arm — it is a
  real float carrier, unlike the notation door — reporting the
  offending field through the existing `NonFiniteSite::DocParam`.
- Nine rows in `crates/editor-core/tests/edit_doc_param_distribution.rs`;
  the F6 prose contract for the new refusal is `display_contract.rs`'s
  census, and the suite row holds only the residue.
  The corpus sink exercises the new kind (`EDIT_KINDS` is 17).
- The `pncad-py` follow-through is LIB's, taken here mechanically:
  the façade re-export, `DocEdit.set_doc_param_distribution`, the
  `.pyi` stub, the tag `doc_param_count_has_no_distribution`, the
  payload arm and the two census rows.
- Filed on LIB's slate:
  `work/lib/doc-param-edit-doors-drop-the-python-dimension.md`.

## After the review (2026-09-16)

- `write_doc_param`'s and `check_node_slots`' rustdoc, which the
  extraction of `distribution_fault_error` had stacked above the
  extracted function, are re-homed. The sweep behind it — every free
  `fn` in `edit.rs` and `persist/check.rs` against the doc block
  immediately above it — found no other instance.
- `write_doc_param`'s header names its four callers (create-or-replace
  plus the three carry-forward doors), `DocEdit`'s header and
  `EditError::DocParamNotDeclared` name the third door, and the two
  `use crate::distribution::` lines are one.
- E11.3 ("a count is a structural parameter, fixed under any error
  analysis") has ONE declared home, a section of
  `DocParam::with_distribution`'s rustdoc; five other sites cite it in
  a clause. The Python pair cites `DocParam.count`, which already
  declares it in that language.
- The doubled `Distribution::check` stays doubled, with the reason
  written at the door: `SetDocParam` reaches the shared write path
  without `with_distribution`, and `with_distribution` is a `pub` door
  reachable without `apply`.
- The Python door's reason for dropping the annotation's dimension is
  the true one (a kernel `Distribution` is dimension-free, so the
  wrapper's `dim` is discarded building the payload), pointing at
  `work/lib/doc-param-edit-doors-drop-the-python-dimension.md`; the
  two LIB doc sites that told a caller to wait for this door are
  present tense.
- The suite's F6 row keeps only what the `display_contract.rs` census
  does not say — that the parameter name is interpolated, and that
  `DistributionRefusal` renders on its own.

## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2774 after one opus style review (MERGEABLE:
three doc-level MINOR, two NOTE, nine style findings; every one taken
or argued with a measurement in the fix pass). The third carry-forward
door mirrors the unit door exactly where it should
(`DocParam::with_distribution`, `DocEdit::SetDocParamDistribution`,
`CarryForwardDoor::Annotation`) and differs only where the field does;
`None` clears through the same door, ruled on this row; the
create-or-replace door and this one classify a distribution fault
through one function. Red first (the notation reverting from mm to m
through `SetDocParam`), five mutants each redding rows. The one
disclosed deviation (the non-finite offset reached through `save` of
an in-memory log, since JSON cannot spell one) is right. Residue in its
own files: `work/lib/doc-param-edit-doors-drop-the-python-dimension`
(the Python wrapper's dimension is dropped at the door because the
kernel `Distribution` carries none — LIB's call) and
`work/chrome/props-header-says-no-door-between-create-or-replace-and-value`
(a viewer header premise both carry-forward doors falsified). One
baseline moved: the kitchen sink's persisted-text hash, for one inert
metadata edit added to it; its name-tables digest did not.
