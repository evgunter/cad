---
id: lift-comparator-misclasses-a-declared-joint-difference
kind: issue
title: the lift census comparator classes a lift whose vertex table is bit-identical but whose tangent_joints differ as Mismatch — the ruling-true lift of undeclared cocircular data reads as a failure
status: closed
opened: 2026-09-08
refs: [2135]
priority: P0
cost: D
closed: 2026-10-02
---

Found by BOOL-10 (PR 2135) when `repair_same_carrier` was re-targeted
from `Step::ArcContinue(p)` to `Step::Tangent` + `Step::TangentArcTo(p)`:
the lift of the undeclared cocircular twin of `half_disc` produces a
vertex table bit-identical to the input with ONE joint now declared —
which is the ruling-true answer (a zero-turn joint is a declared tangent
joint) — and the census comparator's "joint-set difference ⇒
incomparable" rule classes it `Mismatch`. A row on the branch pins the
fact ("vertex table bit-identical, one declared joint") so the class is
not read as a regression. The comparator wants a class for "same table,
declarations differ", or the rule that a lift MAY declare what the data
left undeclared. `lift.rs` is BOOL-9's ground; sequence after both
land. Difficulty S. Not scheduled.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Closed (2026-10-02)

Re-checked against main after the canonical-segment comparator landed:
the undeclared half-disc still classed `Mismatch` on the joint-set rule
alone, its table bit-identical. The comparator now takes the issue's
second option, under the sixth-round ruling (every zero-turn joint is
a declared tangent joint): a lift MAY declare a joint the source left
undeclared, and never drops one the source declared. The joints it
added ride `LiftOutcome::Lifted::declared` (source indices).
`half_disc_undeclared` is a bit-identical census row declaring `[1]`
(`lift_census.rs`'s `an_undeclared_cocircular_run_lifts_as_the_declared_joint`);
both directions are pinned in `lift.rs`'s
`an_added_declaration_compares_and_a_dropped_one_does_not`.
