---
id: nurbs-cert-spends-the-bound-twice
kind: issue
title: nurbs_cert's Q/4 certificate spends the second-order bound twice via an affine detour - a direct Q/8 argument halves NURBS triangles at the same certified delta
status: open
opened: 2026-09-10
---

## The finding

Reported by the PERF-1 lane (PR 2307), outside its fence. `nurbs_cert`'s
per-cell certificate is `Q/4`: it bounds the affine interpolant by a
detour through the cell centre, which spends the second-order bound
twice. PERF-1's derivation for the torus — Taylor with the integral
remainder at each point of the triangle, the first-order term
cancelling, Popoviciu plus Cauchy–Schwarz on the mixed sum — gives
`Q/8` for ANY C² surface with the same per-cell sups, and it is
attained (the torus row `certificate_is_attained_on_the_outer_equator`).
At the same certified δ that is ~2× fewer NURBS triangles; the
tess-budget sweep carries 198 770 NURBS triangles today.

## What a fix is

"Do this faster" of the same shape as PERF-1: re-derive the NURBS cell
certificate as the direct per-point bound, re-pin `nurbs_cert`'s rows,
re-cut the tess-budget baseline deliberately (its NURBS rows are
byte-identical after PERF-1, so a move here is attributable to this
alone). S-MESH territory; numeric; full dual.
