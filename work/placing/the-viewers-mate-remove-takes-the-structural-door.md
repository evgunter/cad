---
id: the-viewers-mate-remove-takes-the-structural-door
kind: issue
title: the viewer's remove_constraint applies the structural DocEdit::RemoveConstraint, not Recording::remove_mate, and may refuse SlidesDoNotSpan on a tilted copy
status: open
opened: 2026-11-01
priority: P1
cost: M
rides_with: mate-tool-commits-one-mate
---


Found by the PLACING designers weighing `mate-tool-commits-one-mate` (2026-11-01); **unverified**, neither designer built it.

The viewer's `remove_constraint` (`crates/viewer/src/session.rs`) applies the structural `DocEdit::RemoveConstraint`. It does not go through `Recording::remove_mate`. `crates/editor-core/ASSEMBLY.md` says the structural door refuses `SlidesDoNotSpan` where the slides it fills in do not span. So removing a mate from a tilted copy may refuse where the geometry-aware door would not. Removing a mate also writes `0` on every freedom it frees, so the copy jumps to a place nobody saw before the commit.

**Owed.** First, a test that removes a plane mate from a copy tilted off the target's axes. If that test refuses, this row is P0. The bundle editor (`mate-tool-commits-one-mate`, decided 2026-11-01) replaces this path: a mate row's `remove` opens the editor with that mate struck out, and Apply writes the whole bundle. So this rides with that unit. Whatever the editor's Apply writes must not take the structural door where the geometry-aware one exists.

## Re-read (2026-11-02, `placing-bundle-editor`)

**The structural refusal it suspected is unreachable today (traced, not run against the old path).** `SlidesDoNotSpan` needs a chart left after the removal: a mate fixing a direction that remains. Under A11 (1) the only admitted pair is a point beside a direction. Taking the point out leaves the direction, whose three slides always span; taking the direction out leaves a point or nothing, which has no chart. So `DocEdit::RemoveConstraint` could not refuse it on any admitted bundle.

**The other half was true**, and the editor closes it. The removal wrote `0` on every freed freedom, so the copy jumped. `SessionOp::RemoveConstraint` now strikes the mate out of the editor's draft (`MateTool::edit_striking`) and writes `SetBundle`. The freed values are measured where the copy stands (`editor_core::mate::read_back`), and the slides are the ones its geometry leaves. `bundle_editor::removing_a_mate_from_a_turned_copy_keeps_it_where_it_stands` rests post_b and spins it 30° about the shelf's normal, so it is off every target axis. It then removes the rest and checks that post_b keeps its pose to 1e-9. The tree's `remove` opens that draft rather than committing.
