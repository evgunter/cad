---
id: editor-core-raw-twin-planes-unreconciled
kind: issue
title: seat6/seat7's kernel-direct twins assert an equality nothing enforces: two plane spellings, hand-held
status: open
opened: 2026-09-15
priority: P4
cost: E
---

## Finding

- **Where**: `crates/editor-core/tests/seat7_sweep_lowering.rs`
  (`raw_cylinder`), `crates/editor-core/tests/seat6_param_source.rs`
  (the inline `raw` block in `the_same_geometry_without_the_channel_refuses`)
- **Importance**: medium
- **Confidence**: sure about the two spellings; likely about the harm
- **Raised by**: the reviewer of `S52`'s PR #2639, 2026-09-15

Both rows assert that a recipe node's lowering reaches **the same body**
as a kernel-direct construction spelled out beside it — *"the same
profile, the same extrude, the same blend at the same radius, and no
recipe layer above them"*. `S52` left both unconverted and said so.

**What nothing checks is the "same".** The recipe side places its sketch
through `on_frame(...)`; the raw side spells `SketchPlane::xy()` (seat6)
or `SketchPlane::new(Affine3::translation(...))` (seat7). Two spellings
of one placement, kept in step by hand. If the recipe's placement
changes, the raw twin does not follow, and the row keeps passing while
asserting an equality that is no longer the one it was written for — or
starts failing for a reason that reads as a lowering bug.

That is the shape `reviewer-style-lane.md`'s Q2 names: an invariant
asserted in prose with nothing enforcing it. The remedy is not to route
the raw side through a shared fixture — that removes the only place a
reader can check the equality, which is the whole value of spelling it
out — but to make the two placements come from **one expression** that
both sides name, so a change to it moves both.

Worth recording alongside: the argument is applied unevenly today.
seat6's raw side already reaches for a shared `square()` while spelling
its plane by hand, so "sharing would hide what the same means" is not
the rule either side is actually following.
