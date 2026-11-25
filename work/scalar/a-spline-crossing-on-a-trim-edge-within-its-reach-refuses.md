---
id: a-spline-crossing-on-a-trim-edge-within-its-reach-refuses
kind: issue
title: A spline edge's crossing whose reach holds a trim edge refuses, where an edge-edge meeting over the reach would place it
status: open
opened: 2026-11-24
---


Found by the crossing-parameter unit (`a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge`), 2026-11-25. BY-INSPECTION of the reader it built; the row `contain::reach_rows::a_shallow_crossings_containment_answers_one_way_or_refuses` pins the refusal.

`topo::contain::placed_within_reach` reads a spline edge's crossing against a face's trim for every parameter the crossing could be stored at: an interior or exterior answer holds where the point is off every boundary cell by the band's escalation plus the reach, and a boundary answer (`OnEdge`, `OnVertex`) holds only where the reach is inside the band's zero and the point reads zero with the zero moved in by it. So a crossing whose reach is not inside the zero, landing within the reach of a trim edge, refuses (`Unsettled::Reach`, or an escalation naming the widened band). Its three callers are `boolean::reduce`'s planar root loop, `spline_wall_event` and `spline_interior`.

That is the allowance reading of DL5.2. The other reading the rule offers builds more: ask the model whether the TRIM EDGE lies within the band of the carrier over the crossing's bracket (an edge against edge meeting, `geom_brep::curve_pair`), and if it does, store the meeting's parameter instead of the crossing's, itself admitted with its own reach along both edges. What it costs: the pierce arms of `reduce.rs` call the curve-pair meeting on a sub-span and re-pose the split point, and that meeting's own readers (the split's interiority on both edges) take its reach.

Who is exposed today: a crossing at `f64` on a well-conditioned form reaches `10⁻⁶` zero bands or less, so its boundary verdicts stand; a thin torus, a carrier far from the chart's origin, or a leaf whose family spreads past the zero band refuses at a trim edge.
