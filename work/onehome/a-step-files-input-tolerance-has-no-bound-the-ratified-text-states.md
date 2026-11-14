---
id: a-step-files-input-tolerance-has-no-bound-the-ratified-text-states
kind: issue
title: nothing bounds a STEP file's declared input tolerance: [import] lets healing move geometry by O(ε_in) and ties ε_in to ε_input only as an instance of it
status: open
opened: 2026-11-10
priority: P2
---


Found by PR 6717's review (ONEHOME rigid-motion). A STEP import reads ε_in from the file's `uncertainty_measure_with_unit`, and `[import]` (`docs/DESIGN.md`, "Adoption tolerance ≠ kernel tolerance") lets healing "move geometry by up to O(ε_in), a reported model change". `[tolerance.recourse]` says "[import]'s ε_in is an instance of ε_input (iii)", and ε_input is `K·ε`. The ratified text states no upper bound on ε_in itself.

Every assembly placement frame is now minted by the standard's `build_axes` (`crates/step-import/src/entities.rs`, `Resolver::item_defined_transformation`): the axis is normalized, and the reference direction is projected and normalized, so the map is rigid by construction. The reader still adopts a direction within ε_in of unit verbatim, so a file declaring ε_in = 100 m can state an axis 4% off unit. `build_axes` then moves the box's corners by tens of metres. PR 6717 reports that move as `NormalizationKind::PlacementReMinted` with its size, and refuses nothing. This is pinned by `crates/step-import/tests/a_step_placement_is_built_by_its_axes_and_a_move_beyond_rounding_reported.rs`, `a_files_slack_axis_is_built_and_its_metres_long_move_reported`. Recognition and the other uses of ε_in are untouched.

**What is owed.** A ruling on whether ε_in is bounded by ε_input everywhere it is read, or only where adoption moves geometry. If it is bounded everywhere, a file declaring more than `K·ε` would be refused or clamped at the door, with the reason in the file's terms.
