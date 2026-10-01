---
id: a-stranded-operand-reaches-the-classification-invariant
kind: issue
title: topo: a stranded operand crossed by a brick ends on ClassificationInvariant (contfp ray schedule exhausted), not a typed refusal
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted]
---



## What (measured)

A unit prism whose top face is split on its diagonal, one half
re-described on the parallel plane `1000 ε` above (so that half's own
edges and vertices lie `1000 ε` off its surface: a stranded operand,
which no valid body is), crossed by the brick
`[0.3, 2] × [0.2, 0.7] × [0.5, 1.5]`, ends every public Boolean (union,
subtract, intersect) on
`ClassificationInvariant { what: "contfp ray schedule exhausted" }`.

- The raise is `reduce.rs`'s `esc`, mapping `ContainError::RayExhausted`
  to the invariant (`crates/topo/src/boolean/reduce.rs:2398`).
- The fixture is `top_split_redescribed`
  (`crates/topo/src/boolean/refusal_routes.rs:3181`), which strands the
  half through `set_face_surface_stranding_for_tests` (`:3214`).
- The row that pins it is
  `a_stranded_split_top_crossed_by_a_brick_reaches_the_classification_invariant`
  (`crates/topo/src/boolean/offer_rows.rs:1473`). It reaches the same
  invariant at ε = 1e-9, 1e-6 and 1e-12, so it is the stranded body's,
  not a tolerance's.

## Why it matters

An operand that violates its own face-edge incidence should end on a
typed operand refusal (the body is not valid), not on a kernel
invariant whose text says a ray schedule ran out. A kernel invariant is
the ending for a kernel bug; here the input is at fault.

## Provenance

Found by PR 3513's coincv5 verifier (NF-2): the `CoplanarNeighbours`
offer cases rested on this fixture, and executing their offers through
a crossing brick reached this invariant at every ε. Those cases now run
on valid bodies; the stranded fixture is kept only as the pinning row
above. Not fixed there.
