---
id: boolean-declares-no-touching-between-copies-of-one-operand-vertex
kind: issue
title: The boolean emits contact records only for cross-operand vertex pairs; its own copies of one operand vertex that touch in a result are declared by nobody
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

`crates/topo/src/boolean/reduce.rs` pushes `VvContact`s for cross-operand pairs and carried pairs only. Where the boolean's `mev_null` copies one operand vertex and two copies land touching in the result (the same shape split's pinch halves have), no record covers them, so the result can refuse at rest as an undeclared `VertexVertex`. Whether it is reached in the corpus is unmeasured. Its fix shape depends on the open pinch-contacts question on TQUERY (records vs a shared point).

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.
