---
id: a-remainder-reader-of-a-named-pose-over-the-cut-refuses
kind: issue
title: A kept reader of a named pose over the cut refuses SeveredEdge, where an anonymous one crosses
status: open
opened: 2026-10-28
priority: P3
cost: M
design: true
---

## What

`split`'s crossing walk (`refactor.rs`, `split`, the `crossing_poses`
loop) lets a kept node's pose read cross the cut only through
`pose_leaves`, which answers `None` for a named pose. A kept mate whose
side is a named pose over a face of a cut copy therefore finds nothing
carried, and the upstream walk refuses `SplitError::SeveredEdge`; the
same mate with the pose written in place crosses, the pose re-authored
over the instance's copy. Inline's host-mate rewrite has the same
anonymous-only gate (`inline`, the `filter(|&side|
doc.var_name(side).is_none())` over the host's mate sides, and
`pose_leaves(doc, side).is_none()` beside it).

Measured: `p2_face::a_kept_mates_named_target_is_severed_where_its_written_one_crosses`
splits `face_across`'s scene with `m`'s target written in place (it
crosses) and the same target renamed (`SeveredEdge`, naming `k`). It
refuses typed rather than stranding a read.

A sibling: an unread named pose over a cut body stays in the
remainder, reading a selection of a body the remainder no longer
holds, as an unread named selection does (`moving_vars` sends a group
whose reads reach only outputs to the staying side).

## Fix shape

A named pose read across the cut is one variable on both sides, which
the variable rule refuses (`UncutVarReference`). A pose read only by
kept nodes could be re-authored over the instance's copy under its
name, as `crossing_poses` does for an anonymous one; whether a named
pose may be re-defined that way is a ruling.
