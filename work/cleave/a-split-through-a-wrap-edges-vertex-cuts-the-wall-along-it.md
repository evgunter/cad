---
id: a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it
kind: issue
title: A split whose section loop meets a one-face wall's wrap edge at one vertex leaves that wall's piece as two faces on one surface across the wrap edge: a cut the author did not write
status: open
opened: 2026-10-06
priority: P2
cost: M
---



Found by PATHS `one-segment-loop-revolves-and-lofts-to-one-wall` (the wrap-edge unit, #4175's implementation).

**What.** Extrude a one-segment circle (r = 1 about the z axis, 2 high, its strut on the meridian through its one vertex) and `topo::split` it by the plane through the bottom vertex with normal `(cos φ, sin φ, 1)` (`crates/sweep/tests/one_segment_loop.rs`, `a_split_through_the_seam_builds_as_the_two_arc_form_does`, "through the bottom seam vertex"). The section ellipse touches the strut only at that vertex, so the piece of wall above the plane is one connected region with the whole strut inside it: one face whose wrap edge is the strut. The split returns it as TWO faces on the one cylinder surface, with the strut between them.

**Why it matters.** D1 now reads a wrap edge as a fact about one face ("a wall keeps the cuts its author wrote and the kernel adds none"), and tier 3 holds the two halves of a wrap edge to one face (`topo::validate`, check 2's adjacency arm). The split's extra face boundary is a cut nobody wrote. This unit keeps the result valid by re-describing a wrap edge the split has parted as an ordinary image in its chart (`Body::rest_parted_wrap_edges`, called from `splitting::finish::split_finish` after the section boundary pass), so volumes and tiers are right; the topology still carries the extra same-key face pair.

**Where to look.** `crates/topo/src/splitting/finish.rs` (`split_finish`) and the reduction that places the self-loop section edge at the vertex the wrap edge passes through: the piece should come back as one face whose loop uses the strut twice.
