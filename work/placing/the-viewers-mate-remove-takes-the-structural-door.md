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
