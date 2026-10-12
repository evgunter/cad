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

`Shelled::carry_contacts` reads `RimNaming::seam_pieces` as division
rows (`crates/topo/src/shell.rs`), so a record on the seam of a
designated chart that wraps between two boundaries lands on the seam
piece whose interior still meets its other cell. No test reaches that
arm: a record needs another body resting on a cylinder-like chart's
seam line, built by a union, and the boolean's support for a vertex or
edge resting on a curved face's seam was not measured. The arm shares
its code with the split's edge pieces and the shell's hole rims
(`ContactRecords::carried_through_pieces` over `Divisions`), which are
rowed (`crates/topo/tests/divided_cell_carry.rs`), so what is untested
is only that the seam rows are the right rows.

## Fix

Find or build a fixture: a tube whose outer wall is designated open
(`band_between_boundaries`), with a body touching the wall's seam, then
assert the record lands on the seam piece that holds the touch and
passes at rest.
