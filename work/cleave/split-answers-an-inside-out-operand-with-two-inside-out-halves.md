---
id: split-answers-an-inside-out-operand-with-two-inside-out-halves
kind: issue
title: topo::split answers an inside-out operand with two inside-out halves: its operand gate reads no orientation
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

`topo::split` (`crates/topo/src/splitting/mod.rs`, `split`) gates its
operand by hand (the null-edge pass in `splitting/classify.rs`;
`split-gates-its-operand-on-null-edges-not-on-tier-2` is the tier-2
half) and reads no orientation, so an inside-out body enters the
pipeline and comes back as two inside-out halves.

Measured on `cleave/inside-out-gate` (main 82b9ceb2 plus the Boolean's
orientation gate), `Tol::witness()`: `prism_z` over the clockwise
triangle (0,0), 190°, 80° (unit radius), z ∈ (0.5, 1), volume −0.2349,
split by the plane z = 0.75 (normal +z) returns `Ok` with both halves
at volume −0.1175. Neither side is refused, and each is the complement
of the half it bounds.

The split is the same door shape as the Boolean: it reads a several-solid
operand as one merged solid (`merge_all_solids` in `split`), so the
orientation reading has to come before that merge, per solid, as the
Boolean's does (`boolean::reduce::gate_operand`, called from
`ops::one_solid` and `gate_operand_pairs`).

## Owed

Refuse an inside-out operand at the split's gate, typed (tier 3's check
7 per solid, `validate::inside_out_solids`), before the merge. Folding
it with `split-gates-its-operand-on-null-edges-not-on-tier-2` into one
operand gate is the natural shape; the finished-body adoption
(`work/reach/boolean-door-adopts-the-finished-body-type.md`, "the other
verb doors … adopt the type each in its own unit") subsumes both.
