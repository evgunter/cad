---
id: hull-bound-refusals-could-refine-the-composite-before-refusing
kind: issue
title: geom-brep: a C2 hull-limb refusal ends at the first Bernstein bound instead of subdividing until the bound decides
status: open
opened: 2026-10-10
priority: P3
cost: M
---



(Option C of design-fork row 103: a sharpness improvement, not an alternative to the bound ending.)

## What

A C2 hull limb refuses as soon as the first Bernstein control-hull bound exceeds the band. Subdividing the composite would tighten the bound until it is ≤ ε, or until a value at a break proves a miss past the band. Either way, the door would mostly see measured misses instead of loose bounds.

## Repair shape

Subdivide adaptively under a budget, with a break-value lower bound. When the budget runs out, the refusal still takes `Unsized::Bound`'s ending (row `a-certified-bound-refusal-reads-as-a-stored-contradiction`). Price it with the mesh/budget meter.
