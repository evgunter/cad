---
id: the-spiric-winding-lever-decides-real-pinched-loops-zero
kind: issue
title: The spiric winding's length lever is 1e3-1e4 times the true length near the pinch, so small real loops read a decided Zero
status: open
opened: 2026-10-13
priority: P3
---


## What

The planar winding reads a cycle carrying a spiric arc at
`Margin::over_lever(area, perimeter)` (`crates/topo/src/loop_winding.rs`,
`spiric_winding`). The spiric arc's share of that perimeter is
`geom_brep`'s `spiric_length_bound`:
`r·√(1 + (d·sin_max/f_lo)²)·span`. It takes the span's sup `|sin|` and
its inf `f` separately. Near the oval's pinch (a plane just short of the
inner equator, where `f → 0`), that is 10³–10⁴ times the arc's true
length.

## Measured (PR 4927's delta review, d4927)

3 000 fuzzed loops on constructible carriers were read against an
mpmath oracle (two independent formulas, dps 40/50). None had a wrong
definite sign. But 10 were decided **Zero** although their true
|area| / true perimeter is 1e-8 to 4e-6 m, because the inflated lever
puts their margin under 1e-9.

This is sound by the lever's contract, but a decided Zero is not
harmless:
- `ring_winding_order` desyncs on it (`crates/topo/src/boolean/join.rs`, about :3945);
- `loop_sense` reads Undecided;
- tier 3 check 6 exempts a Zero winding. So an inverted tiny pinched
  loop would pass check 6 rather than fail it.

The looseness predates PR 4927: the old full-period bound was the same.

## Done when

The lever reads the arc's length to within a small factor near the
pinch: for example, bound the length piecewise over the enclosure's
own pieces, with `sin` and `f` read together on each. Add a row with a
small loop near the pinch that winds decided, and one that
`spiric_length_bound` still bounds from above.
