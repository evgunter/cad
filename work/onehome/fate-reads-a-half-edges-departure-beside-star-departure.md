---
id: fate-reads-a-half-edges-departure-beside-star-departure
kind: issue
title: boolean/fate.rs leaving reads a half-edge's departure itself, beside star::departure
status: closed
opened: 2026-11-10
priority: P2
cost: C
closed: 2026-11-16
pr: 7014
branch: onehome/fate-departure
---


Found by `run-loops-of-split-and-pierce-are-twins` stage 2 (`onehome/star-walk`). Its sweep looked for every reading of an orbit half-edge's departure (`ConicDeparture::of`).

`topo::star::departure` (`star/walk.rs`) is now the one reading of where a half-edge leaves its vertex. The star's walk, the split, `census::departure` and the Boolean all take it.

`boolean/fate.rs` `leaving` (called by `fate::departure`) still reads it itself:
- a line's chord;
- a curved edge's `ConicDeparture::of`, with its bend where `BendBounds::Planar`;
- or a secant to the curve's midpoint.

Its reach is a `LeverBounds` from `cover::edge_reach_bounds`, not the edge's extent, and it returns `Option` where the star returns a `StarFault`.

Owed: decide whether `leaving` reads `star::departure` (its direction and its `BoundSupport`) and keeps only its secant arm and its reach bounds, or say why a fate cut's departure is a different question.

## Closed (2026-11-16)

`leaving` reads `star::departure`. The tangent and the bend are the star's: a
line's chord with a zero bend, a curved edge's departure tangent with its bend
where `BendBounds::Planar`, read off `BoundSupport::Conic`. The two readings
were the same arithmetic on the same inputs (the chord `far − base`, and
`ConicDeparture::of(curve, he_plus, (far, base))`), so the tangent is bit for
bit what `leaving` read itself.

Two things stay the cut's own:
- the secant arm, which tells apart two curves leaving one point tangent to
  each other;
- the reach, a `LeverBounds` off the edge's cover
  (`cover::edge_reach_bounds`). The star levers at the edge's extent from
  below; a fate cut's corner sign reads both bounds of its reach
  (`decide_levered`), a different question from the star's.

`leaving` keeps its `Option` for null scaffolding: it returns `None` before
asking the star, so the star's one refusal there, `StarFault::NullEdge`, does
not arise.
