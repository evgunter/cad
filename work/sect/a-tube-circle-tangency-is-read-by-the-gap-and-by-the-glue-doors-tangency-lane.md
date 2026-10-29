---
id: a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane
kind: issue
title: A tube-circle tangency between two torus faces is read twice: the critical gap of the pair's pose and the glue door's tangency lane along a shared rim
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-10-29
branch: sect/torus-decided-once
---


Found by `sect-toruspose` (branch `sect/torus-pose`) sweeping for every
site that decides a torus pair's tangency.

## Measured

Two torus faces sharing a rim tube circle (two pipe bends joined G1) are
read tangent along it by the glue door's tangency lane
(`crates/topo/src/boolean/contact_verify.rs` `tangency_reading`, the jet
schedule along the shared locus), and the same pair's pose decides its
critical gap (`crates/geom-brep/src/pair_section/gap.rs`
`critical_gap`, the least distance between the cores read against
`|r₁ − r₂|`). The first reads the locus the operation holds, the second
the whole pose, so they ask different questions; but where both run on
one pair nothing checks that they agree, and in band they can disagree
(one tangent, the other escalated).

## What a fix has to supply

Either the glue door reading the pair's decided gap where the section
value builds the pair, or a stated reason why a tangency along a held
rim and the pose's tangency are separate decisions.

## Closed — two questions, argued at both sites and pinned (#sect/torus-decided-once)

The gap decides whether the pair is tangent and where; the glue door's
touch decides that the rim the faces hold lies within ε of both surfaces
at every sample, the residual every carrier owes however its pair was
decided (C2's limbs, C7's first equation). The first does not imply the
second: measured on two G1 bends (cores `R = 2`, `1.5`, planes `0.7`
rad apart, tubes `0.5`), the second's tube `0.6ε` wider and moved
`0.6ε`, the gap reads Zero and the rim stands `1.2ε` off the second
torus on its far side, so the touch escalates (`contact_tangent_gap`);
an exact distance from the rim's points to the torus measures the same
`1.2ε`. Neither reading is read as the other's, and no implication
between them is claimed. The argument is in `tangency_reading`'s docs
(the gap's module docs say only what the gap decides); the row is
`contact_verify::tests::a_rim_the_pose_reads_tangent_can_still_escalate_along_the_rim`,
and `tori_coincide::a_torus_tangent_inside_the_tube_along_a_tube_circle_refuses_at_the_gap`
pins a tube-circle tangency the pose refuses on its gap.

Off the band an operation does not see them disagree today: along a
torus × torus tube circle each torus's transverse curvature is
`cos(φ − β)/(R + r·cos(φ − β))` of the tube angle, whose difference
keeps an oscillation larger than its mean, so `κ_rel` crosses zero and
the glue door's undeclared reading escalates on the second order
wherever its touch passes
(`mate7a_torus_rest::the_g1_tube_chain_refuses_at_the_glue_door`).
