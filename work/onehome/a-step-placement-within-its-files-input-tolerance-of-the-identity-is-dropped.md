---
id: a-step-placement-within-its-files-input-tolerance-of-the-identity-is-dropped
kind: issue
title: a STEP placement whose frames are within ε_in of each other is read as no placement, so a file declaring ε_in = 1000 m loses a 990 m translation silently
status: open
opened: 2026-11-10
priority: P2
---


Found by PR 6717's review (ONEHOME rigid-motion), and measured on `onehome/rigid-motion`.

The reader's transformation door (`crates/step-import/src/entities.rs:2400`, `Resolver`'s `ITEM_DEFINED_TRANSFORMATION`) returns `None`, meaning "the identity", when the two frames' origins, axes and reference directions are each within ε_in of one another. ε_in is the file's declared coincidence distance, and nothing bounds it (`work/onehome/a-step-files-input-tolerance-has-no-bound-the-ratified-text-states.md`).

The cylinder of `crates/step-import/tests/a_step_placement_is_built_by_its_axes_and_a_move_beyond_rounding_reported.rs`, placed about 990 m out by an exact turn and the translation `(700, 700, 0)`, lands as follows:
- ε_in = 1e-6 m and 100 m: the cylinder stands 992 m out, and its instance records the map.
- ε_in = 1000 m: the translation is within ε_in, so the placement is read as the identity. The cylinder imports at the origin (reach 2.69 m), its instance records no map, and nothing is reported.

So a placement the file states is dropped, which is a model change larger than any `[import]` licenses ("Healing may move geometry by up to O(ε_in) — a reported model change"). Here it is O(ε_in) only because ε_in is absurd, and it is unreported.

Placements are now minted by `build_axes`, and the move that construction makes is reported. This reader's identity test still compares the stated fields at raw ε_in, and it runs before any frame is built.

What is owed (the importer's lane):
- Read the identity at a band ε_in does not inflate, or refuse a placement whose frames differ by more than `K·ε` but less than ε_in as ambiguous.
- In either case, pin the ε_in = 1000 m case.
