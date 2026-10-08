---
id: interference-at-rest-is-a-finding
kind: issue
title: D10 stage 5 PR B: interference between world copies is its own finding, quiet under a one-sided gap assertion at its site
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [the-product-is-an-explicit-list, measure-is-an-operation, select-defines-face-and-edge-variables]
---


INTENT stage 5, PR B. Spec: `docs/INTENT-STAGE5-SPEC.md` §3.

D10: at rest, "interference is a finding of its own; neither refuses
where the census has a lane". Today the census's containment arm
(`crates/topo/src/census.rs:5634`) pushes
`ValidationError::InstanceInterference`, and A5's gate attributes it
`Unattributed` (`crates/editor-core/src/assembly.rs:1791`) and refuses
`AssemblyError::AtRest`. A transverse pierce between two copies refuses
as `UndeclaredContact { EdgeFacePierce }`.

B partitions those verdicts out of the gate's refusal into an
`InterferenceFinding` per overlap. The site is per FORK-S5-2 (recommended:
a connected component of the two copies' intersection, named by the
faces bounding it). B lands the quieting rule's one home
(`checks/at_rest.rs`) with its interference half: a holding `Gap`
assertion over the finding's faces of the two world copies, whose
admitted set is negative (FORK-S5-1, FORK-S5-3). The kernel is
unchanged except for one typed `SameSide` field. ASSEMBLY A5
*Interference.*, topo C6's invariant and MATE-4B's "EdgeFacePierce stays
categorical" are re-worded at the at-rest door.

It needs no stage-3 or stage-4 work. It needs stage 2's world copies (C),
single-primitive measures (D) and selections (E) to state a site.

Design forks open: FORK-S5-1, FORK-S5-2 and FORK-S5-3 (spec §11).
