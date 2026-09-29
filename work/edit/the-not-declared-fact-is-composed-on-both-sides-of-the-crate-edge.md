---
id: the-not-declared-fact-is-composed-on-both-sides-of-the-crate-edge
kind: issue
title: EditError::DocParamNotDeclared and the viewer's Refusal::undeclared_wording each compose 'parameter X is not declared'
status: open
opened: 2026-09-28
priority: P4
cost: E
refs: [three-spellings-say-a-parameter-is-not-declared]
---

Found by `three-spellings-say-a-parameter-is-not-declared` (VNEWS),
which converged the viewer's two spellings of an undeclared parameter
on one composer and chose its words to match the edit door's.

## The two compositions

- `crates/editor-core/src/edit.rs`, `EditError`'s `Display`, the
  `DocParamNotDeclared` arm: *parameter {name} is not declared, so
  {door} has no declaration to carry forward — {UNDECLARED_PARAM_RECOURSE}*.
- `crates/viewer/src/session/refuse.rs`, `Refusal::undeclared_wording`:
  *parameter {name} is not declared* — the fact the viewer's
  `Refusal::NoSuchParam` opens with and the properties pane draws
  bare.

The RECOURSE is already one const in editor-core
(`UNDECLARED_PARAM_RECOURSE`, whose doc says why the two doors converge
on it and not on the sentence). The FACT that opens both sentences is
two literals held in step by nothing.

## The fix

Give the fact its home beside the recourse — an editor-core function
composing *parameter {name} is not declared*, spent by the
`DocParamNotDeclared` arm — and have `Refusal::undeclared_wording`
return it. The crate edge already exists (`refuse.rs` imports
`UNDECLARED_PARAM_RECOURSE`); editor-core is EDIT's ground, so the
viewer lane that found this did not cross it.
