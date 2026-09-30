---
id: boolean-decision-and-contradiction-are-rungs-under-boolean-error
kind: issue
title: BooleanDecision and Contradiction are payload rungs under BooleanError that the prelude does not carry
status: open
opened: 2026-09-29
---


(TOPO, PR 3493: `scripts/payload-rung-sweep.py --check` found the two
new rungs.)

## What

PR 3493 gives `topo::BooleanError` two closed decision types (D4 ¶1
(i)), both declared in `crates/topo/src/boolean/refusal_routes.rs`:

- `BooleanDecision`, carried by `BooleanError::Escalated { decision, diag }`.
  Its `Corner` variant carries a `SectorRung`.
- `Contradiction`, carried by `BooleanError::DeclarationContradicted { fact }`
  and by `MergeCoplanarError::DeclarationContradicted`.

The prelude carries `BooleanError` (`crates/pncad/src/prelude.rs`,
section 4) but neither payload; `scripts/payload-rung-sweep.py`'s
`DISPOSITIONS` files both here.

Two more types ride one rung further down, where the sweep does not
look (it reads the types a curated carrier's own declaration names,
and these are named by `BooleanDecision`'s): `SectorRung`, under
`BooleanDecision::Corner`, and `CrossingDecision`, under
`BooleanDecision::Crossing`. The carry below owes them too.

## Why they are not carried yet

A Rust caller can already name and match all four types, since `pncad`
re-exports `topo` whole. What the prelude list would add is the CUR3
property row `carried_refusal_payloads_are_matchable_through_the_prelude`
extended to the new published payloads, and a Python word for each, so
the binding's callers could branch on the decision instead of reading it
out of the sentence. Both are the façade crate's to write.

**The falsifier is a caller who must act on which decision refused**: a
viewer that highlights a corner for `Corner` and a face boundary for
`Containment`, or a Python caller that retries at a smaller tolerance
only where the ending offers one.

## Repair shape

Carry `BooleanDecision`, `Contradiction`, `SectorRung` and
`CrossingDecision` in the prelude's section 4 beside `BooleanError`, as
`SurfaceKind` rides for `CurvedBooleanUnsupported`. Extend
`carried_refusal_payloads_are_matchable_through_the_prelude` to them,
and give each a Python word or a `NOT_BOUND` family in
`crates/pncad-py/tests/test_binding_census.py`. Then delete the pointer
in `Contradiction`'s doc and the two disposition rows.

## One more rung (TOPO, PR 3506)

PR 3506 adds `TorusConvention` (`crates/geom-brep/src/torus_convention.rs`,
re-exported as `topo::TorusConvention`), the
ring-torus convention's half, carried by `BooleanError::DegenerateTorus
{ convention, .. }` and one rung down by `BooleanDecision::Torus`. It
is declared in `geom_brep`, not beside its carrier, so the payload-rung
sweep's narrowed shape (payload and carrier in one crate) does not
count it and it has no disposition row; it is on no curated list all
the same, so carry it with the others. Two further
closed types ride under refusals the prelude does not reach as
payloads: `PlaneRung` (under `CarrierEqError::Escalated`,
`MergeDecision::DeclaredPlanes` and `BooleanDecision::Neighbours`) and
`MergeDecision` (under `MergeCoplanarError::Escalated`).

## More rungs (TOPO, PR 3513)

PR 3513 adds, re-exported beside `BooleanDecision`: `Coincide`,
`LeverArm`, `WallRung` and `SectionRadius` one rung under
`BooleanDecision`; `NeighbourOffset`, carried by
`BooleanError::CoplanarNeighbours { offset, .. }`; and
`RestZipFrontier`, carried by `BooleanError::RestZipUnsupported { what }`.
The last two are payload rungs of `BooleanError` itself, so the sweep
counts them, and their disposition is this row.
