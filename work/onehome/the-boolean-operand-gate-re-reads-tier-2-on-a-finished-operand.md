---
id: the-boolean-operand-gate-re-reads-tier-2-on-a-finished-operand
kind: issue
title: The Boolean's operand gate re-reads tier 2 (ScaffoldingOperand) on operands the scalar's finish already passed
status: open
opened: 2026-11-02
---


Found by the ONEHOME finished-body lane (branch `onehome/finished-body`),
outside its fence.

The public Boolean doors take the scalar's finished body
(`T::Finished`), and every scalar's finish runs tier 2: `AtRestBody`
through tiers 1–3, a dual's `Replay` through `validate_closed`
(`crates/topo/src/finish.rs`). Yet `reduce::gate_operand`
(`crates/topo/src/boolean/reduce.rs:656`) still reads
`validate::operand_scaffolding` on each operand and refuses
`BooleanError::ScaffoldingOperand` (`crates/topo/src/boolean/mod.rs:3234`),
from `gate_operand_pairs` at every door (`boolean/mod.rs:5716`, `5774`,
`5824`, `5983`). On an operand that came through a public door it cannot
fire; it is a second gate on a finished body, the shape the lane deleted
elsewhere (`finished_operand`, `distribute::gated`, `gate_unverdicted`).

The lane kept it because internal callers hand the reduce bare
`Body<T>` reads (the sweep-testing seams, and the merged one-solid
reads), and whether any of those can carry scaffolding is a reading of
the boolean's internals, not of the doors. The work: establish that no
internal caller reaches `gate_operand_pairs` with an unfinished body,
then delete `gate_operand`, `ScaffoldingOperand` and its
`BooleanErrorKind`/offer row/pncad-py tag (`scaffolding_operand`), or
type the internal reads as finished. `InsideOutOperand`
(`boolean/mod.rs:3254`) is the ray-witness read's own refusal, not a
gate, and is the subject of
`inside-out-operand-reading-has-two-homes-and-four-spellings`.
