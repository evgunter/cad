---
id: SYM-11
kind: unit
title: the point channel is not a proof: the theorem-vs-numeric contradiction charged per witness kind
status: review
opened: 2026-09-21
priority: P0
cost: H
branch: sym/11-witness-kind
refs: [sym-f64-far-placement-trips-the-theorem-vs-numeric-assert]
---

## What

`Sym<f64>`/`Sym<Probe>` panic in `Decide`'s theorem-vs-numeric
`debug_assert!` on an ordinary document placed far from the origin and
under rule F's sign amplification: the assertion's premise ("the
enclosure proves it") holds at exactly the lane scalars whose witness
is EXACT, the partition `Real::register_equal` already draws
(`Contradicted` vs `Disputed`, SYM-6). Phase 1 reproduces and counts
both mechanisms, records the pole, pins that the exact channel never
trips it on the six documents (stop clause otherwise), and writes the
partition down; Phase 2 declares the witness kind on the lane scalar
and charges the contradiction by it — asserted at an exact witness,
COUNTED (`SymCounts::theorems_disputed`, a refusal column, not a
discharge kind) with the numeric answer kept at an inexact one; the
far-placement rows and the adversary become gating. No decision at
`Sym<Interval>` moves. Block SYM-B3 slot 0 (H / STRUCTURAL, pre-draw);
protocol v7 IN, the full v6 dual. Spec: `docs/SYM-11-SPEC.md`.
