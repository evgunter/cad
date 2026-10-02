---
id: boolean-declares-no-touching-between-copies-of-one-operand-vertex
kind: issue
title: The boolean emits contact records only for cross-operand vertex pairs; its own copies of one operand vertex that touch in a result are declared by nobody
status: closed
opened: 2026-10-02
closed: 2026-10-02
priority: P2
cost: M
refs: [3856]
---


## What

`crates/topo/src/boolean/reduce.rs` pushes `VvContact`s for cross-operand pairs and carried pairs only. Where the boolean's `mev_null` copies one operand vertex and two copies land touching in the result (the same shape split's pinch halves have), no record covers them, so the result can refuse at rest as an undeclared `VertexVertex`. Whether it is reached in the corpus is unmeasured. Its fix shape depends on the open pinch-contacts question on TQUERY (records vs a shared point).

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Closed (2026-10-02, branch `tquery/split-pinch-shared-point`)

Closed by construction and measured. Ev ruled the shared point (PR
3813), and the build gives every `mev_null` copy the original vertex's
`PointKey`. The boolean mints its copies through that door
(`boolean/insert.rs`, `boolean/vtxfac.rs`). So copies of one operand
vertex that touch in a result sit on one point, and the census clears
them as rung-1 structural sharing (`census.rs`, `Geo::same_point`). No
record is needed.

Measured with the release suites of topo, sweep and editor-core,
instrumented locally and not committed. The probe ran
`gate_at_rest_kept` + `gate_at_rest_declared` on every `Body` result of
`boolean_op_with`, against that result's own `ContactRecords`, and
counted vertex pairs on one point:
- 22,138 results on the branch; none holds two vertices on one point.
  The copies are zipped or killed before rest in every corpus result,
  so the case this item names is never reached.
- Refusals per test are the same on main and on the branch, within
  interleaved-log noise of one line. No result flips either way.

