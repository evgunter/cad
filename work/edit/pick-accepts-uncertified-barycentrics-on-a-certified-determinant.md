---
id: pick-accepts-uncertified-barycentrics-on-a-certified-determinant
kind: issue
title: the exact test accepts barycentrics whose rounding error exceeds the acceptance interval when the determinant is certified but small
status: open
opened: 2026-09-16
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) certifies
the determinant and takes `t` from the hit point's projection, but
accepts `u` and `v` on their rounded values alone. When the
determinant is certified and small — a ray a few units of roundoff
out of a planar face's plane, not through a vertex — the numerators
`s·p` and `d·q` carry absolute errors of order `u·|s|·|p|` that the
division by the tiny determinant magnifies past the width of the
acceptance interval: `u` and `v` land in `[0, 1]` by chance, the
projected `t` is the parameter of a point that is not the crossing,
and the candidate can win.

Measured at the fix pass of
`pick-grazing-ray-answer-depends-on-candidate-order` with a derived
bound on the barycentric error (`(err_N + |u|·bound_det) /
(|det| − bound_det)`, `err_N` from the operation count): over the
tie-break row's aim at every landing of the corpus and the ring,
44 of 36 578 winners and 340 of 247 483 accepted candidates carry a
barycentric error bound of 1 or more (125 winners at 1e-2 or more);
over the wide aim (`review_pick_r2`, 441 126 rays), 203 winners and
4 014 accepted candidates at 1 or more (1 394 winners at 1e-2). One
on the ring after its bump: the −y ray through
`(0.24519632010080758, ·, 0.04877258050403218)` answers the flat face
at `t = 1.4488` — 0.03 before the aimed vertex at `1.48` — from
`u`, `v` with an error bound near 100. `main`'s kernel answered the
same class with its own noise (`1.4694` there), so nothing regressed;
nothing closed either.

The fix is a ruling before it is a unit, because the three shapes
change the closed-boundary contract differently:

- **certify inside** (`u − err ≥ 0`, `u + err ≤ 1`, and the same for
  `v`): refuses every genuine vertex and edge graze, whose true `u`
  or `v` is `0` and cannot be certified either side — the tie-break
  row's whole class;
- **accept the interval** (`[u − err, u + err] ∩ [0, 1] ≠ ∅`): the
  closed contract with its rounding made explicit, which also
  answers `pick-closed-acceptance-loses-a-graze-to-rounding`, and
  which the spec's trap names because it moves ULP-level tie
  behaviour;
- **refuse the uninformative** (the interval covers the whole
  admissible range): no constant, but a candidate whose interval
  reaches outside on one side only is still accepted on a rounded
  value.

Whichever is ruled, the bound is derived at the site from the
operation count, never tuned, and the rows that pin it are the
`index_memo` differential and `review_pick_r2`'s tally.
