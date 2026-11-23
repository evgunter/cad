---
id: merge-door-bend-ending-takes-the-first-order-lever
kind: issue
title: merge_coplanar_faces words an undecided Bend at a kept boundary with the first-order lever and no sized ending, where the boolean ends the same decision as sized SEAM_JET
status: open
priority: P3
opened: 2026-10-09
---

## Finding (ENCL review of PR 4411)

`topo::merge_faces::merge_coplanar_faces` re-describes a kept face's
boundary edges through `boolean::describe_edges`, which asks
`must_carry_reading` (`crates/topo/src/boolean/ops.rs`). An in-band
second-order station comes back as `DihedralReading::Bend` and is
refused as `MergeCoplanarError::KeptBoundaryUndecided`
(`crates/topo/src/merge_faces.rs`). That variant's Display ends the
`Bend` arm with `CROSS_OR_SMOOTH` ("move the geometry so the faces at
that edge clearly cross or are clearly smooth"). That is the
first-order reading's lever, and it has no sized ending. The boolean
ends the same decision (`BooleanDecision::SeamJet`,
`crates/topo/src/boolean/refusal_routes.rs`) as the sized `SEAM_JET`
ending. By D4 ¶1 a decision has ONE recourse, so the merge door's
`Bend` arm should take the bend decision's own lever and its sized
ending, the way the `Lever(Arm)` arm already takes `DIHEDRAL_ARM`'s.

**Reachability.** A merge door's kept boundary reaches `Bend` only on a
definitely-smooth in-band edge. In lane, that is a kept smooth seam
that is near-osculating. Out of lane (the arm PR 4411 opened), no
constructor reaches it today: a cone operand is refused at the
boolean's pair gate before any seam is described.

