---
id: a-strut-bearing-operand-passes-the-boolean-gates-and-refuses-at-the-join
kind: issue
title: A boolean operand with a valence-1 vertex passes the operand gates and refuses at the join as UnpairedLooseEnds, whose text says kernel defect
status: closed
opened: 2026-10-01
priority: P2
cost: E
branch: cleave/strut-gate
pr: 3860
closed: 2026-10-02
---

Found by the LINALG pole-branch measurement (`linalg/pole-branch-shift`).

A body whose sphere face is slit to its pole (a unit dome from
`sweep::test_support::revolved_about_y_at`, base merged, one band
meridian killed by `Body::kef`) is tier-1 valid and tier-2 invalid:
`validate_closed` reports `[ScaffoldingStrutVertex { vertex }]` for the
pole. As operand A of `boolean_op_with(Subtract, slit_dome, brick y∈[0.5,2])`
at both `f64` and `Interval` it passes `gate_operand_pairs` and
`gate_maximal_faces` (`crates/topo/src/boolean/reduce.rs`,
`gate_maximal_faces`: "seam/strut inside one face: not a coplanar PAIR")
and refuses at the join:

> `Join(UnpairedLooseEnds { count: 2 })` — "the operands' sections could
> not be joined: 2 section ends found no partner … otherwise it is a
> kernel defect"

The same holds for the whole ball slit along one meridian
(`cube ∖ slit ball`, the die-pip placement). The refusal names a cause
the input does not have: the operand is scaffolding, which the boolean
never states it accepts. Open question for the owner: whether the
operand gate should refuse a valence-1 vertex typed (the
`ScaffoldingOperand` precedent for uncertified edges) or whether a slit
operand is meant to be served. Fixture shape:
`crates/sweep/tests/pole_slit_window.rs`'s `slits`.
