---
id: poses-move-by-lengths-and-angles
kind: issue
title: Stage 3 A′ (FORK-MOVE): Slide and Spin replace InFrame/PoseCoords, Project and Standoff; reads carry a projection role; Revolve takes axis_through + axis_heading; a lone 0 lowers to VR6's constant
status: open
opened: 2026-10-15
priority: P0
cost: H
refs: [a-pose-is-moved-by-lengths-and-angles, intent-stage3-is-built]
---


FORK-MOVE (fork log row 149; ruling `a-pose-is-moved-by-lengths-and-angles`, ratified by Ev on PR 5189). The unit is specified in `docs/INTENT-STAGE3-SPEC.md` under "A′ (FORK-MOVE), after A and before C and D". Stage 3's A has landed `InFrame`/`PoseCoords`, `Project`, `Standoff` and `Revolve { axis_origin: [S; 2], axis_direction: [S; 2] }` (`crates/editor-core/src/pose.rs`, `eval/wire/pose.rs`, `node.rs`). A′ restates them as §1 now says:
- `Slide { pose, along, by: Length }` and `Spin { pose, about, by: Angle }`;
- reads that carry a projection role, adding the x and y roles that today's projection arms lack;
- `Revolve`'s `axis_through: [Length; 2]` plus `axis_heading: Angle`.

A′ also builds VR6's constant `0`, which is unbuilt: `Formula::slot_root` (`crates/editor-core/src/formula.rs`) mints a free `Scalar` for a lone `0`, and `Expr` has no zero leaf. With it, `equal_variables` (`crates/viewer/src/props.rs`) compares VR8 lowerings. C and D wait on this unit.
