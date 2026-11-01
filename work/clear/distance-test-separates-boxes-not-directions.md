---
id: distance-test-separates-boxes-not-directions
kind: issue
title: the distance arm separates axis-aligned boxes, so two near-parallel walls tilted against the axes need cells about as small as their gap; a support along a direction would need cells of about sqrt(gap x radius)
status: open
opened: 2026-11-01
priority: P0
cost: M
---


Found by the PRICE designers (2026-11-01). On the Klein loop the outer and
inner walls are 0.05 m apart over a 2.8 m body.

- **The box test.** The distance arm's axis-aligned box needs cells about
  as small as the gap wherever the walls tilt against the axes.
  - 3v3×6v1 and 4v1×5v1 run to depth 21–27.
  - 3v3×5v1 and 4v1×6v1 hold in 0.5 s at the same gap, so the cost
    follows the walls' orientation, not the geometry.
- **The proposal.** An oriented support: the hull of ⟨P_i, n⟩ over the
  restricted net's dehomogenized control points, along the cell's centre
  normal. That needs cells of about √(g·ρ) instead of about g, so about
  36× fewer pairs on the Klein loop.
- **Prototype data.** At 2^5 cells per axis the support separates the gap
  pairs fully, where the box needs 2^7.

This is the second lever after `rect-jet-forms-the-rational-derivative-in-the-absolute-frame`.
