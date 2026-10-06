---
id: an-l-prism-top-edge-exactly-in-the-cube-face-plane-ships-an-undeclared-contact
kind: issue
title: A cube ∪/∖ L-prism whose top edge lies exactly in the cube's face plane ships a body with an undeclared contact (tier 3′ red, pre-existing)
status: open
opened: 2026-10-05
priority: P1
cost: M
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired, d10-one-way-to-say-intent-is-unbuilt]
---



## What

Found by `join/pinch-uncrossed-residue` re-running PR 4038's review r2
battery (`crates/sweep/examples/r2_pinch_probes.rs` on
`join/pierce-pinch-families-review-r2`) on main `4cf3f8b9`. Identical
on PR 4038's frozen head (`review-r2/H-cube.txt.gz`), so it is older
than this lane and unmoved by it.

`Ltop fib0 edge psi=1.9`: the L-prism's reflex top corner `(1, 1, 1)`
on a cube edge, the cube along Fibonacci direction 0 of 120, so
`m = (0.1288, 0, 0.9917)` has no `y` part and the prism's top edge
`(1, 1, 1)–(1, 2, 1)` lies exactly in the cube's near face plane. Three
runs build with the oracle volume, tier 2, the certificate and a legal
operand, and fail tier 3′ (`validate_pseudomanifold` against the
result's own contacts), definite, not escalated:

- prism ∪ cube and cube ∪ prism: `UndeclaredContact { VertexOnFace }`
  at `(1, 2, 1)` and `UndeclaredContact { EdgeFaceOverlap }` at
  `(1, 1.5, 1)`;
- prism ∖ cube: `StaleContactDeclaration { VertexOnFace }`.

An operand edge lying in the partner's face is an undeclared
coincidence (D10's ground, held): the op should refuse it or carry its
contact, and here it does neither.

## The shape to give

Diagnose which lane let the edge-in-face coincidence through without a
refusal or a contact record, beside
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`.
Repro: `R2P_T3=1 R2P_PICK=fib0 r2_pinch_probes cube Ltop`.
