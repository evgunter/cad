---
id: a-lower-bounds-violated-over-a-box-is-existential-but-priced-as-the-leaf
kind: issue
title: A lower bound's Violated over a box is existential, but the dials fold prices the whole leaf's mass as violated
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the DOORS correctness review of PR 6292 (m5, unsure).

Over a parameter box the two kinds of `Violated` do not mean the same
thing:

- **A value assertion's `Violated` is universal over its enclosure.**
  Every point of the box breaks the bound.
- **A clearance's lower-bound `Violated` is existential.** It carries
  one pair at one point (the leaf's midpoint), and one point of the
  box breaks the bound (`crates/editor-core/src/clearance.rs`, the
  module header's verdict section).

The dials door's fold prices the leaf's WHOLE mass as violated on
that one point: `ClearanceMass::price`
(`crates/editor-core/src/clearance.rs:1537`) sends `Violated` to the
`violated` column. That pricing predates PR 6292; the assertion route
inherits the same meaning wherever a report sums masses by verdict.

## Repair shape

Either price an existential `Violated` as "some of this leaf's mass
is violated", which needs a column of its own, or have the drive
bisect a box-dependent lower bound as it now bisects an upper one
(`EngineRefusal::BoxDependent`), so that a priced `Violated` leaf is
universal. The second matches the upper bound's ruling on PR 6292.
