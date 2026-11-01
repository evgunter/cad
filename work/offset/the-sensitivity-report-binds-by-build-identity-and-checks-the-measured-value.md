---
id: the-sensitivity-report-binds-by-build-identity-and-checks-the-measured-value
kind: unit
title: The sensitivity report binds its dual passes to the f64 evaluation by build identity and refuses on a measured-value mismatch; the anchor walk goes
status: closed
opened: 2026-10-25
priority: P1
cost: M
parent: offset
refs: [5689]
closed: 2026-10-25
pr: 5720
branch: offset/sensitivity-build-identity
---


Ev's ruling on PR 5689 (2026-10-25, DUAL-DESIGN DL3) replaces the dual
pass's anchor walk with an input binding and one drift check.

**Today.** `crates/editor-core/src/stackup.rs` pairs a dual pass with a
rebuilt `f64` anchor:
- `pair_record` compares a handed `f64` evaluation's per-node content keys
  with a rebuild's (module doc ~14–45; ~677).
- `pair_pass` compares every seeded `Dual64` pass with the anchor at every
  node (~712). It needs `order` equality and a `payload_digest` per node.

**Build:**
- **The identity.** A build identity holds the document's `ContentPin`
  (`editor-core/src/ident.rs:94`), the parameter bindings as bits, `Tol`,
  and the resolved parts' pins. If no pin of the resolved parts exists
  (`part.rs`, `ident.rs`), add one or file the gap. Without it, two
  evaluations of one document can differ through an edited part file,
  which the per-node keys cover today.
- **The binding.** The report constructor takes the `f64` evaluation and
  the passes, and refuses typed unless their identities are equal. This
  replaces `pair_record` and `pair_pass`.
- **The drift check.** In `read_pass` (~649), compare the value half of
  each pass's measured quantity (`measure_of`) bit for bit with the `f64`
  evaluation's measure, refusing typed on a mismatch. It is one number per
  pass. The design leaves room to name the node later.
- **The lift valve.** A dual `Failed(ProfileLaneReplay)` maps to
  `Unliftable`. Read it off the pass's own result arm, not against an
  anchor.
- **Leave alone.** The chamber verdict's tie (`bind_verdict`/`tie`,
  ~1005–1040) is a same-scalar replay and stays.

**Test.** A pass with a perturbed value must refuse as drift, and a pass
from another revision, ε or parts set must refuse at the binding.

## Closed

Built in PR 5720. `stackup.rs` binds the `f64` build of record to the
unseeded dual base and to every pass by `BuildIdentity` (document id,
content pin, binding bits, tolerance, resolved parts), refusing a
typed `BindingViolation`; each pass's measured value half is compared
bit for bit (`SensitivityRefusal::MeasuredValueDrift`); the lift valve
reads the pass's own `ProfileLaneReplay`. The anchor walk and the
value-channel digest are gone. `EvalOptions::pin` asks a run for its
`Evaluation::pin`. The resolved parts' pin already exists as
`DocRef.pin` (A4); the carried part rows are the identity's parts set,
so no new pin and no filed gap.

The review's fix pass on the same PR made these changes:
- the driver picks the first error in parameter order whatever the
  schedule;
- a record that refused every node is `BindingViolation::NotABuild`;
- a dual-only refusal carries its node, its cause and the record's
  reading (`RecordReading`) into the `Stackup` blocker;
- `PairingViolation` is renamed `BindingViolation`;
- `Evaluation::pin` and `Evaluation::tolerance` are crate-private
  behind getters.

`EvalOutcome::Completed` on a whole refusal is filed as
`a-whole-evaluation-refusal-reports-completed`.
