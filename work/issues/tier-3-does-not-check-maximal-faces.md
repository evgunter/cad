---
id: tier-3-does-not-check-maximal-faces
kind: issue
title: Tier 3 does not check maximal faces; the boolean door does, so a split's coplanar artifact is valid at rest and refused as an operand
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

Filed from PR 4998's reviews (the first review's side point, restated in the second).

D1 asks for maximal faces. The boolean door enforces that on its operands (`gate_maximal_faces`, `crates/topo/src/boolean/reduce.rs:773`). Tier 3 (`topo::validate_geometric`) has no such check. So a body can pass `AtRestBody::validate` and then be refused `NonMaximalFaces` when used as an operand.

**Witness.** `sweep/tests/split_edge_on_faces_poses.rs`, `a_smooth_edge_at_the_zero_bands_edge_states_the_pair_and_builds`:

- Each `PAIR_POSES` split builds a piece that tier 3 calls valid.
- Unioned with a far brick, the unmerged piece refuses `NonMaximalFaces`. The cause is that the section face and the flush prism face it meets are left unmerged.
- After `merge_coplanar_faces` the piece is a legal operand.

That gap is F7's caller-merge obligation: a split leaves its coplanar artifacts for the caller's opt-in merge (the module docs of `topo::splitting::finish`). The obligation is documented, but nothing at rest checks it.

## What it needs

One of these:

- Tier 3 gains a maximal-faces check, at least as a reading the at-rest gate can report. Every finisher that leaves coplanar artifacts on purpose would then have to say so.
- Or D1's maximal-faces clause states that maximality is an operand gate rather than an at-rest invariant, so that the two doors' answers are the documented split.

Either way, the at-rest contract and the boolean's gate should say the same thing.
