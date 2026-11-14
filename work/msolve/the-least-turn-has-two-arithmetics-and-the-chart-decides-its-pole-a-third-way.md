---
id: the-least-turn-has-two-arithmetics-and-the-chart-decides-its-pole-a-third-way
kind: issue
title: The least turn has two arithmetics (editor-core LeastTurn, geom-core RigidMotion::turned_between) and the value chart decides its pole a third way
status: open
opened: 2026-11-03
priority: P1
cost: M
---


## What

Found by FRAME's designer pair on `a-frame-across-a-direction-is-a-written-frame-turned-onto-it` (2026-11-03). Neither designer verified it by execution.

- `geom_core::LeastTurn` (`crates/geom-core/src/linalg/rigid_motion.rs`, moved down from editor-core's `mate/turn.rs` by PR 6962) is the mate chart's and the snap's least turn: the Rodrigues form `c·I + [k]× + kkᵀ/(1+c)`, `k = a × (a + b)` where `c` reads negative. `RigidMotion::least_turn` mints from it. Editor-core keeps only its pole read, `mate::turn::at_pole`, by `one_plus_c.quoted() <= 0`.
- `geom_core::linalg::rigid_motion` `RigidMotion::turned_between` is the same map in a quaternion form over `|h|²/2`. Its pole is decided under the linear band (`TurnError::{Opposed, Undecided}`).
- `mate/value.rs`'s chart decides the same pole a third way, through `mate_chart_pole` on the chord `|n + d|`.

These are three spellings of one map and its pole. PR 6962 does the first half of the ask: the mate/snap turn's arithmetic lives below editor-core, in `geom_core::linalg`, and its near-opposed axis is `turned_between`'s spelling `a × (a + b)`. The two forms (Rodrigues and quaternion) and the two pole decisions are still two; whether they become one, and which bits move (mate's or sweep's), is fork 270 (PR 6964, waiting on Ev). The chart and the viewer's snap should read the same arithmetic, so that what a gesture writes and what the chart reads back agree by construction. If the solver's `SolveScalar` lanes cannot share `Decide`'s, say why at the site. Not checked: whether the chart's `Backwards` margin is `turned_between`'s margin mapped (both are `1 + c` against the linear band).

P1: two implementations of one underlying logic, which entrenches as more places read the least turn.
