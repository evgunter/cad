---
id: point-in-loop-row-names-name-the-wrong-walk
kind: issue
title: the point_in_loop_* K rows are the crate-private corner walk's and the public point_in_loop's arc walk raises point_in_arc_loop_*: the telemetry names read backwards against the function names
status: open
opened: 2026-10-03
priority: P3
cost: E
---


Found by the PR 3917 review (S1). Since that PR, the public door
`topo::point_in_loop` (`splitting::containment`) reads a loop on its
edges' carriers. On a loop of lines it drops to the crate-private
corner-polygon walk `point_in_vertex_polygon`, whose rows are
`point_in_loop_{segment,boundary,side,advance,arm}` (`containment.rs`'s
`ROWS` and `polygon_walk`'s arm literal). On any arc it raises
`point_in_arc_loop_*` (`WALK_ROWS`, `ARC_LOOP_ROWS`, `carrier_walk`). The
public door's own plane certification raises
`point_in_loop_{normal,plane,query}`. So a K-REPORT reader has to know
that "the `point_in_loop` rows" means the lines-only polygon walk, not
the function of that name.

**Not renamed in 3917 on purpose.** Goldens, k-lint baselines and
`boolean::decision_words` (`boolean/mod.rs`) key on these names. A
rename is a roster change and needs re-baselined K figures.

**Fix shape.** Rename the polygon walk's five rows to name it (say
`point_in_polygon_*`), or rename the arc rows to the public door's
name. Either way, do it in one PR with the K-REPORT roster entry, the
`predicate-dimension-audit.md` rows, `decision_words`, and the k-lint
baseline re-derived per the K-REPORT runbook.
