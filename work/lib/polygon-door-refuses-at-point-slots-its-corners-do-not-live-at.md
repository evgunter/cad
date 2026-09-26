---
id: polygon-door-refuses-at-point-slots-its-corners-do-not-live-at
kind: issue
title: The Python polygon door checks every corner against step i's PointX slot, but a polygon's corners past the first live at TargetX/TargetY, so a dimension refusal names a slot the program does not have
status: open
opened: 2026-09-25
priority: P3
cost: E
refs: [step-arg-roles-are-spelled-in-three-homes]
---


(GATHER lane, `gather/step-arg-roles-one-home`.) Found by the sweep for
per-consumer spellings of a `StepArg` role assignment.

`Node.polygon` in `crates/pncad-py/src/py/doc.rs` (the `point` closure,
about `:1766-1778`) checks each corner's two expressions against
`SlotId::Profile { loop_: 0, step: i, arg: PointX / PointY }` for every
corner `i`. The program it then hands the corners to,
`LoopProgram::polygon_expr` (`crates/editor-core/src/program.rs`), makes
corner 0 an `At` step, whose roles are `PointX`/`PointY`, and every later
corner a `LineTo(Point)` step, whose roles are `TargetX`/`TargetY`. So a
wrong-dimension coordinate on any corner past the first raises
`SlotDimensionMismatch` naming a slot the program does not have.
`work/lib/LIB-SEATS.md` (the `slot_expr` bullet) and `work/lib/log.md`
say the door's refusal is "the value `apply` would raise for the same
expression in the same slot". That holds for corner 0 only.

The door hand-spells a role assignment that the program's role table
(`loop_roles` in `program.rs`) already holds. It is the fourth spelling
in the class the GATHER row `step-arg-roles-are-spelled-in-three-homes`
reduced to one. The fix that removes the spelling is to build the
program first, then check each `(step, arg)` of
`LoopProgram::step_args()` against its expression's dimension, which
reads the addresses from the program rather than restating them. The
dimension would still come from `SlotId::dimension()`, as it does now.
Nothing tests the door's refusal past corner 0.
