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
