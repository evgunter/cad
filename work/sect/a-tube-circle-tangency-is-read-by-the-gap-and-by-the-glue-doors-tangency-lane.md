---
id: a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane
kind: issue
title: A tube-circle tangency between two torus faces is read twice: the critical gap of the pair's pose and the glue door's tangency lane along a shared rim
status: open
opened: 2026-10-28
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
