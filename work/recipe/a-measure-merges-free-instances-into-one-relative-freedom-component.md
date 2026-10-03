---
id: a-measure-merges-free-instances-into-one-relative-freedom-component
kind: issue
title: A measure reading faces of two unmated instances merges them into one A9 relative-freedom component
status: open
opened: 2026-10-03
priority: P2
cost: E
---

`mate::relative_freedom_components` (`crates/editor-core/src/mate/solve.rs:659`) walks `Node::inputs()`, so a `Measure` whose refs read a face of each of two unmated instances joins them: measured on main by a probe (two instances, then a measure reading one face of each), the partition goes from 2 components to 1 and the roots from `[inst0, inst1]` to `[measure]`. A measurement fixes no pose (A9, `crates/editor-core/ASSEMBLY.md`), and the partition is public through Python's `relative_freedom_components`.

`mate::spaces_with` (`solve.rs:473`) already treats a measure or assertion as space-free ("a number, which lives in no space"), so the two walks disagree. The fix is not "A9 excludes every read": a face frame read off instance A's face carries A's pose (a body sketched on it moves with A). A9 should run over every edge but stop at nodes whose value is space-free (Measure, Assertion), as `spaces_with` does.

Found by a designer on `[ev]` PR 3929 (round 2, the edge-typing question); it lands with or after whichever reference shape that PR settles.
