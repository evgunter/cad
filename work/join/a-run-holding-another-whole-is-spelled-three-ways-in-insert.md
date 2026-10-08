---
id: a-run-holding-another-whole-is-spelled-three-ways-in-insert
kind: issue
title: insert.rs spells 'run m holds run k whole' three ways: b_runs' interval holds, holds_whole, and both cuts under held_cut
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-vertex-orbits-walk-order-and-run-rule-are-spelled-several-times]
branch: join/insert-one-walk-order
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

## Built (branch `join/insert-one-walk-order`)

"Holds whole" has one reading, `insert::arc_holds(before, outer,
inner)`. It reports which of `inner`'s ends lie on `outer`'s arc
(`on_arc`) under one total order, with one tie rule, stated at its
definition: an end along one of `outer`'s own ends counts inside.
- `arc_holders` reads it over walk positions (`<`). A plan's positions
  are distinct, so the tie rule never applies there.
- `holds_whole` reads it over strut segments' cuts (`walks_before`).
  It keeps its one-physical-sector gate and its one-arc case (both
  ends tied), which it settles by the pieces' codes.

`held_cut` asks a different question, whether a run holds one cut. It
reads the same arc (`on_arc`) and still settles a tie with
`tied_held`. Neither tie behaviour moved:
- the former origin readings agree with the fixed one, case by case
  (the PR body has the table);
- `holds_whole`'s former test checked only `ilo` against `lo` and `ihi`
  against `hi`, and the other two comparisons follow from both
  segments being ordered in one physical sector.

## Measured (release, main 047d10d5 against branch head 8b03369e)

0 lines moved, across 89 battery runs:

| battery | lines |
|---|---|
| `pierce_runs_battery` | 4 537 |
| `pinch_runs_battery` | 3 025 |
| `corner_pairs_battery` | 16 381 |
| `join1_r1_reflex_battery` | 1 153 |
| `j3r2_r1_reflex_battery` | 1 153 |
| `rc_wide_battery`, all 84 shards | 40 320 `RCW` lines |

Every run exited 0 on both sides. The whole workspace suite (debug,
12 951 tests) passed on head except one wall-clock row. That row reads
the same time on main and is filed as
`a-names-alike-bound-sits-at-the-debug-builds-own-time`.
