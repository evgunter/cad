---
id: a-pose-chain-written-deeper-than-the-load-doors-envelope-does-not-load
kind: issue
title: A pose chain written inline deeper than the load door's envelope saves an edit log that does not load
status: open
opened: 2026-10-23
---


A pose read is written inline wherever an edit authors one — a
mate's `on` or `to`, a split's tool — as nested `PoseDef`s
(`Slide { pose, .. }`, `Spin { pose, .. }`, `Flip { pose }`), and no
door bounds how deep the chain nests. The load door's body limit is
`persist::nesting::BODY_NESTING`, `ENVELOPE + 2 · MAX_NESTING`
(`crates/editor-core/src/persist/nesting.rs:76`), where `ENVELOPE` is
the deepest position a save writes an expression at.

Until D′2 that deepest position was a mate's world read, which the
edit door bounded (`MAX_WORLD_READ_DEPTH`), and `ENVELOPE` was 35. D′2
retired world reads; the deepest position is now a pose chain in a
mate's `to`, and `ENVELOPE` is 34, the depth
`expr_nesting_bound::deepest_pose_document` writes (six slides over a
spin, flipped). A chain written one slide deeper saves, and its edit
log refuses to load ("the body nests deeper than … levels"); the
snapshot holds each pose as a variable of its own and is not affected.

Done when a pose chain's depth is bounded where it is authored (the
edit door refusing typed, as `MAX_NESTING` does for an expression),
and `ENVELOPE` is that bound's deepest position, or the load door
reads a pose chain at a depth of its own.
