---
id: doc-param-distribution-edit-has-no-door
kind: issue
title: No DocEdit annotates a standing document parameter — SetDocParam would drop the notation
status: dispatched
opened: 2026-09-16
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
