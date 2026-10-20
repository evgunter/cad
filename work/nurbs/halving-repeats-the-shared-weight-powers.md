---
id: halving-repeats-the-shared-weight-powers
kind: issue
title: cell_pieces halves the shared weight powers once per quotient: W² twice and W³ four times per cell, identical bits
status: closed
opened: 2026-10-14
closed: 2026-10-19
---


## What

Since PR 5081, `geom_core::spline::derivative`'s `cell_pieces` halves
each quotient's numerator and weight power together
(`Raised::pieces`). `S_u` and `S_v` share `W²`, and the area and the
three second partials share `W³`, so each cell halves `W²` twice and
`W³` four times, every copy bit for bit the same. The retired
`cut_both` cut each weight power once. The FULL review of 5081 (its
NOTE) estimates this at about 17% of the halving work on a bicubic
cell, so about 8% of `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian`
(halving is about 49% of it by callgrind). That is an estimate, not
measured.

## Shape

Halve each weight power once per cell and pair its pieces with every
numerator that shares it. The step-3 cut
(`nurbs/cut-becomes-restriction`) already restricts the shared `W²`
once (`CellFirstQuotients::restrict`), so the halving side is what
remains. Bit-identical by construction: the copies are clones. A row
counting weight-power halvings per cell would pin it.

## Closed

`shared_pieces` halves a weight power into its pieces once and pairs
them with every numerator over it; `cell_pieces` halves `W²` once for
`S_u` and `S_v` and `W³` once for the area and the second partials, and
`Raised::pieces` is its one-numerator case. Bit-identical: halving is
deterministic and every `W³` is `W²·W` of the same `W`. Rows:
`a_cell_halves_each_weight_power_once` (1 / 2 / 2 per cell) and
`a_shared_weight_halving_is_each_quotients_own_to_the_bit`. Measured
on `rational_face_bound_dominates_sampled_hessian`: 13.41 G → 11.92 G
instructions (−11.1%), all of it in `cell_pieces`; the shelling and r1
rows' wall times are unchanged within noise. `CellFirstQuotients::pieces`
and the cut's both-axes `cut_pieces` still halve the restricted `W²`
per axis; the cut lane switches them to `shared_pieces` after #5391.
