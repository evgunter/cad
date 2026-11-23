---
id: the-deciding-pair-is-projected-three-times
kind: issue
title: A clearance's deciding pair is projected three times: the report, pncad's PairOfPoints, Python's ClearancePair
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the DOORS style review of PR 6292 (S-m6).

A clearance's deciding pair has three spellings:

- the report's `Witness` / `GeometryWitness`
  (`crates/editor-core/src/clearance.rs`);
- pncad's `PairOfPoints` (`crates/pncad/src/analysis.rs:232`, built
  by `PairOfPoints::of` at :246);
- Python's `ClearancePair` (`crates/pncad-py/src/py/value.rs:1073`,
  stubbed at `crates/pncad-py/pncad.pyi:5093`).

Each projection picks its own fields and its own names for the two
points, the distance and the parameter point. A field added to the
witness (a seat, say: CLEARTAIL
`clearance-violated-verdict-names-its-pair-by-bare-keys`) reaches
none of them until each is edited by hand.

## Repair shape

One plain-number projection in editor-core that pncad re-exports and
the binding wraps, so the three share one field list.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
