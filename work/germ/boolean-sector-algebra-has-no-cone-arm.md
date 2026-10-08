---
id: boolean-sector-algebra-has-no-cone-arm
kind: issue
title: Past the pair gate and the cone's crossing lane, every op on a cone operand refuses at sectors::sector_face: the boolean's sector algebra has no cone arm
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [VERBS-CONE, 4135]
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

## Evidence (2026-10-07, TANG `tang/cone-ring-island-winding`)

Found while reaching for a cone face's ring lane, using the pose in
`crates/sweep/tests/a_ring_on_a_cone_face.rs`. That pose is a box turned
−50° about `z`, whose edge pierces the wall of a π/6 cone twice. Both
faces at the edge cut the cone in ellipses.

The experiment was never committed. With `Cone` on both rosters
(`reduce::boolean_arm_exists`, `revert_arm_exists`) and `sector_face`'s
cone arm let through, every op refuses
`CurvedBooleanUnsupported { kind: Cone }` one site further on, at
`vtxfac` where the box's vertex lands on the cone face.
`face_outward_normal_at` has no cone arm, and its `Ok(None)` is that
site's refusal.

With a plane×cone germ lane added beside the cylinder's, the next site
behind that one is the join's germ-pair dispatch: `bool_connect`'s
`no_arm`, `CurvedBooleanUnsupported`. Past it,
`chord_join::bool_planar_chord_spec` would refuse the planar side's
chord as `SectionInvariant` ("boolean planar-side germ partner is
neither a cylinder nor a sphere (arm not wired)").

The ring lane behind all of these is wired: a cone face winds its
island and re-homes its rings by `chord_join::path_island_winding` and
`path_ring_side`. The sweep row there holds the crossings to the closed
form and pins every op at the pair gate. It goes red when this item, with
`VERBS-CONE`, opens the gate, and then becomes the lane's first public
row: the six ops at tiers 3 and 3′, against a slice-integral volume.
