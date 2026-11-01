---
id: a-fate-corner-inside-a-curved-edge-of-its-faces-division-refuses-as-a-placement-limit
kind: issue
title: A fate cut's pierced corner inside a curved edge of its kept face's division refuses as a placement limit; the side reading takes lines only
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
---


Found by the fix of
`a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece`
(branch `cleave/fate-cut-end-on-a-division-edge`, PR 6237); its
witness was found by that PR's review and re-measured by its fix pass.

## What

`fate::piece_beside` reads which piece of a kept face holds a dropped
corner whose vertex lies inside an edge of the face's division: the
edge's half-edge in each piece, read as a straight corner at the point
(`corner_holds`), its direction the edge's chord. That holds for a line
only. On any other edge (a circle, an ellipse, a spline) the tangent at
the point is not the chord, and `piece_beside` refuses
`fate::Limit::BesideNoPiece` before reading.

**Witness**, at default ε, 1e-6 and 1e-12 alike: `x` is the cube
`[0, 1]³` (`sweep::test_support::cube`) ∪ the cylinder
`sweep::test_support::cylinder_of_arcs_at(n, 0.5, (1 + 0.5/√2,
1 + 0.5/√2), z ∈ [1, 2])` for `n` = 2 or 3 arcs: its base circle passes
through the cube's corner `(1, 1, 1)` at no vertex of the circle, and
the lumps touch there only. `y` is the box `[−0.5, 2.5]²` over z
`[1, 1.5]` or `[0.5, 1]`, at rest, `x`'s records carried. `y ∖ x` above
and `y ∪ x` below refuse

    ClassificationInvariant: classification invariant violated: a fate
    cut's end on an edge of its face's division is held by no one piece
    beside it

(`Limit::BesideNoPiece`, on the section path, in `Limit`'s words). The
other four cells per `n` (`y ∪ x` and `y ∩ x` above, `y ∖ x` and
`y ∩ x` below) build tier 3 and 3′. The circle centred at `(1.5, 1)`,
tangent to the cube's edge `x = 1` at its midpoint, builds in all six.
Probe: `~/.local/share/cad-work/rev6237/cad/crates/sweep/examples/rev6237_curved.rs`.

## Owed

Pin the witness at all three ε. Then read the edge's tangent at the point
(its parameter there, `Curve3::deriv`, and the reach to each end,
`geom_brep::edge_reach_bounds` from that parameter) and pass it to the
same straight-corner reading, or show the pose cannot arise.
