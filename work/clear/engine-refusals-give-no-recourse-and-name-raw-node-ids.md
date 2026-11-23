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

## Update (2026-11-02, DOORS' `the-gui-has-no-clearance-consumer`)

The raw-id half is closed at both doors that draw the sentence today:
`pncad::analysis::ClearanceOutcome::of` takes the speaker and says a
refusal through it (`Said(refusal, by)`), `clearances_at_nominal`
passes the document's (`Speaker::of(doc)`), and the viewer's clearance
row reads its `f64` refusal through the same door with the landed
document's speaker (`viewer::tree`'s `cleared`). A labelled side now
reads `body 0 of Extrude "left" (…)`, never `body 0 of node <tag>`
(`pncad/tests/all.rs`'s
`a_clearance_refusal_names_each_side_as_the_document_says_it`).
`ClearanceRefusal`'s own `Display` still says each node by tag, as a
document-free rendering must, and one consumer still reads it there:
`pncad-py`'s `Value.assertion` (`crates/pncad-py/src/py/value.rs`, the
`ClearanceVerdict::Refused` arm's `r.to_string()`), which holds a value
and no document — BIND's ground; it wants the document the value came
from, or `clearances_at_nominal`'s already-spoken sentence. The recourse half is open: the engine's
arms (`EngineRefusal::say`) still end without one.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
