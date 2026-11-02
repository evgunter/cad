---
id: engine-refusals-give-no-recourse-and-name-raw-node-ids
kind: issue
title: A clearance's engine refusals give no recourse and ClearanceRefusal renders raw node ids, both user-facing now
status: open
opened: 2026-11-02
priority: P2
cost: M
---

Found by the DOORS style review of PR 6292 (S-m11); reported by the
lane as well.

A clearance assertion's third state is user-facing now (the viewer
draws its sentence under the row; pncad's `ClearanceOutcome::Refused`
carries it). Two things in that sentence were written for engine
rows:

- **No recourse.** `EngineRefusal::say` (`crates/topo/src/clearance.rs`,
  e.g. the `Budget` arm at :625) states what stopped the sweep but
  not what the author can do. `ClearanceRefusal::NotADistance`,
  `EmptyBand` and `NeedsCertifyingLane` end in a `Recourse:`; the
  engine's arms do not.
- **Raw node ids.** `ClearanceRefusal`'s `Engine` arm names each side
  as `body {i} of {node}` through the speaker it is rendered with
  (`crates/editor-core/src/clearance.rs:704`). Rendered with
  `Speaker::TAG` (its `Display`), that is a hex id like
  `3fa9c1d2a0b1` rather than the seat's name.

## Repair shape

A recourse per engine arm, in the shell op's mould
(`crates/topo/src/shell.rs`'s `cavity_refusal_ending`: the geometry's
lever, not-yet-built, or the kernel's defect). The viewer and pncad
render through the document's speaker (`doc.spoken`). DOORS' next
unit (the GUI's clearance consumer) is where the sentence is drawn.
