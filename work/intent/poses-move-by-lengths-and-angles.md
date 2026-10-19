---
id: poses-move-by-lengths-and-angles
kind: issue
title: Stage 3 A′ (FORK-MOVE): Slide and Spin replace InFrame/PoseCoords, Project and Standoff; reads carry a projection role; Revolve takes axis_through + axis_heading; a lone 0 lowers to VR6's constant
status: closed
closed: 2026-10-17
pr: 5243
branch: intent/s3-a-prime-poses-move-by-lengths-and-angles
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

## Closed

Built by PR 5243. `PoseDef` moves a pose by `Slide { pose, along, by: Length }` and `Spin { pose, about, by: Angle }`; `InFrame`, `PoseCoords`, `Project` and `Standoff` are gone. A pose read is a `RoleRead { read, role }`, in a definition and at a node's pose seat (a split's tool, a ring's axis), with a frame's x, y, z, origin, xy, yz and zx roles, a plane's normal and an axis's direction; both doors check a role against the kind that offers it and the kind the read admits. `Revolve` holds `axis_through: [Length; 2]` and `axis_heading: Angle` (`SlotId::AxisHeading`). `Expr` has the constant `Zero` at any dimension; a lone `0` at a slot's root lowers to it, and `SetParam` reads it at the slot's dimension. The viewer's offers compare VR8 lowerings (`Doc::lower_alike`, `Doc::lowers_to_constant`), so a constant is neither offered nor draws an offer. The façade writes `PoseDef::point_in`; Python has `PoseDef.slide`/`spin`/`point_in`, `RoleRead`, `Formula.turn_over` and `Formula.zero`.

Measured, ids masked, at `f64`, main against the branch: every corpus outcome is identical, and 8 points of `kitchen_sink` moved by at most 2.2e-16, because `turn/4`'s cosine rounds to 6e-17. An axis-aligned heading therefore moves by rounding too, not only a datum that was not axis-aligned. `review_pick_r2`'s tally (red on main, `work/cleave/review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose.md`) moves with the gallery ring; the evidence is on that row.

