---
id: a-boolean-drops-its-operands-curve-and-patch-records
kind: issue
title: A boolean drops its operands' curve and patch records: only vertex and edge granularity carries
status: open
opened: 2026-10-11
priority: P2
cost: M
---

## What

Stage 4 F carries an operand's own records into every pair boolean and
every union fold step (`a-boolean-drops-its-operands-own-contact-records`,
closed). It carries the vertex and edge granularities alone:
`ContactRecords::carried()` (crates/topo/src/boolean/mod.rs:851) builds
`OperandRecords` from `vv`, the vertex-on-face rows, `ve` and `ee`, and
its doc says a face-granularity record "has no carried kind and stays
here". The op remaps only the curves its own reduction discovered
(crates/topo/src/boolean/ops.rs:3715).

So a curve or patch record an operand holds — a tangent
`CurveContact` a union's fold step k decided, or one a pair boolean's
operand carries — does not reach the result once that body re-enters an
op: after step k + 1 the union's records lack it.

Found by stage 4 F's review (M3). No witness is built yet: the corpus
scenes that passed 3′ on F's carry hold vertex and edge contacts only.

## Owed

- A witness: a union of three members where two touch along a tangent
  curve and the third folds in after them, and its 3′ verdict.
- The carried kinds for curve and patch records, under `remap_carried`'s
  drop rules (a curve record survives where its curve's entities do).
