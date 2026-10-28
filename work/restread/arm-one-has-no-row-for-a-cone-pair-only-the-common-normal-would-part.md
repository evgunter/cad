---
id: arm-one-has-no-row-for-a-cone-pair-only-the-common-normal-would-part
kind: issue
title: Census arm 1 dropped the two axes' common normal on an argument no cone row witnesses
status: open
opened: 2026-10-28
priority: P3
cost: E
refs: [census-arm-1-reads-directions-from-the-pairs-own-faces-only]
---



Found by PR 5858's verifier.

## What

PR 5858 removed `a × b`, the common normal of two carriers' axes, from
`census::depth_gap::across`. Its argument: a cylinder's seam rulings
run along its axis, so `toward`'s offset from the other axis to the
nearest ruling point already is the common normal. A cone's rulings
do not run along its axis, so the argument does not cover cone pairs.

The verifier measured cone × rod and cone × cone pairs (4×4 seam spins,
clamped, oblique and crossing rods, gaps 0 to 0.1). The head and the
head with `a × b` restored clear the same 251 of 269. With `toward` off,
`a × b` adds clears (83 → 155), all of which `toward` already covers.
So no regression was found, but no committed row pins it:
`cross_solid_apart` holds no skew, crossing or cone pair, and the
crossing-rods row was dropped with `a × b`.

The probe source is the verifier's `ver5858_probe.rs` (scratch, not
committed).

## Owed

Land a cone × rod and a cone × cone row in `cross_solid_apart` that
clears only past the pads, and a skew-rods row. With them, the
redundancy argument in `across`'s doc is guarded rather than prose.
