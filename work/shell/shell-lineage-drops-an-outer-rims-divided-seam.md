---
id: shell-lineage-drops-an-outer-rims-divided-seam
kind: issue
title: Shelled::lineage states no fate for an outer rim's divided seam: its seam_pieces self row hides the seam from lineage::roots
status: open
opened: 2026-10-13
priority: P2
cost: E
refs: [5049]
---

Found by `fuse/one-lineage-list` (the `lineage-lists-are-five-newtypes-over-one-core`
unit, on fuse) while measuring whether `RimNaming::seam_pieces` could go on a
checked mint list. **Suspected, not yet pinned by a test.**

`band_between_boundaries` (`crates/topo/src/shell.rs`, the
`SeamedRim` it returns) writes `seam_pieces: vec![(seam, source_seam), (top, source_seam)]`.
With `side` `Outer` the first piece keeps the divided edge's key, so
`seam == source_seam` and the first row maps the seam to itself. A
probe over the sweep and editor-core shell tests (`verbs_shell`,
`shell_node`, `emit_shell`, `edge_piece_locality`: 90 tests) saw 7 rims
written, 4 of them with that self row.

`Shelled::lineage` (`crates/topo/src/shell.rs`, the loop over
`roots(&rim.seam_pieces)`) states the divided seam's fate from
`crate::lineage::roots`, which returns each divided-from cell that no
row lists as new. With rows `[(S, S), (T, S)]`, `S` is listed as new
(the self row), so `roots` returns nothing and the loop never states
`S`'s fate: the outer rim's divided seam is left to whatever `lineage`
says of a cell it never touched, not `opened([S, T], corners)`. The void
rim is read wrong the other way. Its divided edge is the cavity twin
`W`, so its rows are `[(S, W), (T, W)]` with no self row: `roots` finds
`W`, and `Descent::of(W, …)` holds `W` itself, so `W`'s fate is
`opened([W, S, T], corners)`, the twin listed among its own pieces.

What is owed: a row that builds an outer seamed rim and asserts the
divided seam's fate (red today if the suspicion holds), then the
reader fixed, for instance by reading `seam_pieces` as the map it is
(group the pieces by their second column) rather than through
`roots`/`Descent`, which covers both cases.

## Merged in: `the-shell-seam-piece-carry-has-no-fixture` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: "The shell carry's seam-piece arm has no row: no fixture puts a record on a wrapped chart's divided seam". Its body follows, headings demoted.

### What

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

### Fix

Find or build a fixture: a tube whose outer wall is designated open
(`band_between_boundaries`), with a body touching the wall's seam, then
assert the record lands on the seam piece that holds the touch and
passes at rest.
