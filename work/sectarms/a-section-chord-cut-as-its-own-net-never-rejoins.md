---
id: a-section-chord-cut-as-its-own-net-never-rejoins
kind: issue
title: Section chords are cut as separate spline nets, so two arcs of one fitted curve never rejoin at a vertex that goes 2-valent
status: open
opened: 2026-11-16
priority: P2
cost: M
---


Found by both designers of the closed-loop carrier fork (PR 7042, fork row 279), by reading, not running.

## What

`chord_join::cut_arc` gives each spline section chord its own sub-net, so that every reader of the chord reads only that net. `edge_join::crease_at` reads a spline's own clamped end as a crease, and its module docs say of two such pieces that "the vertex is never joined". So when a later operation leaves a vertex 2-valent between two chords of one fitted section curve, open or closed, the join keeps it. One way this happens: `merge_faces` re-merges two faces of one surface that a seam had split, and the seam edge the loop crossed dies. The body then keeps a vertex where a dead edge happened to cross, against DESIGN's maximal edges and `[intrinsic.contingent]`.

`topo::split::split_edge` keeps both children forward on the parent's carrier as spans, and conic chords are spans of one circle. Only the section's spline chords are cut nets.

## Done when

A spline section chord is its carrier and a span, as a conic chord is (the edge door, `edge_nurbs::analytic_rung3` / `edge_piece`, already cuts its own piece). A row builds a body whose section-loop vertex goes 2-valent between two arcs of one fitted curve and shows the join removing it. Not run yet: confirm the case first with a probe (for example, cylinder ∩ ball with the ball centred on the cylinder's wall).
