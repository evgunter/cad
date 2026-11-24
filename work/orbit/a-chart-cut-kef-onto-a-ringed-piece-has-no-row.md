---
id: a-chart-cut-kef-onto-a-ringed-piece-has-no-row
kind: issue
title: A chart cut's kef onto a ringed dying piece has no row
status: open
opened: 2026-11-24
priority: P2
cost: M
---

## What

`crates/topo/src/boolean/slit.rs`'s `close_cuts` moves each ring of the
dying piece onto the kept one (`ring_move_minting`) before it `kef`s a
chart cut's piece away. PR 7664's review (R1, at d8b78b203) replaced
that move with a no-op and all 150 cut-in, sphere, snowman, pip, lens
and spheres_crossing rows stayed green; R2 counted 921 `kef`s over its
probe set and found the dying piece ringless in every one. So the path
is unexercised, and the case where both pieces carry rings is unread.

## Done when

A row builds a pose in which the piece the `kef` kills carries a ring
(e.g. a ball cut by one escape circle and holding a second section
circle on the dying lune), asserts the result's rings land on the
survivor, and goes red under the no-op mutant. The site comment that
names this row then comes out.

Related, from the same review (notes, not asked here): the
`left_standing` guard (`ops.rs`) is a `debug_assert`, so a release
build that missed a piece would return a non-maximal body without a
word; and the held-key preference in `close_cuts` is pinned only by
mesher-outcome rows.
