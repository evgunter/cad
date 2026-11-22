---
id: frame-relative-composes-through-the-world-root
kind: issue
title: Frame::relative composes through the world root, so a mated pair inherits its parent's placement box
status: open
opened: 2026-11-18
priority: P2
refs: [7079]
---

Filed by the ONEHOME orchestrator from fork 288 (PR 7079). Designer
C's report named it, and all three agreed the frame tree fixes it.

`Frame::relative` (`crates/editor-core/src/eval/framed.rs:89`)
composes two frames as `a.inverse().compose(b)`, both measured from
the space root. A bolt mated to a plate whose placement is boxed
therefore gets a relative motion as wide as the plate's box, although
their true relative pose does not depend on it: the box's width
enters twice and does not cancel at `Interval`.

`[placement.frame]` now says "the relative motion of two frames is
composed through the nearest frame they share". Done when
`relative` walks to the nearest common frame, with a measured row:
the plate and bolt of DESIGN's mate example, the plate placed over a
box, the bolt's relative motion as wide as the mate's own width and no
wider.
