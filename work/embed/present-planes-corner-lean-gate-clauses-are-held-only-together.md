---
id: present-planes-corner-lean-gate-clauses-are-held-only-together
kind: issue
title: present_plane's two-line corner gate has two lean clauses that differ only in band, and no row holds either alone
status: open
opened: 2026-11-17
priority: P3
cost: E
---



## Finding

Found in the fix pass of the winding-rule unit (`embed/winding-rule`, PR 7071),
2026-11-17.

`present_plane`'s two-line corner arm refuses unless both `lean(n1, bv)` and
`lean(n2, a)` decide Positive at `SELF_INTERSECTION_CORNER`. With `n1` the face
side of edge 1's traversal (which ends at the vertex) and `n2` that of edge 2
(which starts there), both leans are the one cross product `−(a × b)` (signed by
the orientation), divided by `|a|` and `|b|` respectively. Each is the distance of one edge's
far point from the other edge's line, so they have one sign, and they part only
where one distance is inside the band and the other is not (a short edge, or a
corner nearly flat).

Two mutants were run (`topo` under the `ci` profile):

- deleting the whole gate turns `a_planes_reflex_corner_of_two_lines_is_not_its_wedge`
  red;
- deleting the first clause alone (`if false && interval_sign_as(lean(n1, bv), …) || …`)
  turns nothing red.

## What closing it takes

A row with a corner whose short edge's far point is inside the band of the long
edge's line, while the long edge's far point is clear of the short one's. It
should refuse, and it would go red on the one-clause mutant. Alternatively, the
gate decides the one quantity once, at the smaller of the two scales, with a
row for that.
