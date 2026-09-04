---
id: placed-union-has-no-session-op
kind: issue
title: No GUI door fuses a pattern: Node::PlacedUnion has no SessionOp
status: review
opened: 2026-09-01
github: 1456
branch: chrome/placed-union-has-no-session-op
pr: 1762
---

## From GitHub issue 1456

Opened 2026-09-01; 0 comments.

Found by the `story_authoring` integration lane: `AddPattern` yields `Instances`, and `AddBoolean` correctly refuses them (`WrongNodeKind { wanted: Body }` — the F4 partition working as designed). But the kernel's one-body pattern-union, `Node::PlacedUnion` — which the `die_tool` corpus uses and the heatsink demo wished for — has **no `SessionOp`**. A user who patterns a feature in the GUI has no path to merge the result into their part short of N−1 hand-authored transform + union chains.

The refusal side of this is right; what is missing is the affordance the refusal should be pointing at. The shape is one insert op (`AddPlacedUnion { pattern }` or similar) plus a row in the property panel's combine section — everything below the session already exists.

Repro is in-tree: `crates/viewer/tests/story_authoring.rs` deliberately turns this dead end into its delete/undo chapter (the pattern experiment is priced by the delete affordance, deleted, and restored by one undo), so the suite documents the gap until the door exists.

(story-suites orchestrator)

## Home

`work/issues/` — a missing viewer `SessionOp` and property-panel row; GUI and GAUTH are both closed and no open program's territory covers `crates/viewer`.
## Fixed (CHROME, 2026-09-04) — and the item's own sketch was not buildable

**The correction first, because it changes what this item was asking
for.** The sketch above says the shape is "one insert op
(`AddPlacedUnion { pattern }` or similar)". It is not:
`Node::PlacedUnion`'s `input` is the **prototype BODY**, not a pattern
— a fused node replicates the body itself. So the door's seat wants a
body, and there is no "the input is not a pattern" refusal to write; a
`Pattern` node at that seat refuses `WrongNodeKind { wanted: Body }`,
exactly as it does at the unfused door.

A pattern-CONSUMING spelling was considered and rejected on two
grounds. It would either copy the pattern's `count`/`kind` into a
sibling node — two sources of truth for one rule, drifting under any
later slot edit — or REPLACE the node, and `DocEdit` carries no
replace-or-convert variant, so that spelling would have required an
`editor-core` change across this program's `keep_out`. Reported rather
than crossed.

**"The pattern is not one this door can fuse" also has no door-level
answer.** Disjointness is CERTIFIED at evaluation rather than declared,
and the certificate is sufficient-not-necessary, so the honest refusal
is `PlacementsUncertified` on the node's own badge. The door does not
pre-screen it, for the reason the blend door does not pre-screen its
selection: a second authority on the same fact.

**What landed.** `SessionOp::AddPlacedUnion { input, count, rule }`
(`crates/viewer/src/session.rs:1381`), sharing ONE door with
`AddPattern` (`session.rs:2873`) the way `add_blend` serves fillet and
chamfer. The chrome is not a new tool button but an output choice on
the existing pattern form — `PatternOutputChoice { Instances, Fused }`
(`crates/viewer/src/combine.rs:235`) — because a separate `ToolKind`
would have duplicated the whole pattern form. The door mints the
PARAMETRIC constructor; `Explicit(Vec<Frame>)` has no GUI authoring
vocabulary at all (Python's `Node.placed_union_at` is its door), so
that arm is unreachable from here and the pairing is re-checked by
`apply`'s `placement_rule_fault` at the commit door rather than
asserted.

**The three proofs this unit's correctness lane owed**, all local and
all passing: the fused node **round-trips** through save → open into a
fresh session, bit-equal document and bit-equal re-evaluated solid;
**undo then redo** returns bit-equal in both directions with history
length unchanged; and the **refusals are typed and reachable** —
gesture-in-flight, non-body prototype, non-axis pick under the circular
rule — each committing nothing and minting no history state, added to
both hand-written lists in the existing all-doors refusal row.

**`story_authoring`'s dead-end chapter is retargeted, not deleted.**
The boolean refusal of the raw pattern stays, because it is still
correct; the chapter now continues into the fused door and the rook
gets real crenellations with a closed-form volume. The delete/undo
cascade it documents got stronger rather than weaker — four dependent
nodes instead of three.
