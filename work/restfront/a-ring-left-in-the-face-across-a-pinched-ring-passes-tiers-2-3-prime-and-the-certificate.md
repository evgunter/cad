---
id: a-ring-left-in-the-face-across-a-pinched-ring-passes-tiers-2-3-prime-and-the-certificate
kind: issue
title: A ring left in the face across a pinched ring from the region holding it passes tier 2, tier 3′ and the certificate; only the volume sees it
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Found by PR 6177's re-review (CLEAVE), planting a fault in the joiner.

## The witness

`crates/sweep/tests/a_cylinder_through_a_hole_tangent_to_its_wall.rs`
`a_cylinder_through_a_hole_tangent_to_its_wall_beside_other_holes`,
first row: the cylinder `r = 0.3`, `z ∈ [−1, 1]`, through the plate
`[−1, 1]² × [−0.5, 0]` holed by `r = 0.1` about `(0.4, 0)` (tangent
outside the wall, both rims' vertices on the ruling through `(0.3, 0)`)
and besides by `r = 0.05` about `(−0.15, 0.05)`, inside the wall, and
`r = 0.1` about `(−0.6, 0.5)`.

The plate's top and bottom are each divided by the wall's section
circle from a vertex of the tangent hole's rim round to itself
(`chord_join.rs` `ChordJoiner::place_strut`). The face outside the
circle keeps one pinched ring, the tangent hole's rim and the circle
through their shared vertex, and the disc inside the circle should take
the small hole's ring (`rehome_rings`).

Planted fault: both `rehome_rings` calls in `ChordJoiner::join` /
`place_strut` removed, so the small hole's ring stays in the face
outside the circle, inside its pinched ring. `A ∪ B` then builds a body
that passes tier 2, tier 3′ and the geometric certificate:

`OK BAD t2=true t3p=true cert=true operand=true v=2.388772091 want=2.392699082`

The volume is short by `π · 0.05² · 0.5`, the small hole's prism, and
only `differential::outcome`'s closed form caught it (the review's run,
`~/.local/share/cad-work/rev6177r/scratch/plant.log`).

## What it needs

A face whose ring lies inside another of its rings holds no region
there: tier 3's nesting statement (check 9) should refuse it. Measure
whether check 9 passes over the pinched ring (the hole's rim and the
circle in one loop, through their shared vertex) or reads the inner
ring as nested correctly, and close whichever gap lets it through.
