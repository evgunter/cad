---
id: maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip
kind: issue
title: The maximal-faces gate's curved arm cannot tell a declared continuation's recorded curved skip from an unlicensed cosurface adjacency
status: parked
opened: 2026-10-01
priority: P2
cost: M
design: true
refs: [cosurface-disjoint-curved-walls-refuse, a-union-glues-same-sense-cosurface-walls-without-merging-them]
blocked_on: [3990]
---

Found by the unit that built the continuation ruling
(`cosurface-disjoint-curved-walls-refuse`, PR 3613), whose brief asked
for the maximal-faces gate (`gate_maximal_faces`,
`crates/topo/src/boolean/reduce.rs`) to gain a curved arm "so an
unlicensed cosurface adjacency never ships from any op". The unit
stopped on that part and did not build it, because two ruled sentences
cannot both hold under any curved arm the gate can state today.

## The conflict, measured

- DESIGN's merge bullet: a declared continuation's curved group is the
  merge's recorded skip, "recorded and shipped", and "every boolean
  output is a legal boolean operand".
- The rounded two-plate stack (`crates/sweep/tests/reach_continuation.rs`,
  `the_stack_is_a_legal_operand`), every wall pair declared a
  continuation, unions to 14 faces with four `SkippedMerge` records:
  each corner fillet stays two faces, P's and Q's, on two surface keys
  of one carrier, adjacent across the z = 1 arc. That body unions
  again, with a clear plate and with a third stacked plate, because the
  gate's curved branch accepts every curved adjacency it does not
  find to be same-key planar.
- A curved arm built as the planar arm's twin (different surface keys,
  carrier ladder with sources, numeric coincidence ⇒ refuse) refuses
  exactly that body: the two fillet faces come from different recipe
  sources and their carriers coincide numerically. The licence that
  made the skip legal is the union's declaration, recorded on the
  outcome (`BooleanNaming::merge_skipped`), not on the body, so the
  next op's gate cannot see it.

## What the unit did instead

Booleans no longer create an unlicensed cosurface adjacency: an
undeclared continuation refuses at the reduction on every carrier kind
(`refuse_undeclared_continuations`). What the curved arm would add is
protection against an operand built by some OTHER op (a sweep, an
import, a hand-built body) carrying two coincident curved faces on
different keys.

## Options (not weighed)

1. The union re-keys a declared continuation's skipped curved pair onto
   one surface (and source), so the shipped form is the same-key curved
   adjacency the gate already calls canonical, and a curved arm can
   refuse every different-key coincident adjacency.
2. The body carries the licence (a recorded continuation the gate
   reads), so the arm refuses only unlicensed adjacencies.
3. The merge gains its curved rung, so no curved continuation ships
   unmerged and the arm has nothing licensed to tolerate.
