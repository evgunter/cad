---
id: the-channel-contradiction-check-walks-the-tiers-ladder-on-every-definite-decision
kind: issue
title: The check that the two channels do not contradict walks the tier's whole ladder on every definite decision, and on a curved body's ray decisions that is 75 to 150 ms each
status: open
opened: 2026-11-15
priority: P1
cost: H
design: true
---


Filed by BOXFINISH (2026-11-15) from its M5 measurement
(`boxfinish/probe-switch-off` at `2f37154e`, `BOXFINISH_TRACE=3`; the
box-scalar switch's box arm off, so every sweep door finishes its leaf
at `Sym<Interval>`). It holds BOXFINISH's cost row
(`work/boxfinish/tier-3-at-every-box-replay-leaf-multiplies-the-decisions.md`),
which the switch's deletion waits on; hence its band.

**What is measured.** A finish that refuses nothing costs 57 ms on a
six-face slab and 842 ms on a four-face body with a cylinder wall
(2 to 4.6 ms at `f64`), 2.76 s on a three-face revolved circle. Almost
all of it is inside `sign_within`, not in building margins (799 of
842 ms; 2711 of 2763), and in a handful of decisions:

- the cylinder-walled body: `side_at_infinity_winding`, 6 decisions,
  453 ms (75 ms each) and `side_at_infinity_denom`, 2 decisions, 239 to
  305 ms; together 82% of the finish. On the slab the same two are 6
  decisions in 7.4 ms and 6 in 6.1 ms;
- the revolved circle: `curve_pair_span_end_chord` 100 decisions
  559 ms, `bool_torus_region_at` 12 decisions 354 ms,
  `azimuth_near_cut_side` 60 decisions 284 ms,
  `side_at_infinity_winding` 2 decisions 276 ms, and smaller.

**Each of those decisions is decided numerically, every time**
(per drive: `side_at_infinity_winding` 24 of 24, `_denom` 12 of 12,
`bool_wall_turn_true` 48 of 48, `curve_carrier_clear` 60 of 60). The
symbolic work on them buys no verdict. Stack samples of the slow row
(four usable) all sit in `sym::contradicts` → `ladder` → `rungs` →
`walk` → `form_in`, two of them under `side_reader::aimed_ray_inner`.

**Where that is in the code** (read, `crates/geom-core/src/sym.rs`,
`Decide::sign_within` for `Sym`): after the numeric channel answers a
definite sign, the exact-witness arm runs
`debug_assert!(!contradicts(self.node))` and the inexact arm
`if contradicts(self.node) { count_theorem_disputed() }`.
`contradicts` "RUNS THE WALKS (`discharge`), which is the cost each arm
pays", and the workspace ships `debug-assertions = true` in release, so
both arms walk in every profile. The walks freeze on the ring's guards,
not the term budget: per slab finish 96 nodes in
`side_at_infinity_winding` (80 `Mul`, 16 `Add`, every one
`Refusal::Ring`); on the cylinder-walled row over two drives, 1084
nodes in `side_at_infinity_winding` and 228 in `_denom`.

**Inferred, not measured:** that the curved cases cost more per climb
because the margin's DAG is larger (a ray against a cylinder or a torus
carries roots and turns); form sizes and rungs per decision were not
captured; whether a repeated decision re-walks its form or is served
from the walk's memo.

**The question.** What the check that the two channels do not
contradict is for, and what it may cost: whether it runs on every
definite decision, once per form, under a budget of its own, or in a
run that asks for it; and what a run that skips it can no longer say.
It is a soundness self-check and a receipt column
(`SymCounts::checks_expanded` / `checks_frozen`, the disputed and
contradicted counts), so the answer is a design one.

**What would say it is closed** (BOXFINISH's side): with the switch's
box arm off, a finish of a cylinder-walled or revolved body that
refuses nothing costs within a small multiple of the slab's per
decision, and the seven fast rows M4 lists run inside the 1.0 s budget.
(BOXFINISH orchestrator)
