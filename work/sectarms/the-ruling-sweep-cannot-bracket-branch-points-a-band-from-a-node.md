---
id: the-ruling-sweep-cannot-bracket-branch-points-a-band-from-a-node
kind: issue
title: The ruling sweep escalates where two branch points stand within a few bands of a node: its cuts halve toward the node and never bracket them
status: open
opened: 2026-11-04
priority: P3
cost: M
refs: [cylinder-sphere-reads-its-pose-on-the-ruling-sweep]
---



Filed by the cylinder × sphere ruling-sweep lane (branch
`sectarms/cylinder-sphere-ruling-sweep`, 2026-11-04).

## What

`pair_section::ruling`'s discriminant recursion (`Sweeper::discriminant_cell`,
`Sweeper::split`) cuts an undecided cell near its middle, at the first
of `off_a_zero`'s points (the middle, then ±⅛ and ±¼ of the width) where
`Δ` decides. A branch cell needs `Δ` decided on both of its ends and its
slope decided over it. Where the two branch points stand within a few
bands either side of a node of `Δ` (an extreme of `Δ` within a few bands
past zero), take the cell from the node's ruling. Its slope's lower
bound is zero at that ruling, so the cell is never monotone. Every cut
the ladder offers lands where `Δ` is in the band, until the halving
reaches the node, and the cell ends at the floor and escalates
(`ruling_discriminant`).

## Witness

Cylinder `r = 0.5` about `z` and a sphere at `(0.25, 0, 0)` of radius
`0.75 − 5.5e-9`, at the band `(4.95e-10, 4.95e-9)`. The far reach is
`−5.5e-9`, so the loop is a figure-eight's lobes 1.1 bands short of its
node. `pair_section` escalates on `ruling_discriminant`, and the pose
decides one loop. `topo::boolean::offer_rows`'s
`transverse_frame_reach_short_of_tangency` pins this as the reason its
offer stays withdrawn. Before the pair read its count on the sweep, the
same pose refused later, at the march (`TubeStraddles`).

From the band arithmetic, the zone covers a far reach up to about three
escalation widths past zero. The cylinder pair's sweep shares the
recursion, so its near-node poses are expected to meet the same limit.
That has not been measured.

## What the change could be

When the ladder finds no cut that decides, also try cuts
geometrically close to each end (`lo + w·2⁻ᵏ`, `hi − w·2⁻ᵏ`). These
find a decided point between a node and its branch point, after which
the cell beyond is a branch cell. Every new cut is still a decided
reading, so this only turns escalations into answers.
