---
id: halving-repeats-the-shared-weight-powers
kind: issue
title: cell_pieces halves the shared weight powers once per quotient: W² twice and W³ four times per cell, identical bits
status: open
opened: 2026-10-14
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
