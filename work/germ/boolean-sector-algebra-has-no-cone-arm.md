---
id: boolean-sector-algebra-has-no-cone-arm
kind: issue
title: Past the pair gate and the cone's crossing lane, every op on a cone operand refuses at sectors::sector_face: the boolean's sector algebra has no cone arm
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [VERBS-CONE, an-edge-crossing-a-cone-face-has-no-root-lane]
---


Found by the REACH lane that gave a cone face its crossing lane
(`an-edge-crossing-a-cone-face-has-no-root-lane`).

## Measured (2026-10-06, on that lane's branch)

The boolean refuses a cone operand at the pair gate
(`reduce::gate_operand_pairs`, `CurvedPairUnsupported { site:
OperandGate, kind: Cone }`): `reduce::boolean_arm_exists` leaves `Cone`
off its roster. With `Cone` put on the roster as an experiment (never
committed), the crossing sweep of finished bodies completes in both
directions — a widening and a narrowing frustum (revolves about `y`,
radii 1/2 → 1 and 1 → 1/2 over `y ∈ [0, 1]`) against a cube turned so
its body diagonal is the cone's axis, an axis-aligned brick, a thin
brick past a full cone's apex, a coaxial rod through a cap and a rod
tilted across the wall — and every op (∪, ∩, A ∖ B, B ∖ A) then refuses
`CurvedBooleanUnsupported { kind: Cone }` at one site,
`sectors::sector_face`'s `SectorCarrier::Cone` arm ("the boolean's
sector algebra has no cone arm"). A pose whose edge meets the apex
refuses `CrossingAtConeApex` first, and one whose rim lies on the cone
`CurvedPierceUnsupported`.

`topo::sweep_split_admitting_cones` (`sweep-testing`) reaches the same
point from a finished body without the experiment, and
`sweep/tests/reach_cone_root_lane.rs` holds its splits against an
oracle; `every_op_refuses_a_cone_operand_at_the_pair_gate` there pins
the gate and goes red when this lane, with `VERBS-CONE`, opens it.

## What

The sector algebra downstream of `sector_face` (`within`, `side_code`,
`sector_overlap`, the wideness and bisector algebra,
`insert::germ_dir`, `vtxfac::pierce_germ_dir`) reads a pierce's
material side off the face's outward normal at the pierce point. A
cone's normal is defined everywhere off the apex
(`geom_brep::implicit_gradient`), and the crossing lane refuses a root
within the band of the apex (`CrossingAtConeApex`). Unmeasured: whether
the cylinder's arm read with the cone's normal is the whole of it, or
the second-order reading at a tangency needs its own arm (a cone's
normal curvature varies along a generator). The split lane already
reads a cone sector (`splitting::neighborhood`, which takes the
cylinder's arm for it).
Behind this site: the join's cone germ pairs
(`cone-pairs-in-general-pose-have-no-section-arm`, `VERBS-CONE`).
