---
id: a-step-files-input-tolerance-has-no-bound-the-ratified-text-states
kind: issue
title: nothing bounds a STEP file's declared input tolerance: [import] lets healing move geometry by O(ε_in) and ties ε_in to ε_input only as an instance of it
status: open
opened: 2026-11-10
priority: P2
---


Found by PR 6717's review (ONEHOME rigid-motion). A STEP import reads ε_in from the file's `uncertainty_measure_with_unit`, and `[import]` (`docs/DESIGN.md`, "Adoption tolerance ≠ kernel tolerance") lets healing "move geometry by up to O(ε_in), a reported model change". `[tolerance.recourse]` says "[import]'s ε_in is an instance of ε_input (iii)", and ε_input is `K·ε`. The ratified text states no upper bound on ε_in itself.

Before PR 6717, a file declaring ε_in = 100 m was adopted at a 100 m band, so its placement could be 4% off unit and still import, snapped by metres.

**What PR 6717 does for now.** `geom_core::FileCoincidence::adoption_band` clamps ε_in into `[ε, K·ε]` for the placement's adoption (`crates/geom-core/src/predicate.rs`). A placement off rigid by more than `K·ε` at the session box refuses `PlacementMap`. That is pinned by `crates/step-import/tests/a_step_placement_is_adopted_at_the_files_band_and_its_re_mint_reported.rs` `a_files_coincidence_distance_is_bounded_by_the_runs_k_eps`. Recognition and the other uses of ε_in are untouched.

**What is owed.** A ruling on whether ε_in is bounded by ε_input everywhere it is read, or only where adoption moves geometry. If it is bounded everywhere, a file declaring more than `K·ε` would be refused or clamped at the door, with the reason in the file's terms.
