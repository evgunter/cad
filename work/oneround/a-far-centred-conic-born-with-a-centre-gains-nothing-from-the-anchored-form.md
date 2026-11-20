---
id: a-far-centred-conic-born-with-a-centre-gains-nothing-from-the-anchored-form
kind: issue
title: A conic born with a far centre (an import, an authored arc, every circle mint) evaluates in its centre form and gains nothing from the anchored form
status: open
opened: 2026-11-20
priority: P3
cost: M
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Filed by `oneround/anchored-conic` (2026-11-20).

A conic is stored in the form it was born in, and each form has its
own evaluation: a conic born with a centre is
`center + u_ref·(a·cos θ) + v_ref·(b·sin θ)`, bit for bit as before
the anchored form existed. So a far-centred conic that was born with
a centre still sums its points from terms as long as its semi-axes.

Who is born with a centre: every import (`step-import`), every
authored arc and circle, and every circle the kernel mints (no circle
mint is converted: `a-far-circle-has-no-re-seaming-door`). Of the
section tables only `plane_cone_section`'s ellipse is born anchored.

Measured on the ρ = 1500 near-gap families
(`in_band_sweep::the_near_gap_families_certify_by_their_stored_form`):
born with their centre, 7, 7, 7, 7 and 1 of 40 certify, the meter's
bound 3.8 `u·ρ`; born anchored at the graze point, 40 of 40 in every
family.

What giving such a conic an anchored reading would take, measured in
phase 1 and in this unit:
- Summing its zero point `c + u_ref·a` in the chain: the meter's
  bound 0.77 `u·ρ`, 40 of 40. It costs every such conic its
  quarter-turn points to the bit (`c + r` is rounded into the zero
  point and `−r` does not undo it): with EVERY conic evaluated so, the
  STEP round trip lost its byte-for-byte fixed point by one ulp
  (`freecad::cross_dialect_fixed_point`), 19 of 48 mesh digests moved
  and a carried certificate stopped equalling a fresh one. That is why
  the chain follows the born form.
- Summing the zero point exactly: 0.004 `u·ρ`. It needs a
  product-with-residual door on `Real` at every scalar, gives nothing
  at `Interval`, and does not give back the quarter-turn bits.
- Re-minting it anchored at import (Ev's "anchored form as an internal
  cache", PR 4812): possible for a conic whose centre is past the box,
  where the quarter-turn points are not in the model anyway. Whether
  any import produces one is not surveyed.

The lists of what moved under one chain for every conic are on the
branch `analysis/oneround/conic-vertex-probes`, under
`analysis/oneround/conic-vertex/phase2-one-chain/`.
