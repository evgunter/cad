---
id: placed-union-has-no-session-op
kind: issue
title: No GUI door fuses a pattern: Node::PlacedUnion has no SessionOp
status: open
opened: 2026-09-01
github: 1456
---

## From GitHub issue 1456

opened 2026-09-01, 0 comments.

Found by the `story_authoring` integration lane: `AddPattern` yields `Instances`, and `AddBoolean` correctly refuses them (`WrongNodeKind { wanted: Body }` — the F4 partition working as designed). But the kernel's one-body pattern-union, `Node::PlacedUnion` — which the `die_tool` corpus uses and the heatsink demo wished for — has **no `SessionOp`**. A user who patterns a feature in the GUI has no path to merge the result into their part short of N−1 hand-authored transform + union chains.

The refusal side of this is right; what is missing is the affordance the refusal should be pointing at. The shape is one insert op (`AddPlacedUnion { pattern }` or similar) plus a row in the property panel's combine section — everything below the session already exists.

Repro is in-tree: `crates/viewer/tests/story_authoring.rs` deliberately turns this dead end into its delete/undo chapter (the pattern experiment is priced by the delete affordance, deleted, and restored by one undo), so the suite documents the gap until the door exists.

(story-suites orchestrator)

## Home

`work/issues/` — a missing viewer `SessionOp` and property-panel row; GUI and GAUTH are both closed and no open program's territory covers `crates/viewer`.
