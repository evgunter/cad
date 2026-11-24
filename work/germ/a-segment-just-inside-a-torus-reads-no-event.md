---
id: a-segment-just-inside-a-torus-reads-no-event
kind: issue
title: A line segment inside a torus solid whose shallowest point stands 4ε to 9ε under the surface reads as no event: the both-ends-inside torus arm takes certified roots past the span as NoInterior
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [the-line-torus-door-misses-silently-within-the-band]
---


Found by ORBIT's torus-touch build (branch `orbit/torus-touch-build`),
beside `the-line-torus-door-misses-silently-within-the-band`.

## Measured (ε = 1e-9, K = 10; donut `R = 2`, `r = 0.5` about y)

A segment of half-length 1e-6 centred a depth `g` under the outer
equator's top `(0, 0, 2.5)`, read by `line_quadric_roots`:

| direction | g ≤ 5e-10 | 2e-9 | 4e-9 | 9e-9 | 2.4e-8 |
|---|---|---|---|---|---|
| along the axis | Tangency | escalates | escalates | roots past the span | roots past the span |
| generic (1, 0.6, 0) | Tangency or escalates | escalates | roots past the span | roots past the span | roots past the span |

`reduce.rs`'s both-ends-on-one-side arm for a torus reads certified
roots none of which lies inside the span as `NoInterior`, no event. A
segment whose ends stand decided inside (past `Kε`) while its middle
rises to within 4ε–9ε of the surface (the torus's residual is concave
along a line) is therefore silent in the band's sliver.

## What it touches

The section certificate's touch needs the sweep to see an edge within
the touch's margin `m ≤ 2ε` of the other carrier (premise S′,
`boolean::section_cert`, "A touch"). This band, 4ε–9ε, lies past that,
so a touch at an extreme does not lean on it; a reading that widened
the touch's margin, or a touch at a saddle (whose reach is about `2m`,
4ε), would. The fix it wants is the sibling's: the silent answer
decided as a clearance in metres (the residual's extreme over the
span).
