---
id: split-answers-an-inside-out-operand-with-two-inside-out-halves
kind: issue
title: topo::split answers an inside-out operand with two inside-out halves: its operand gate reads no orientation
status: dispatched
opened: 2026-10-03
priority: P1
cost: M
branch: cleave/split-operand-gate
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
Boolean's does. The Boolean takes finished operands
(`boolean-door-adopts-the-finished-body-type`): `AtRestBody::validate`
refuses an inside-out solid where the operand is finished, and
`boolean::reduce::gate_unverdicted_operand` reads check 7 per solid
before `ops::one_solid` merges an operand that carries no verdict (a
dual).

## Owed

Refuse an inside-out operand at the split's gate, typed (tier 3's check
7 per solid, `validate::inside_out_solids`), before the merge — or take
`AtRestBody`, keeping the per-solid read for an operand with no verdict,
as the Boolean does. Folding
it with `split-gates-its-operand-on-null-edges-not-on-tier-2` into one
operand gate is the natural shape; the finished-body adoption
(`work/reach/boolean-door-adopts-the-finished-body-type.md`, "the other
verb doors … adopt the type each in its own unit") subsumes both.

## Built (branch cleave/split-operand-gate)

The split's doors take the finished-body type, as the Boolean's do:
`split`, `split_reduce`, `plane_section`, `vertex_sides` (and the
test-support `split_through_the_join`) take `&AtRestBody<T>`;
`verbs::Verb::run_split` takes it too, and the editor's split node
finishes its operand at the seat (`finished_operand`, as the Boolean's
does). At a certifying scalar an inside-out body never reaches the door:
`AtRestBody::validate` refuses it `NegativeVolume`. Where no verdict
rides the operand (a dual), the door reads tier 2 and then check 7 per
solid, before `merge_all_solids` reads a several-solid operand as one,
and refuses `SplitReduceError::InsideOutOperand { solid }`.

The per-solid read has one home, shared with the Boolean:
`AtRestBody::gate_unverdicted` (`validate.rs`), which
`boolean::reduce::gate_unverdicted_operand` now calls; the Boolean's
tier-2 read and the split's go through `validate::operand_scaffolding`.

Measured on main `575b309d`, `Tol::witness()`, the clockwise wedge
(volume −0.23492) split at z = 0.75: `Ok` with both halves at −0.11746
at `f64`, `Interval` and `Dual64` alike. On the branch: refused at the
at-rest gate (`f64`, `Interval`) and `InsideOutOperand` at every split
door (`Dual64`); a two-solid body whose total is positive refuses naming
the wedge's solid; the counterclockwise control splits into two halves of
+0.11746 (`topo/tests/split_operand_gate.rs`).

Corpus, main `575b309d`, the workspace's `ci` profile with a probe at
`split` and `plane_section`: 1,916 door calls; 1,849 operands pass the
at-rest gate, 54 are duals (no verdict), and 13 calls in 11 tests hand in
an operand the gate refuses. Those rows were restated: described
fixtures (`describe_as_intersections`) where the pose finishes, the
operand's refusal pinned where it cannot (straight profile corners,
relabelled faces, a two-outer-shell solid), and rows that read the
carrier gate or the reduction past the door through
`topo::test_support::{split_carrier_gate, split_reduce_unfinished}`.
Filed: `split-result-gates-have-no-row-past-the-finished-operand`,
`split-carrier-gate-rows-read-the-gate-past-the-door`.
