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

## 2026-10-11 — an edge-face span the op now mints, dropped on carry (FLUSH, PR 4737)

PR 4737 makes the boolean record a line edge's span in a plane face
(`boolean::ops::record_spans`). `ContactRecords::carried` drops that
record like any edge–face one, so the next op never sees it. The
review probe battery `sweep` `review_4737_span_probes::review_4737_span_battery`
(ignored, release) unites a triangle prism with a brick whose face
`y = 0.5` holds the prism's apex edge, then cuts the union with a plate
across the contact, carrying its records:

```
CARRY ang=0 whole_in_face plate I x => records=[0, 0, 0, 0, 0, 0]
  t3p=ERR [UnrecordedContact VertexOnEdge (0.0, 0.5, 1.123),
           UnrecordedContact VertexOnEdge (0.0, 0.5, 0.8),
           UnrecordedContact EdgeFaceOverlap (0.0, 0.5, 0.9615), ...]
```

18 of the battery's `plate I x` lines refuse 3′ this way, at all three
turns. The lines are identical on PR 4737's pre-fix head (`dfe5ea10`):
the carry drops the face span, and the cut's new ends on the apex edge
have nothing to inherit. Carrying an edge–face record through the cut's
lineage (onto the pieces whose interiors still meet the face, as D1
rules for an edge–edge one) is this row's owed step for that kind.
