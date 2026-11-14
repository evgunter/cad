---
id: an-arc-on-a-turned-plane-misses-its-own-vertex-at-check-3-over-the-box
kind: issue
title: An arc on a turned sketch plane fails check 3's endpoint pin over the box: carrier and vertex reach one point through the frame by two routes
status: open
opened: 2026-11-14
priority: P2
cost: M
---

Found by BOXFINISH's probe row `m3_turned_plane_door` (probe branch
`boxfinish/probe-switch-off`, never merged), re-run on
`boxfinish/sweep-frames`, the branch on which the sweep doors stop
fitting planes to placed vertices. The row extrudes a full circle of
radius 5e-3 drawn on `SketchPlane::from_frame(RigidMotion::rotation(y,
30° ± 1°))`.

Before that branch the build refused `newell_plane_residual` at
`Interval` and `carrier_endpoint_end` at `Sym<Interval>`
(`[1.2e-18, 7.8e-4]`). After it, nothing in the door fits anything, and
both scalars refuse at check 3's endpoint pins
(`crates/geom-brep/src/certify.rs:4907-4920`,
`Margin::of(spec.carrier.eval(t).distance(end))`):

- `Interval`: `Escalated { check: EndpointStart, sample: 0,
  Indeterminate { margin: Enclosure [0, 3.3715e-4], band (1e-9, 1e-8),
  predicate: "carrier_endpoint_start" } }`.
- `Sym<Interval>`: `Escalated { check: EndpointEnd, sample: 8,
  Indeterminate { margin: Enclosure [1.1876e-18, 3.3715e-4], …,
  predicate: "carrier_endpoint_end" } }`; session counts symbolic 3,
  numeric 15, frozen 0.

The circle's carrier is the placed centre plus `r·(cos t·u_ref + sin
t·(axis × u_ref))`, with `u_ref` the frame's first axis turned; its
vertex is the frame's placement of `(r, 0)`. Both are the same point
for every θ in the box, but they reach it through the frame's entries
by two routes, so the residual's enclosure carries the frame's width,
at first order in δ (`3.37e-4 ≈ r ×` the axes' bracket). At
`Sym<Interval>` the residual is a `distance` (a square root) and its
end sample evaluates `cos 2π`, `sin 2π` as rounded constants, so it
never folds to zero and falls back to the enclosure. Inferred, not
traced: which of the two (the root, or the rounded full turn) keeps the
symbolic lane from closing.

The frame itself is
`work/scalar/a-rotation-about-an-exact-axis-widens-the-axis-it-turns-about.md`
(`v.y = [0.98255, 1.01745]` in a `UnitVec3`); fixing that narrows the
bracket but does not remove the two routes. Check 1 on the same frame
is `check-1-reads-a-turning-frames-length-off-its-bracket.md`, whose
note records this row's earlier refusals.

What closes it: the endpoint pin of a carrier whose frame is the same
frame as its vertices' placement is decided without reading the
frame's width (by construction, or symbolically), and the probe row's
door builds at `Interval` or `Sym<Interval>` over the 1° box. A row at
the door that fails on today's code.

