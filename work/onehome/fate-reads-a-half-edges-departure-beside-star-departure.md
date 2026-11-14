---
id: fate-reads-a-half-edges-departure-beside-star-departure
kind: issue
title: boolean/fate.rs leaving reads a half-edge's departure itself, beside star::departure
status: open
opened: 2026-11-10
priority: P2
cost: C
---


Found by `run-loops-of-split-and-pierce-are-twins` stage 2 (`onehome/star-walk`). Its sweep looked for every reading of an orbit half-edge's departure (`ConicDeparture::of`).

`topo::star::departure` (`star/walk.rs`) is now the one reading of where a half-edge leaves its vertex. The star's walk, the split, `census::departure` and the Boolean all take it.

`boolean/fate.rs` `leaving` (called by `fate::departure`) still reads it itself:
- a line's chord;
- a curved edge's `ConicDeparture::of`, with its bend where `BendBounds::Planar`;
- or a secant to the curve's midpoint.

Its reach is a `LeverBounds` from `cover::edge_reach_bounds`, not the edge's extent, and it returns `Option` where the star returns a `StarFault`.

Owed: decide whether `leaving` reads `star::departure` (its direction and its `BoundSupport`) and keeps only its secant arm and its reach bounds, or say why a fate cut's departure is a different question.
