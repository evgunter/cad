---
id: viewer-value-doors-read-a-defined-variable-as-absent
kind: issue
title: The viewer's range probe and exists-notice read a defined variable as one the document does not hold
status: closed
opened: 2026-10-05
closed: 2026-10-07
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

## Closed

Closed by the stage-1 GUI unit (`typing-a-value-mints-or-offers-a-variable`).
The probe's seed reads the variable (`Doc::var`) and refuses a defined
one with a new flat arm, `Refusal::VariableIsDefined` ("d is defined by
a formula and holds no value of its own to move — probe a variable it
reads"); `NoSuchVariable` is kept for an id the document does not hold.
The sample edit reads the dimension off the variable's kind, so it no
longer answers `None` for a defined one. The add-variable form's notice
reads `props::named_variable`, which answers a defined variable at its
kind's dimension. Rows: `gui_variables::the_range_probe_says_a_defined_variable_is_defined`,
`gui_variables::the_exists_notice_reads_a_defined_variable_as_holding_its_name`.
