---
id: a-fold-step-reads-a-vertex-against-a-carrier-the-pass-never-judged
kind: issue
title: A union fold step reads an accumulation vertex against the joining member's carrier, a pair the pass never judged, so the union refuses in some member orders
status: open
opened: 2026-10-26
---

Found by FLUSH's glue-verdict designer lane (round 3, probes on
`flush/glue-verdict-design-015xnd`, `crates/topo/tests/honest_fold_probes.rs`,
`tilt_along_the_row_is_order_dependent_in_substance`), measured with the
kernel alone: the pass's verdicts handed back to each step as the fold
does (now inside `topo::nary_boolean`).

## What

PR 4323's rule is that a union's fold step decides no carrier pair the
pass did not: each step is handed the pass's verdicts, and the n-ary
door (`crates/topo/src/boolean/nary.rs`, `restated_rows`) debug-asserts
that every face-pair row a step records restates one. But a step still reads
vertex-against-carrier questions by value (`bool_vertex_face_side`,
`Coincide::VertexOnFace`), over cells of members whose boxes never met
and whom the pass therefore never judged, and a reading in band
escalates the step. Which member's vertex is read against which
member's carrier follows the fold order, so the union refuses in some
orders and builds in others, which `[operations.operators]` calls a
kernel defect.

## Measured

Three plates in a row: P1 `[-9,1]×[0,1]×[0,1]`, P2 `[1,2]×…`, P3
`[2,3]×…` tilted by θ = 0.4ε about the line `x = 2, z = 1` (across the
row, so its top is within θ·1 of the flat plane and the flat plane's far
end at `x = -9` is θ·11 = 4.4ε off P3's). ε = 1e-9, band (1e-9, 1e-8).

- The pass: (P2,P3) five verdicts, all Zero (margins ≤ 4.9e-10);
  (P1,P2) five verdicts at 0.0; (P1,P3) boxes apart, not judged.
- Fold with the pass's verdicts given, order [P2,P3,P1]: builds.
- Order [P3,P2,P1]: step 2 refuses
  `Escalated { Coincidence(VertexOnFace, Spent(Continuation)),
  bool_vertex_face_side 4.4e-9 }`: P1's far vertex read against the
  accumulation's kept top description (P3's, tilted).
- Order [P1,P2,P3]: step 2 refuses the same way: the accumulation's far
  vertex (P1's) read against P3's top carrier, a plane whose face lies
  eleven units away. The given Continuation pair is spent and does not
  settle it.
- An honest fold with no given pairs refuses in the same two orders and
  builds in the third.

So the pass makes the carrier-pair verdicts order-free, but the step's
vertex readings are not covered by it, and here the one that refuses is
about a member pair the pass never looked at.

## Owed

Either the step reads a vertex only against carriers of faces whose
extent it is near (the question asked of a far vertex against an
infinite carrier is moot for the result, whose fused face keeps one
description), or the pass judges and hands over the vertex readings too,
or a reading the pass did not back is a finding rather than a refusal.
Whichever, the permutation tests (`union_member_orders_decide_alike`)
should gain this fixture. Not a lever-arm issue: the margin is the plain
point–plane distance; the LEVER row
`lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces` is the
separate over-metering the same probes showed on a tilt along the row.

## The kernel door (FLUSH, 2026-10-26)

Since `flush/nary-union-door` the pass and the fold are one kernel
operation (`topo::nary_boolean`), which holds every member body, every
judgement and each step's lineage. So the second and third answers
above are local to `crates/topo/src/boolean/nary.rs`: the door can judge
a member pair whose boxes are apart only when a step reads across it,
or tell a step's reading the pass did not back from one it did, with no
channel between two crates. Nothing here is fixed by that move.
