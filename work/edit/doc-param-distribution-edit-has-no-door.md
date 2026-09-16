---
id: doc-param-distribution-edit-has-no-door
kind: issue
title: No DocEdit annotates a standing document parameter — SetDocParam would drop the notation
status: open
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
