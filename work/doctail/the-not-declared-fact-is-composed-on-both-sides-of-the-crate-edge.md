---
id: the-not-declared-fact-is-composed-on-both-sides-of-the-crate-edge
kind: issue
title: EditError::UnknownVar and the viewer's Refusal::undeclared_wording each compose the fact that a variable is not in the document
status: open
opened: 2026-09-28
priority: P4
cost: E
refs: [three-spellings-say-a-parameter-is-not-declared]
---

Found by `three-spellings-say-a-parameter-is-not-declared` (VNEWS),
which converged the viewer's two spellings of an undeclared variable
on one composer.

## The two compositions

- `crates/editor-core/src/edit.rs`, `EditError`'s `Display`, the
  `UnknownVar` arm: *{var} is not a variable of this document, so
  {door} has nothing to act on — {UNKNOWN_VAR_RECOURSE}*.
- `crates/viewer/src/session/refuse.rs`, `Refusal::undeclared_wording`:
  *variable {var} is not in this document* — the fact the viewer's
  `Refusal::NoSuchVariable` opens with and the properties pane draws
  bare.

The RECOURSE is already one const in editor-core
(`UNKNOWN_VAR_RECOURSE`, whose doc says why the two doors converge on
it and not on the sentence). The FACT that opens both sentences is two
literals, in two wordings, held in step by nothing.

## The fix

Give the fact its home beside the recourse — an editor-core function
composing it, spent by the `UnknownVar` arm — and have
`Refusal::undeclared_wording` return it. The crate edge already exists
(`refuse.rs` imports `UNKNOWN_VAR_RECOURSE`); editor-core is DOCTAIL's
ground (and INTENT's, for D10), so the viewer lane that found this did
not cross it.
