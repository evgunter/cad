---
id: a-full-turns-pinned-vertex-refuses-at-the-loft-door
kind: issue
title: a loft of one-segment loops whose vertex is pinned refuses PinnedCornerUnsupported: the full turn's wall is built around its strut
status: open
opened: 2026-11-22
priority: P2
cost: M
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


Filed by CHARTTAIL's `charttail/collapsed-column` lane (PR 7325). That
lane builds class 1 of the loft door's pinned corners: a corner pinned
through the span its wall covers is one vertex with no strut. It builds
everywhere but a one-segment loop's one vertex.
`crates/sweep/src/loft/stacking.rs` `stacking_door` still refuses
that case `LoftError::PinnedCornerUnsupported` after every slab has
passed, as a kernel limit. Pinned by
`crates/sweep/tests/pinned_corners_at_the_loft_door.rs`
`a_full_turns_pinned_vertex_refuses_as_unsupported` (two circles, one
turned about the tangent at its vertex).

## Fix

Build the full turn's wall with no strut: the wall's wrap column is
collapsed to the vertex, so its loop is the two rims, with the joint at
the vertex a collapsed side's (`AzimuthRule::OnCollapsedSides`, which
reads the side's held channel at its lever). The full-turn assembly
(`crate::swept::build_full_turn`, `loft.rs` `full_turn`) mints its
strut first and builds around it. Then delete the refusal.
