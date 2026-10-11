---
id: an-eccentric-ellipse-tied-with-a-plane-refuses-on-its-global-curvature-bounds
kind: issue
title: An ellipse tied with a plane face refuses ConicBend on its carrier's global curvature bounds even where it clearly rises off the plane
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, 4575]
---


## Finding

From the dual review of PR 4575 (R1 and R2, sampled). `sectors::conic_side`
bounds an ellipse's lift with its carrier's GLOBAL curvature bounds
(`Bend::of`: `κ ∈ [b/a², a/b²]`) and caps the witness at
`l = min(extent, π/(2·κ_max))`. For an eccentric ellipse the global
`κ_max = a/b²` is huge, so `l` collapses and `κ_min = b/a²` is tiny,
and the certified lift is far below the arc's real one: an ellipse with
`b/a = 0.01` rising `5e-5` off the plane at its witness refuses
`ConicBend`.

The bounds that make the reading sound need only hold over the stretch
the reading reads, from the base vertex to `l`. Bounding `κ` over that
parameter interval (the ellipse's curvature is monotone between axis
ends) would certify the same arcs as a circle of comparable bend.
Reachability by a modelled body is unshown.
