---
id: a-run-holding-another-whole-is-spelled-three-ways-in-insert
kind: issue
title: insert.rs spells 'run m holds run k whole' three ways: b_runs' interval holds, holds_whole, and both cuts under held_cut
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-vertex-orbits-walk-order-and-run-rule-are-spelled-several-times]
---


## What

Found by PR 4249's dual review (r1 Q1, r2 Q1). `crates/topo/src/boolean/insert.rs`
asks "does run m hold run k whole" three ways:
- `b_runs`' `holds` reads walk positions: whether an interval of B's walk
  order holds a pair's first germ;
- `holds_whole` reads two strut segments in one physical sector, and
  counts an end along one direction as inside;
- `sibling_holds` counts how many of a run's two cuts `held_cut` places
  inside the other run. `held_cut` asks `tied_held` at an end along one
  direction and leaves nested struts to `holds_whole`.

`holds_whole`'s own comment already records one way it diverges from
`held_cut`.

## Why not folded in PR 4249

The three read different data (walk positions against sector geometry)
and settle ties differently, so one helper means choosing one tie rule
for all three. That is not a contained change. Each reader's tie
behaviour is pinned by rows today. A unification needs those rows as its
oracle.

## Since (branch `join/nested-pairing-shared-vertex`)

`sibling_holds` and `sibling_holders` are gone. A plan's holders are
now read one way, at the plan and again after the reconcile: from the
runs' arcs of their walk (`insert::arc_holders`, the reading `b_runs`
had). Two spellings remain: walk positions within one plan
(`arc_holders`), and sector geometry across plans (`holds_whole`, and
`held_cut` in the reconcile).
