---
id: the-boolean-operand-gate-re-runs-tiers-1-and-2-on-finished-operands
kind: issue
title: The boolean's operand gate re-runs tiers 1 and 2 on operands the door's type already guarantees finished
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [boolean-door-adopts-the-finished-body-type]
---


Left by `boolean-door-adopts-the-finished-body-type`.

The boolean door takes `&AtRestBody`, and at every certifying scalar an
operand reaches it only with `AtRestOutcome::Validated`: tier 3 passed,
tiers 1 and 2 with it. `reduce::gate_operand_pairs`
(`crates/topo/src/boolean/reduce.rs`) still runs
`validate::closed_by_tier` on both operands at every reduction (the
public door's, and the re-cut's re-entry), refusing `CorruptOperand` /
`ScaffoldingOperand`. Against a `Validated` operand those refusals are
unreachable and the pass is a second payment of tiers 1–2 per operand
per op, against DESIGN's "pays that gate once, at the door that built
it". At a dual (`NotRunAtThisScalar`) it is the only structural check
the operand gets.

What closes it: the door carries each operand's outcome into the
reduction and runs the structural pass only on an operand with no
verdict, or a measurement showing the pass is negligible and a sentence
saying why it stays. FUSE's door measurement put tiers 1–2 at 0.44 s
against tier 3's 1.90 s over 963 topo results; the operand pass is two
such runs per op.
