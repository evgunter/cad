---
id: the-tour-folds-face-normals-and-flux-signs-by-hand
kind: issue
title: demos/tour folds a face's sense bit by hand (teapot normals, patch_quadrature's flux sign) beside the public door it uses elsewhere
status: open
opened: 2026-11-14
priority: P3
---


## Finding

Found by the review of TESSLANE's sense-door unit (`tesslane/sense-door`),
2026-11-14. topo's fold census walks only `crates/*/src`, so these are
invisible to it.

- `demos/tour/src/teapot.rs`, `patch_quadrature`: `let sense = if f.sense
  { 1.0 } else { -1.0 };`, then `let normal = jet.du.cross(jet.dv) * sense;`.
  This is a NURBS face's outward normal folded by hand. The same file reads
  `pncad::geom_brep::OutwardNormal::from_chart(*normal, f.sense)` a few
  hundred lines further down, so the door was in reach.
- `demos/tour/src/patch_quadrature.rs`, `measure`: the same `±1` local
  multiplies both a NURBS flux and, in the plane arm, `a * lever`, where
  `lever` is read along the plane's STORED chart normal. In the plane arm
  that is a normal folded through a scalar.

Per `memories/demo-purpose.md`, a demo's awkwardness is a library finding.
Either the natural spelling through the public doors works and the demo
should use it, or the gap that pushed the demo to fold by hand is what this
row records.

## What closing it takes

Rewrite both sites through `OutwardNormal::from_chart` (or a keyed door)
where a normal is meant, and say whether a flux term is genuinely a signed
scalar at its point of use. If a door is missing for a demo's natural
spelling (for example, a NURBS face's outward normal at a chart point from
outside `topo`), file that gap and cite it here. Re-pin any tour output that
moves, with the cause.
