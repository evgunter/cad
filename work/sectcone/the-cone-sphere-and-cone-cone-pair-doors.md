---
id: the-cone-sphere-and-cone-cone-pair-doors
kind: unit
title: Cone × sphere and cone × cone in general pose have their rung-3 doors: poses on the section value with tangency and apex margins, the ssi doors, route flipped, the join carried
status: closed
opened: 2026-11-22
priority: P0
cost: H
parent: cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door
branch: sectcone/cone-sphere-cone-doors
closed: 2026-11-24
pr: 7457
---


**Review tier: dual** (two Opus reviewers; Ev 2026-11-21). The poses' tangency decisions and apex margins are certified logic. Difficulty letter: H.

## Spec

`crates/geom-brep/src/intersect.rs` `route(Cone, Sphere)` and `route(Cone, Cone)` answer `implemented: false`. Both halves of the ℝ³ certificate take the cone: the tube arm (PR 7358) and limb 2 at the search (PR 7350). PR 7357 classified cone × cone, and cone × oblique cylinder, on the section value as `Arm::ConeRuled`, but no march carries it (`pair_section::uncarried` → `NoCarrier::ConeUnmarched`). The pattern end to end is sphere × torus:
- the pose: `SphereTorusPose`;
- the door: `ssi::sphere_torus_ssi`;
- the carried value: `PairPose::SphereTorus`;
- the join: topo's `rides_carried` arm.

The sibling `the-cone-torus-pair-door` (branch `sectcone/cone-torus-door`) is in flight on the same pattern. Read its branch, share its door helpers rather than minting a second copy, and merge main often.

1. **Cone × cone:**
   - PR 7357's `PairPose::ConeCone` / `RuledCone` sweep already decides tangency (crossing nodes and touches) and the apex margin.
   - Give the arm its march: components are `ClassCarrier::General`, marched by `PairSection::carry`, with the C5 nodes as chain ends.
   - The common-apex pose stays the rulings' closed form, decided at the gate.
2. **Cone × sphere:**
   - Build a pose on `pair_section`, extending `RuledCone` to a sphere partner if that's the natural fit. It decides in closed form where one exists:
     - the tangency;
     - the apex on the sphere: C5's node clause (`[ev]` PR 6432), giving node, touch or refusal, never R-tan.
   - The coaxial pose stays `cone_sphere_section`.
   - The pose is an instance of the section object, never a per-pair frame.
3. **Doors:**
   - Add `ssi::cone_cone_ssi` and `ssi::cone_sphere_ssi` (or one ruled-cone door), following `sphere_torus_ssi`: a tangent pose refuses before any rung.
   - Flip both `route` rows to `implemented: true` and update their notes.
   - `uncarried`'s cone arms then carry. Decide cone × oblique cylinder explicitly: if it marches by the same door, take it; if not, say why and file it.
4. **Join:** topo's `rides_carried` carries both poses, and the section table holds them.
5. **Witnesses:**
   - a cone unioned with a ball, and a cone with another cone, in general pose, at the default ε, 1e-6 and 1e-12, every op in both orders;
   - pose rows against a traced oracle: crossing, tangent (node and touch), apex node, apex touch, clear;
   - each must be able to go red.

Out of scope: the cone × torus door (sibling), and U2 / U3 (`the-section-certificate-reads-the-cone-pairs-on-the-value`, `the-join-reads-the-cone-pairs-on-the-value`), which move topo's section certificate and its frame-lane copies. Announce the seam with U2's lane where `section_cert` or the join's cone arms are both touched.

Territory: `ssi*` is SSIARITH/SSIEDGE's. Run `python3 scripts/work.py territory --base origin/main` and announce the seam.


## Closed (2026-11-24, PR 7457)

Cone × cone, cone × oblique cylinder and cone × sphere are classified on the
section value (`ConeRuledPose`, a sphere's tangency in closed form) and
marched from the count's witnesses (`ssi::march_cone_ruled`,
`ssi::cone_ruled_ssi`); `route` is flipped, `serves` takes the pairs and the
join carries them. Left open, each on its own row:
`the-r3-march-ends-no-branch-at-a-cone-node` (node arcs refuse the carry),
`a-cone-against-a-ball-or-a-cone-refuses-at-the-cone-face` (the boolean
witness), `route-pose-reads-oblique-cone-cylinder-on-its-closed-form` and
`the-cone-search-fuzz-reads-near-coincident-coaxial-apexes-intractable`.
The section certificate still reads cone × sphere on topo's own copy; that
move is on `the-section-certificate-reads-the-cone-pairs-on-the-value`'s
addendum.
