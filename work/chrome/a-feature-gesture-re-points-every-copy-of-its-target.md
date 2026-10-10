---
id: a-feature-gesture-re-points-every-copy-of-its-target
kind: issue
title: A feature gesture re-points every placement of its target body, not only the placement whose copy the author picked
status: open
opened: 2026-10-08
---


Found by INTENT stage 2 unit C (branch `intent/s2-c-world`, PR #4359);
filed on the orchestrator's ruling.

A feature gesture (fillet, chamfer, transform, fused pattern, combine)
re-points every world placement of its target body to the result
(`feature_over`, `combine_in_world` and `world::placements_of_target`
in `crates/viewer/src/`). A copy is its own thing: the gesture should
re-point only the placement whose copy the author picked. Today the
pick is lost on the way in. `Seats::pick` seats a copy's body
(`world::seat_of`), so a `SessionOp` carries the body and not the
picked placement. Only the blend door sees the placement, and that is
because its selection names are on the copy.

The fix threads the picked placement through the body seats and the
`SessionOp`s that hold one. Each gesture then reads the body (never the
world copy, which would mix spaces) and re-points only that placement;
a body picked in the tree, with no copy picked, keeps re-pointing all
of its placements.

Until then Duplicate makes the copy a body of its own (a pattern of
two, its second instance projected and placed), not a second placement
of the original: a feature on either would otherwise re-point both.
When this row lands, Duplicate can become a second placement of the
same body, with no `Transform` (#4326: `Transform` retires).
