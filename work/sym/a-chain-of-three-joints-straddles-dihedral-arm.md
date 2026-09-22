---
id: a-chain-of-three-joints-straddles-dihedral-arm
kind: issue
title: the chain's certifiable box is bounded at 0.111 of its study by a dihedral_arm enclosure [0, 7.34e-3] straddling the band during a mapped edge's re-certification
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, `Sym<Interval>`, on the chain of
`demos/tour/src/chain.rs` at σ = 0.01 rad per joint. At THREE and at
FOUR links:

```
node 12 — the transform op refused: transform: mapped edge EdgeKey(2v1)
failed re-certification: certification: the transversality margin at
sample 1 escalated: predicate 'dihedral_arm' indeterminate: enclosure
[0e0, 7.338367397126071e-3] cannot be classified against the band
(zero = 1e-9, escalate = 1e-8)
```

This is what bounds `chain::CERTIFIABLE_FRACTION` to `0.111`: narrow
the study and the same leaf certifies whole, and the four-link tip's
assertion is then certified with the enclosure per joint drawn on the
sheet. So the answer EXISTS and this is the width it stops at — the
plate's ceiling in the same shape, at six orders of magnitude more
width (the plate's is `7.81e-7`).

The enclosure's LOWER end is exactly `0e0`, which is the shape of a
dependency-widened margin around a structurally-zero quantity rather
than of a real sign flip: a bar's cap-to-side dihedral is a right
angle at every joint angle, and the transversality arm at that rim
does not actually approach zero as the joints move.
The plate's own ceiling is the same class, and its cell's header
carries the worked version of this argument: there the margin was
affine, its true range was positive everywhere, and the real flip
first entered the box well beyond the width the enclosure stopped at.
(That row itself is gone with its program; `demos/tour/src/tolerance.rs`
is where the argument survives.)

## What answers it

The true range of `dihedral_arm` at that rim over the whole study,
measured — if it is positive everywhere, this is dependency widening
and the fix is the tier's (a tighter form for the arm, or a registered
identity for the right-angle rim the extrude's own builder guarantees),
and `CERTIFIABLE_FRACTION` moves with it. If it genuinely reaches zero,
the ceiling is real and the row closes saying so.

## Home

SYM — the tier's reach on a construction Ev asked for.
