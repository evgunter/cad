---
id: the-shell-seam-piece-carry-has-no-fixture
kind: issue
title: "The shell carry's seam-piece arm has no row: no fixture puts a record on a wrapped chart's divided seam"
status: open
opened: 2026-10-11
priority: P3
cost: M
refs: [surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key]
---

## What

`Shelled::lineage` (`crates/topo/src/shell.rs`) divides a divided seam
(`RimNaming::seam_pieces`) into its pieces and the ring corners they
end on, its span across the opening removed, so a record on the seam
of a designated chart that wraps between two boundaries lands on the
seam piece (or the corner) whose interior still meets its other cell.
No test reaches that arm: a record needs another body resting on a
cylinder-like chart's seam line, built by a union, and the boolean's
support for a vertex or edge resting on a curved face's seam was not
measured. The carry itself is the one carry every op shares
(`ContactRecords::carried_through` over a `topo::Lineage`), which is
rowed (`crates/topo/tests/one_record_carry.rs`,
`crates/topo/tests/surgery_carries_records.rs`), so what is untested is
only that the seam's fate is the right one.

## Fix

Find or build a fixture: a tube whose outer wall is designated open
(`band_between_boundaries`), with a body touching the wall's seam, then
assert the record lands on the seam piece that holds the touch and
passes at rest.
