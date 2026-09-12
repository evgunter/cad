---
id: lift-comparator-misclasses-a-declared-joint-difference
kind: issue
title: the lift census comparator classes a lift whose vertex table is bit-identical but whose tangent_joints differ as Mismatch — the ruling-true lift of undeclared cocircular data reads as a failure
status: open
opened: 2026-09-08
refs: [BOOL-10, 2135, BOOL-9]
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
