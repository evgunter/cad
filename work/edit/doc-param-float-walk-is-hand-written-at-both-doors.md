---
id: doc-param-float-walk-is-hand-written-at-both-doors
kind: issue
title: A doc param's finiteness is spelled twice: param_site at the load door, SetDocParam at the edit door
status: closed
pr: 2780
branch: edit/one-predicate-round-three
opened: 2026-09-16
refs: [three-door-predicates-are-hand-copied-not-shared]
closed: 2026-09-16
---


Found by the `edit/one-predicate-round-two` unit's fresh census of
`validate_document` (entry 2 of that table, on
`three-door-predicates-are-hand-copied-not-shared`), which read every
refusal the function can produce rather than only the sites spelling
`SnapshotError` inline.

**The predicate, twice.** "A continuous document parameter's stored
floats are all numbers" is decided at both doors, by two functions:

- `crates/editor-core/src/persist/check.rs`'s `param_site` — matches
  `DocParam::Continuous { value, .. }` for `!value.is_finite()`, then
  `distribution.first_non_finite()`, and answers a
  `NonFiniteSite::DocParam { name, field }` naming WHICH offset.
- `crates/editor-core/src/edit.rs`'s `SetDocParam` arm — tests
  `!value.is_finite()` on the nominal, then routes
  `DistributionFault::NonFinite` out of `Distribution::check` into
  `EditError::NonFiniteDocParam { name }`.

The *distribution* half is already one rule asked two ways
(`Distribution::first_non_finite` against `Distribution::check`'s
`NonFinite` arm); the *nominal* half is two `is_finite` calls with no
function between them.

**The shape the fix takes**, the same one the round-two unit made three
times: one predicate over a `&DocParam` — say
`DocParam::first_non_finite() -> Option<Option<DistributionField>>`, or
a small fault enum — with one home beside `DocParam`, asked by the
create-or-replace edit door and by `first_non_finite`'s walk, each
naming the answer in its own vocabulary. The load door already carries
the richer answer (which field), so the move is toward it, not away.

**Why it was not done in that unit.** Its three moves each unified a
predicate whose two spellings were already the same question; this one
is not, quite: the edit door answers "refuse this parameter" and the
load door answers "this float, at this site, is what the format cannot
write", and the `NonFiniteSite` vocabulary is persisted-refusal
vocabulary with a Python tag behind it. Deciding whether the edit door
should carry the field name too is a small design question, which makes
this a unit rather than a follow-through.

## Dispatched (2026-09-16, EDIT orchestrator) — middle tier, with the slot-dimension row

Built by the unit `edit/one-predicate-round-three`, whose spec is the
`## Spec` section of `load-door-checks-slot-dimensions-for-profile-nodes-only`;
this row's decision is ruled there.

## Built (2026-09-16, `edit/one-predicate-round-three`)

One predicate beside `DocParam`: `DocParam::first_non_finite() ->
Option<DocParamField>` — the nominal, or which distribution offset —
asked by `write_doc_param` (every parameter door's shared tail) and by
the load door's `param_site`.

The move is toward the richer answer, as this row said: the edit
door's refusal carries the field too
(`EditError::NonFiniteDocParam { name, field }`), and so does the load
door's site, which now spells the nominal as `DocParamField::Nominal`
rather than as an absent `DistributionField`. `DocParamField` is the
one word for that answer at both doors; the F6 row and the existing
rows that read either refusal now pin WHICH float.

`edit_one_predicate::a_non_finite_doc_param_is_refused_at_both_doors_naming_the_field`
is the both-doors row, over the nominal and over a sigma.

Outside EDIT's fence, mechanical: `crates/pncad/src/document.rs`
carries `DocParamField`, `crates/pncad-py/src/edit_payload.rs` names
the new field as uncrossed, and the binding census lists it as
flattened. LIB's files.
## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2780; the record is on
`load-door-checks-slot-dimensions-for-profile-nodes-only`'s `## Closed`.
