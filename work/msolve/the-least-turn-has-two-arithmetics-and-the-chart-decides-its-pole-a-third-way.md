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

- `crates/editor-core/src/mate/turn.rs` `LeastTurn` calls itself the least turn's one home. It uses the Rodrigues form `c·I + [k]× + kkᵀ/(1+c)`, and its pole is read by `one_plus_c.quoted() <= 0`.
- `geom_core::linalg::rigid_motion` `RigidMotion::turned_between` is the same map in a quaternion form over `|h|²/2`. Its pole is decided under the linear band (`TurnError::{Opposed, Undecided}`).
- `mate/value.rs`'s chart decides the same pole a third way, through `mate_chart_pole` on the chord `|n + d|`.

These are three spellings of one map and its pole. The least turn needs one home below editor-core, in `geom_core::linalg`, which the kernel's frame door now also reads. The chart and the viewer's snap should read the same arithmetic, so that what a gesture writes and what the chart reads back agree by construction. If the solver's `SolveScalar` lanes cannot share `Decide`'s, say why at the site. Not checked: whether the chart's `Backwards` margin is `turned_between`'s margin mapped (both are `1 + c` against the linear band).

P1: two implementations of one underlying logic, which entrenches as more places read the least turn.
