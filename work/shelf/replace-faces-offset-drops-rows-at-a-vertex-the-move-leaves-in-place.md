---
id: replace-faces-offset-drops-rows-at-a-vertex-the-move-leaves-in-place
kind: issue
title: replace_faces_offset drops the rows at every vertex in its move set, including one the move leaves within the band of where it was
status: open
opened: 2026-10-06
priority: P3
refs: [set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart]
---

Found by reading, while answering a review question on PR 4165; not
reproduced. A 1348-case offset probe (native bodies and every STEP
fixture through `replace_faces_offset`, `offset_planes_together`,
`offset_charts_together`, `shell` and `shell_open`) showed no change in
outcome or rows from the drop, so this is a doubt, not a defect seen.

`replace_faces_offset` (`crates/topo/src/replace_face.rs`) drops the rows
of every edge that ends at a vertex in `moved`, after
`move_points_then_rechart` and before the re-anchors' `set_edge_curve`
calls. `moved` holds every plan endpoint, each at its first candidate
point. Nothing checks that the point differs from where the vertex was.

For a vertex the move displaces past the band, the drop loses nothing.
A fitted row on a neighbour that keeps its chart ends on the chart point
of the old vertex, so the closing mint's `pcurves::carry_rows` would
re-certify it against an edge that ends elsewhere, and refuse.

For a vertex in `moved` that lands within the band of its old point, the
row may still certify. Then the drop removes a row that `carry_rows`
would have carried. Where the closing mint has no route of its own to
that row's class, the face leaves without it. No probe case reached this.

Closing options:

- restrict the drop to vertices the move displaces, decided at the band;
- or show that an offset plan never leaves an endpoint in place, and say
  so at the drop.

