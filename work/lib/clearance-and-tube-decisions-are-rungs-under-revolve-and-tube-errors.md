---
id: clearance-and-tube-decisions-are-rungs-under-revolve-and-tube-errors
kind: issue
title: ClearanceDecision and TubeDecision are payload rungs under RevolveError and TubeError that the prelude does not carry
status: open
opened: 2026-10-12
priority: P4
cost: E
---


(ENCL, PR 4766: `scripts/payload-rung-sweep.py --check` found the two
new rungs.)

## What

PR 4766 gives two sweep refusals closed decision types (D4 ¶1 (i)), so
each escalation's ending follows from its decision's kind:

- `sweep::ClearanceDecision` (`crates/sweep/src/revolve/mod.rs`), carried
  by `RevolveError::SliverAxisClearance { decision, .. }`.
- `sweep::TubeDecision` (`crates/sweep/src/revolve/tube.rs`), carried by
  `TubeError::Escalated { decision, .. }`.

The prelude carries `RevolveError` and `TubeError`
(`crates/pncad/src/prelude.rs`) but neither payload, and
`scripts/payload-rung-sweep.py`'s `DISPOSITIONS` files both here.

## Why they are not carried yet

A Rust caller can name and match both through `sweep`. The prelude
carry owes the CUR3 property row extended to them, and a decision on
the Python tags: `pncad-py`'s `revolve_error_tag` and `tube_error_tag`
(`crates/pncad-py/src/tags.rs`) give one tag per arm
(`sliver_axis_clearance`, `escalated`), so a split by decision is a new
Python word. Both are the façade crate's to write.
