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

Filed from PR 4998's reviews (the first review's side point, restated in
the second and third).

D1 asks for maximal faces. The boolean door enforces that on its operands
(`gate_maximal_faces`, `crates/topo/src/boolean/reduce.rs:773`). Tier 3
(`topo::validate_geometric`) has no such check. So a body can pass
`AtRestBody::validate` and then be refused `NonMaximalFaces` when it is
used as an operand.

**Where it shows.** A split leaves its coplanar artifacts for the
caller's opt-in merge: F7, in the module docs of
`topo::splitting::finish`. A section face flush with a face of the body
is one such artifact. The piece carrying it is tier-3 valid, refuses as
a boolean operand until `merge_coplanar_faces` runs, and is a legal
operand after it.

PR 4998's first fix pass pinned this on its `PAIR_POSES` rows. Those
rows went away in its third pass, because their section and prism faces
now read as one carrier and the poses refuse at the split, as on main.
So no row pins the gap today.

## What it needs

One of the following, so that the at-rest contract and the boolean's
gate say the same thing:

- **Check it at rest.** Tier 3 gains a maximal-faces check, at least as
  a reading the at-rest gate can report. Every finisher that leaves
  coplanar artifacts on purpose then says so.
- **State the split.** D1's maximal-faces clause says that maximality is
  an operand gate rather than an at-rest invariant, so the two doors'
  answers are the documented split.
