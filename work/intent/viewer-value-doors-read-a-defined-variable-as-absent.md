---
id: viewer-value-doors-read-a-defined-variable-as-absent
kind: issue
title: The viewer's range probe and exists-notice read a defined variable as one the document does not hold
status: open
opened: 2026-10-05
---

INTENT-LITERALS PR A gave a variable a second definition, `Defined`,
and the viewer's value doors ask `Doc::free`, which answers `None` for
one. Each then speaks a defined variable as absent:

- `probe.rs`'s `BoundsTarget::Param` arm
  (`crates/viewer/src/session/probe.rs`, the `doc.free(*var)` lookups)
  refuses `Refusal::NoSuchParam`, whose sentence is "variable … is not
  in this document", and its sample edit answers `None`;
- the add-parameter form's exists-notice
  (`crates/viewer/src/pane/properties.rs`, the `committed.free(var)?`
  read) draws no notice for a name a defined variable holds, so the
  declare door's `VarNameTaken` is the first the person hears of it.

The value gesture was the third, and PR A's review pass closed it:
`DocSession::begin_param_gesture` now opens over a defined variable at
its kind, and the value door refuses its first preview
(`EditError::NotAFreeVar`), because a defined variable's row now has a
field a person can drag.

The panel draws no bounds row for a defined variable, so neither of
the two left is reachable from a click today; each is reachable from a
`SessionOp` built by hand. The edit doors behind them already answer
correctly (`EditError::NotAFreeVar`). The fix is a refusal that says
the variable is defined, decided with
`viewer-param-vocabulary-names-a-variable` and
`typing-a-value-mints-or-offers-a-variable`, where the GUI's variable
affordances are designed.
