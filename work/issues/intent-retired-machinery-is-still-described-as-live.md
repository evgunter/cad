---
id: intent-retired-machinery-is-still-described-as-live
kind: issue
title: prose and one Python surface still describe INTENT-retired machinery as live (literal Frame, PlacedUnion, Datum, world pin)
status: open
opened: 2026-11-02
priority: P4
cost: C
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said. Filed here because the five sites have five owners (`work.py territory`); a lane touching any of them fixes its line as a drive-by. (PLACING orchestrator)

- `crates/editor-core/src/placement.rs`, the module header: "the literal `Frame`… which an explicit pattern rule lists until patterns are index variables". Also the `Frame` doc's "a listed frame asks when it evaluates". Patterns are index variables now. (WIRE / WIREHOLD.)
- `crates/pncad-py/pncad.pyi`, the `Frame` class doc: "It is authored through constructors… refused at the edit door". No door takes a `Frame` any more. The constructors `translation`, `rotate_then_translate`, `point_at` and `path_start_frame`, and `EditError::PlacementAxis`, live on only as `Step::Literal`'s vocabulary. **This half is an authoring surface with no door (P3, not P4), so whether the constructors go is LIB's call.**
- `demos/README.md`: the heatsink and impeller entries name `SetStructuralParam` on a `PlacedUnion`, the hub's `Datum::Axis`, `Boolean(Union)` and a "world pin", all retired.
- `crates/viewer/README.md`: the Transform sentence. It is rewritten with the placement editor (`work/placing/the-transform-gesture-moves-a-placed-body-through-a-transform.md`); listed here only for completeness.

**Test-prose and fixture shapes, from the second sweep (2026-11-02):**
- `crates/editor-core/tests/intent_placement_carries_at_rest.rs::a_pick_of_a_moved_copy_stands_with_it` builds `slid(doc, outer, outer, [0,7,0])`, a further copy of the `⊔` into its own space, and calls it "a moved copy". `[placement.placement]` says nothing moves a body. The kernel's behaviour is a legal further copy, so only the name and the doc comment are wrong. (TCOST/TINT.)
- `crates/viewer/tests/msolve5_unplaced_read_badge.rs::read_below_pattern` builds an identity `Place` of `lift[1]` into the family's own space, then a family of that. That is the `Node::Part`-as-identity-`Place` shape (SPACES' `the-projection-tool-writes-a-member-read-a-pick-already-seats`) inside a fixture. Re-spell it as a read when that row lands. (Unsure that it needs anything.)
