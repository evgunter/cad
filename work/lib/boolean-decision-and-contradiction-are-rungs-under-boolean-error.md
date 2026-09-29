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
section 4) but neither payload. The non-carriage and its falsifier are
argued beside `Contradiction`'s declaration, and
`scripts/payload-rung-sweep.py`'s `DISPOSITIONS` cites that paragraph.

## Repair shape

Carry `BooleanDecision`, `Contradiction` and `SectorRung` in the
prelude's section 4 beside `BooleanError`, as `SurfaceKind` rides for
`CurvedBooleanUnsupported`. Extend
`carried_refusal_payloads_are_matchable_through_the_prelude` to them,
and give each a Python word or a `NOT_BOUND` family in
`crates/pncad-py/tests/test_binding_census.py`. Then delete the argument
paragraph in `refusal_routes.rs` and the two disposition rows.
