---
id: slot-forms-want-one-trait
kind: issue
title: The per-type try_map_slots, authored and stored wrappers want one trait over a value generic in its slot form
status: open
opened: 2026-10-06
---


INTENT-LITERALS PR B made every slot-bearing value generic in its slot
form (`Formula` authored, `Expr` stored), and each type now hand-writes
the same three things:

- its own `try_map_slots`: `Datum`, `TubeWindow`, `PatternKind`,
  `PartSelect` (`crates/editor-core/src/node.rs:4055-4147`), `Node`
  (`node.rs:4169`), `Placement` (`crates/editor-core/src/placement.rs:620`),
  `Alignment` (`crates/editor-core/src/mate.rs:392`), and the program
  types `ProgramTarget`, `ProgramArcData`, `Step`, `LoopProgram` and
  `ProfileProgram` (`crates/editor-core/src/program.rs:2499-2648`);
- a re-authoring wrapper, each `try_map_slots(Formula::from)`:
  `Node::authored` (`node.rs:4352`), `Placement::authored`
  (`placement.rs:688`), `Alignment::authored` (`mate.rs:487`),
  `LoopProgram::authored` (`program.rs:1341`), `MeasureExpr::authored`
  (`crates/editor-core/src/measure.rs:577`);
- a test lowering helper, each `try_map_slots(Expr::try_from).expect`:
  `test_support::{stored, stored_expr, stored_program, stored_loop,
  stored_placement}` (`crates/editor-core/src/test_support.rs:81-130`).

One trait over "a value generic in its slot form" — an associated
`type In<S2>` and one `try_map_slots` — would give `authored` and
`stored` one home each. As written, every slot-bearing type PR C adds
(it adds `S = VarId`) owes three more copies.

Raised by the PR B review (#4072, S1).
