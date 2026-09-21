---
id: load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates
kind: issue
title: Load-shaped doors outside persist/check.rs were never swept for edit-door predicates spelled twice
status: closed
opened: 2026-09-16
branch: edit/one-predicate-round-two
pr: 2772
closed: 2026-09-16
---


The sweep behind `blend-selection-canonical-check-load-only` matched
every `return Err(SnapshotError::…)` in
`crates/editor-core/src/persist/check.rs`'s `validate_document` and
asked, per site, whether the predicate is shared with the edit doors or
hand-written at the load door. It found three hand-copied pairs
(`three-door-predicates-are-hand-copied-not-shared`), but the pattern
sees ONE door: a predicate duplicated between an edit door and a
load-shaped door that does not spell `SnapshotError` inline is
invisible to it. Two such doors exist and were never looked at:

- **`crates/editor-core/src/persist/wire.rs`** — the deserialization
  seat. Its refusals are `serde::de::Error::custom` and its own error
  type, not `SnapshotError`, so nothing it checks was matched. Whether
  it re-decides anything an edit door decides is unmeasured.
- **`crates/editor-core/src/program.rs`'s replay** — a snapshot's edit
  log is replayed through `apply`, so by construction it inherits the
  edit doors rather than mirroring them; what is unmeasured is whether
  replay does any pre- or post-check of its own beside that.

The pattern that would find them, which this row exists to run: for
each door that admits a document or a node from OUTSIDE the edit log
(`persist/wire.rs`, `persist/mod.rs`'s `load`/`save` seats,
`program.rs`'s replay), list every refusal it can produce and ask
whether an edit door produces a refusal over the same predicate. A hit
is a predicate with two homes; the fix each time is one home with the
doors naming the answer in their own vocabulary.

Recorded as a row rather than a sentence in a PR body, because a PR
body is not a slate: the blind spot outlives the unit that disclosed it.

## Closed (2026-09-16, `edit/one-predicate-round-two`)

The pattern the row asks for, run at merge base `3648067fd` and
re-checked at the merge of `662380075` (which moved `edit.rs`'s and
`persist/mod.rs`'s doc prose only — no refusal moved): for every
door that admits a document or a node from OUTSIDE the edit log, every
refusal it can produce, and whether an edit door decides the same
predicate.

| Door | Refusals it can produce | Does an edit door decide the same predicate? |
| --- | --- | --- |
| `persist/wire.rs` — `Expr` / `MeasureExpr` rebuild | `DimensionError` from `Expr::*` and `MeasureExpr::*`; `UnknownDisplayUnit` from `quantity::unit_by_symbol` | **Delegated, not duplicated.** The wire REBUILDS through the same smart constructors authoring calls, so there is one home already. The residue is that they are stringified through `Error::custom` — PORT's `load-path-stringifies-structured-refusals`, not this row's. |
| `persist/wire.rs` — `plane_ref` | a `plane` key that is not a node id (a pre-frame-node sketch-plane placement) | No. A wire-SHAPE refusal about a spelling the current document type cannot hold; no edit door has anything to say about it. |
| `persist/wire.rs` — `arc_sweep` / `arc_side`, `deny_unknown_fields` | a kernel-foreign tag or field this build has no name for | No. Wire-shape only, same reason. |
| `persist/strict.rs`, `persist/pairs.rs` | a duplicated map key / appearance key | No, and there cannot be: an in-memory `BTreeMap` holds no duplicate, so the edit door has no refusal to duplicate. |
| `persist/kernel_wire.rs`, `persist/canon.rs` | none (adapters and the canonical-bytes writer) | — |
| `persist/mod.rs` — `load` / `save` seats | `HeaderId`, `IdMismatch`, `Parse`, `Unreadable`, `Serialize`, `ToleranceConflict`, `ToleranceInvalid`, `EditReplay` | No. Header, parse and serializer failures are direction-asymmetric by nature; ε reconciliation is process state, not a document property; `EditReplay` DELEGATES — it is `apply`, the edit doors themselves. |
| `program.rs` — the replay probe (`ProfileProgram::check`) | `ProgramRefusal::{Resolve, Transition, Geometry, Validate}` | **Delegated.** `InsertNode`'s VQ9 door and `first_program_fault` call the one function. They select different ARMS of its answer — the load door takes `Transition` only, because a program that refuses to resolve or to build geometry is legal at rest (V1 class 2) — and both sites say so. One question, two documented readings, not two spellings. |
| `persist/check.rs` — `first_program_fault`'s slot walk | `ProgramFault::SlotDimension` | **HIT.** The same predicate as `edit.rs`'s `check_node_slots` — a slot expression carries the dimension `SlotId::dimension` fixes — spelled a second time, in a second vocabulary, over profile nodes only. Filed as `load-door-checks-slot-dimensions-for-profile-nodes-only`, not fixed here: unifying the two decides three open questions (the missing-expression case, whether the load door asks `check_param_refs`, and which vocabulary survives), so it is a unit rather than a follow-through. |
| `persist/check.rs` — `validate_document` | `SnapshotError::*`, `NonFiniteSite`, `DistributionFault`, `DisplayUnitRefusal` | The original sweep's ground, and its classification was short. Read afresh at this branch's head: the per-site census lives in `three-door-predicates-are-hand-copied-not-shared`'s `## Census` section, which names every refusal the function can produce with its predicate and its edit-door twin, and supersedes the original sweep's count of three. |

**The blind spot this table cannot see.** It reads the doors the row
named, plus the four `persist/` modules beside them. It does NOT read
the doors that admit a document by another route: `assembly.rs`'s mint
and instantiate seats, the workspace store's scan, and
`ProfileProgram::from_recorded`'s `RecordedProgramError` (a recorded
step list arriving from evaluation rather than from a file). A
predicate duplicated between an edit door and one of those is invisible
here, exactly as `first_program_fault` was invisible to the
`SnapshotError` pattern — the same defect one door further out.
