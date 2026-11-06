---
id: slot-forms-want-one-trait
kind: issue
title: The per-type try_map_slots, authored and stored wrappers want one trait over a value generic in its slot form
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-11-06
branch: reads/slot-forms-trait
pr: 6569
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

## Closed: the need is gone (2026-11-06)

The premise above is stale, and a trait would now be a forced
abstraction. As of `27ef07cec9`:

- **Re-authoring is not `try_map_slots(Formula::from)` any more.** A
  stored slot is a `VarId` with no dimension, so re-authoring writes
  `Formula::var(var, dim)`, and `dim` comes from the slot's ADDRESS.
  A trait over only `try_map_slots` cannot write it. Re-authoring has
  one home at the level it is used: `Node::authored_with`
  (`crates/editor-core/src/node.rs`). It needs `&Doc` for an
  assertion's payload reads. It reaches the payload through the
  `ProfilePayload::authored_with` seam (`program.rs`), and for each
  loop through `LoopProgram::authored_with`. `Placement`,
  `Alignment` and `MeasureExpr` and their `authored` are gone.
- **Lowering is not `try_map_slots(Expr::try_from)` any more.** A
  formula lowers by minting into a document. The door's one home is
  `edit::lower_value`, which is generic over the value's walk;
  `lower_node` and `lower_loops` hand it theirs. The test helpers
  `test_support::stored` and `stored_expr` had no callers, and are
  deleted. `stored_reading`, `stored_program` and `stored_loop`
  differ in what they do: reads given, a program through a node, and
  a loop with no reads.
- **The walks do not share one shape.** Six map slots alone, through
  a bare `f(&S)`: `TubeWindow`, `ProgramTarget`, `ProgramArcData`,
  `ProgramStep`, `LoopProgram` and `ClearanceBounds`. Six map reads
  alone, each handing its own address: `RoleRead`, `Coord`,
  `MeasurePrimitive`, `Bodies` and `DrawnOn` take `OperandSlot` or
  none, and `Mate` takes `MateSide`. Seven map both: `ProfileProgram`,
  `BodyRead`, `Claim`, `Constraint`, `LoftSection`, `PoseDef` (its
  scalars handed a `PoseSlot`) and `Node` (whose payload type changes
  too, `P` to `P::Authored`, which no `In<S2>` expresses). Only the
  first six fit `In<S2>` plus one `try_map_slots`, and no code reads
  any of them generically.
- **The prediction did not hold.** Nine slot-bearing types have
  landed since the row was filed (`Mate`, `Constraint`, `Value`,
  `Claim`, `BodyRead`, `PoseDef`, `LoftSection`, `StationMove`,
  `Bodies`). Each came with at most its own walk and no `authored` or
  `stored` copy, because the door and re-authoring work at node
  granularity. The one exception is `Coord::authored`, a
  read-form conversion for the bundle door.

What a trait could not remove is each type's walk over its own
fields, and that is not duplication. The generic seam that IS
consumed generically already has its trait: `ProfilePayload`
(`type Authored`, `lower`, `authored_with`), over `Node<P>`'s payload.
