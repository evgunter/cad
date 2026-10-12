---
id: apply-cut-ins-walks-its-loops-twice-and-overloads-its-predicate-names
kind: issue
title: apply_cut_ins walks the face's loops twice, orders two ends on one arc by hand, and names different questions with one predicate
status: open
opened: 2026-10-06
priority: P4
cost: M
---

Found by the dual review of PR 4044 (`analysis/reach-dual/4044-r1`,
style Q1/Q7; `-r2`, style Q1, Q2, Q7).

## What

`boolean::ops`'s `apply_cut_ins` (ops.rs ~3661, about 400 lines in one
function):

- **Two loop walks** with the same `linked` / `LoopBoundary::Cycle` /
  `loop_walk` scaffold (ops.rs ~3814 for the hits, ~3992 for the ends'
  half-edges). One returns on a lone-vertex loop and the other
  `continue`s, and `scripts/gates/loop-boundary-discards.sh` needs an
  entry for each.
- **Two ends on one arc** are ordered by a `below_first` boolean (ops.rs
  ~3975) and a mutable `end` closure (ops.rs ~3963) that splits the edge
  farther along the arc's parameter first, so the nearer end still lies
  on the arc's key. A type holding "the split order of the hits on one
  arc" would keep that invariant out of the call sequence. The local
  `Hit` struct (ops.rs ~3806) is declared inside the function.
- **Overloaded predicate names**: `bool_sphere_cut_span` names the arc's
  direction (ops.rs ~3876), a root inside the span (~3881-3882) and the
  two-ends order (~3978); `bool_sphere_cut_half` names the circle's
  crossings on the half-plane (~3731) and a boundary hit on it (~3893);
  `bool_sphere_cut_meridian` names the centre's meridian and the
  `u_ref` fallback's (~3704, ~3706). An escalation's diagnostic cannot
  tell them apart.

## Asked

One loop-walk helper both passes use; a type for the two-ends-on-one-arc
order; a distinct predicate name per question, with the meter registry
updated.
