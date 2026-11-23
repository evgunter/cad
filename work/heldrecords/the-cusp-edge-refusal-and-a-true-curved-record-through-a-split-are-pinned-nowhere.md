---
id: the-cusp-edge-refusal-and-a-true-curved-record-through-a-split-are-pinned-nowhere
kind: issue
title: Two properties lost their rows in PR 7124: a solid missing its cusp edges refuses, and a true curved record lands on the right side of a split
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Two properties that had rows on main and have none on PR 7124's head
(`embed/records`, 2026-11-23).

1. **A solid missing its cusp edges refuses** (`CuspEdgesMissing`).
   `a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared`
   built one through `plate ∖ a rod tangent from inside`; that cut
   refuses now on the tangent record it wrote
   (`a-cut-by-a-rod-tangent-from-inside-records-its-material-crescent-as-a-tangent-contact`),
   so the test was deleted. The cut was the only door that builds a
   closed body missing its cusp edges. The kernel's sheet fixtures were
   tried in its place and answer an unrelated `CensusUndecidable`
   beside the cusp finding. Owed: a fixture that is a closed solid
   missing its cusp edges and nothing else.
2. **A true curved record carried by a split onto the side that holds
   it.** `crates/sweep/tests/a_carried_record_no_reading_covers.rs` on
   main carried a curved record through a split and pinned the side it
   landed on. The record it used has no touch behind it, and a gate
   refuses it now, so the rows that are back pin the refusals: a split
   keeping both cells refuses at its side's gate, a split dividing them
   refuses `Carry { Stale }`. No row carries a curved record that is
   true through a split and reads which side holds it; planar records
   have `one_record_carry` and `divided_cell_carry`. Owed: a body whose
   own door records a true curved touch (an arc resting on a face), cut
   by a plane that keeps both cells on one side.
