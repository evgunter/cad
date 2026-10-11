---
id: a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check
kind: issue
title: A crown whose ring touches the split plane at one corner panics at the join's section-spur check
status: dispatched
opened: 2026-10-10
priority: P1
cost: M
branch: cleave/crown-spur-panic
---

Found by the re-review fuzz of PR 4565, on main before it and on its
head alike.

## What

`topo::split` panics in the join's section-spur check
(`splitting/join.rs`, the `split_section_spur` reading's `Sign::Zero`
arm: "section polygon … runs out along a touching edge and back"). It
does so on exact, valid input: a `meeting::posed_crown` whose ring has
one corner exactly at `MEET`'s height and the rest above it, split by
the plane through `MEET`, at every pose and both normals (80 cells in
the fuzz). That is a D9 panic on valid input.

Witness C2: `posed_crown` with bottom (0, 0, −1) and ring
(0.4760, 1.3181, 0.3518) (-0.7080, 0.8592, 0.1597)
(-1.1890, 0.5653, 0.1028) (-1.1262, -0.2072, 0) (0.4166, -1.4394, 0.2963),
the plane through `MEET` with normal +z, at rest.

The plane touches the crown's top fan along the one edge from `MEET` to
the corner at height 0, and the section polygon runs out along it and
back, which the `unreachable!` argues no contact can mint. Not traced
further.

## Neighbour

`work/emit/split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge.md`
(closed, PR 3133): a plane tangent along an edge and crossing elsewhere
kept the edge as a zero-area spur. That fix's guard is the arm this
witness reaches.

## Owed

A row over C2 at every pose and both normals, at all three ε, that
builds sound or refuses typed. Then the arm's argument re-made, or the
arm made a typed refusal if the input class is honestly unbuildable.
