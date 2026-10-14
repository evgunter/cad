---
id: a-fin-united-with-a-pyramid-across-its-fold-leaves-a-region-winding-minus-one
kind: issue
title: A fin united with a pyramid across its fold builds a result with a region winding −1, which check 7's aimed ray reads inside out
status: open
opened: 2026-10-14
priority: P1
cost: M
---


Found by FLUSH's shell-role reader unit (`flush/shell-role-reader`), at
ε 1e-9.

## Measured

`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`
`every_edge_a_vertex_read_again_reads_is_classed_against_the_germ`:
the fin (`test_support_meeting::fin`, two faces folded 1e-4° apart)
∪ the probe pyramids 2 and 4 (bearings 130° and 250°, rise −0.3) builds
on main at all four poses, and its result passes main's check 7, whose
volume sum is positive. On the branch the door refuses all eight
(`x ∪ y` at four poses, two probes) as
`ResultInvalid { [NegativeVolume { solid }] }`; the other orders and
ops are unchanged.

The ray that reads it (probe 2, at rest), aimed at 60° at the fin face
`FaceKey(1v3)` from twice the result's reach beyond it, meets, in order
of advance:

| face | advance | outward sign |
|---|---|---|
| 15v1 | 1.367276727 | exits |
| 7v3 | 1.367277191 | enters |
| 1v3 (aimed) | 1.367284501 | enters |
| 4v3 | 1.367284967 | exits |
| 8v3 | 1.367399225 | enters |
| 12v1 | 1.509149585 | exits |

Its start lies outside a ball holding every face of the result, so it
winds 0, and the winding along it runs 0, −1, 0, 1, 0, 1, 0. Between
faces 15v1 and 7v3, two planes 4.6e-7 apart along the ray (≈ 460ε,
definite), it is −1: the union leaves a sliver whose faces are wound
inside out. The volume cannot see it, being a sum.

## What to find

Which step of the join leaves faces 15v1 and 7v3 wound against each
other across the fold, and whether the result was meant to hold that
region at all. The test file lists the two scenes in
`KNOWN_INSIDE_OUT`, so the refusal is pinned until this row lands.
