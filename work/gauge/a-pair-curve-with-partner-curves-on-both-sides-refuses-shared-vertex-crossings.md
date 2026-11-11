---
id: a-pair-curve-with-partner-curves-on-both-sides-refuses-shared-vertex-crossings
kind: issue
title: A pair whose curve at a shared vertex has partner curves on both of its sides refuses SharedVertexCrossings: no witness exists, and the known construction (one non-crossing chord family per vertex) is unbuilt
status: open
opened: 2026-11-11
priority: P3
cost: H
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---


Filed by GAUGE's shared-vertex pairing build (PR 6832, lane
`gauge-svc`, 2026-11-11), as the decision's typed limit.

## What

At a vertex several crossing pairs share, each pair starts its pairing
where its runs lie on the side the op keeps, and a blocked pair takes
its other start. The pairs at one vertex choose together
(`insert::choose_starts`, `crates/topo/src/boolean/insert.rs` near
line 1298). Where no choice leaves every run clear of the other pairs'
cuts, `insert::reconcile_shared` refuses
`BooleanError::SharedVertexCrossings`, typed (`insert.rs` near line
1307; the variant's docs in `crates/topo/src/boolean/mod.rs` near
line 3577). A vertex holding more than `insert::CHOICE_PAIRS` (12)
crossing pairs refuses the same way.

No choice is clear only where a pair's curve has partner curves on
both of its sides, so that both of its starts hold another pair's
crossings. Two shapes would do it:
- three nested cones of one operand at an apex, for example a pyramid
  less a nested pyramid less a third nested inside that, all at one
  apex;
- a shell beside a separate piece.

Neither has a witness. No line of `four_pairs_battery` refuses, and
nothing in the topo, sweep or editor-core suites does. Family 1's
nested pyramid builds: its two cones leave one start clear.

## The known construction

Designer B's round-2 report on fork `shared-vertex-pairing` (fork-log
row 261) gives it:
- pair each pair along the kept arcs of the partner solid, the one
  whose vertex no other pair shares;
- place the shared vertex's null edges as one non-crossing family
  across all its pairs, a run inside another pair's run minting at
  that run's copy.

B argues this never crosses. The partner vertices' links are disjoint
curves, and every partner keeps the same side of the shared link in
one op, so their kept arcs are disjoint curves inside one disk of it:
a non-crossing chord diagram. Every null edge then lies on one cone of
the result.

## Owed

- A witness: three nested cones at an apex, or a shell beside a piece,
  each refusing `SharedVertexCrossings` today.
- The nesting placement across pairs, built against it.
