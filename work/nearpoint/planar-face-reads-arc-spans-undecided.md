---
id: planar-face-reads-arc-spans-undecided
kind: issue
title: props: planar_face integrates every circle and ellipse arc's stored span with no span decide, so a hand-built arc past a period counts the extra turn
status: open
opened: 2026-10-11
priority: P3
cost: E
---


Found by the sweep of the FLUX cone-tail unit (`flux/cone-tail`), which
decides every conic span the cone's vector-area lane reads.

`geom_brep::props::planar_face` (`crates/geom-brep/src/props/mod.rs`)
sums `loop_vector_area` over every loop with no band and no decide;
`loop_vector_area` (`crates/geom-brep/src/props/loop_area.rs`) reads a
circle's span through its segment term `R²·(Δt − sin Δt)` and an
ellipse's through `a·b·(Δt − sin Δt)`, beside a chord that is zero for
whole turns. A hand-built full circle stored over `[0, 2τ]` closes on
itself and answers `(1/2)·R²·4π`, twice the disc. Every other reader of `loop_vector_area` decides the span
first: the cylinder's rims are minted through `Rim::decided`, the
sphere's arcs through `props_sphere_arc_span_*`, the torus through its
classification, the cone through `require_cone_conic_spans`; and
`topo::props::quad_lane` reads a body's own edges, certified within
the bound.

Certified bodies are unaffected: every certified door bounds the span
at `τ + zero/r`. Reach is `LoopEdge::hand_built` through the public
`planar_face`.

Fix shape: `planar_face` takes a band and decides
`require_span_within_period` on every circle and ellipse edge, at the
radius and at certification's ellipse meter (the smaller semi-axis
magnitude), under its own pair. One red-first row on a hand-built loop.
