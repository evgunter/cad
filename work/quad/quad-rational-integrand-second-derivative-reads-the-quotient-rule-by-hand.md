---
id: quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand
kind: issue
title: The rational flux lane's f_dd of N/w³ is the interval quotient rule over hulls; the quotient statement reads it per piece
status: closed
opened: 2026-10-13
priority: P1
cost: M
closed: 2026-10-22
pr: 5582
---


## What

Found by NURBS's sweep for rational second-derivative recipes (the
quotient ruling's unit 2, `work/nurbs/log.md`). The rational flux lane
bounds its integrand's second derivatives by evaluating the quotient
rule in interval arithmetic over cell hulls:

- `crates/geom-brep/src/props/quad.rs:3160` `quotient_second`:
  `f_dd = N_dd/w³ − 6·N_d·w_d/w⁴ − 3·N·w_dd/w⁴ + 12·N·w_d²/w⁵`, each of
  the six inputs a separate hull, so the terms lose their correlation.
- `crates/geom-brep/src/props/quad.rs:3762`: the pad's
  `|∂_d cross_num|/w³ + 3·|cross_num|·|w_d|/w⁴`, the first-order
  triangle-inequality form of the same shape.

Both quantities are quotients with polynomial numerators over a power
of `w`: `f_dd = M/w⁵`, `M = N_dd·w² − 6·N_d·w_d·w − 3·N·w_dd·w +
12·N·w_d²`. `geom_core::spline::derivative`'s `QuotientPiece` reads
such a quotient on exact dyadic pieces (raise both to one degree; a
convex combination of `m_k/b_k`), and since unit 2 that statement
carries every rational first- and second-order bound in
`patch_bound`, `mesh::chords` and `offset_meters`.

## Shape of the fix

Not a soundness finding: the interval quotient rule is sound, only
looser. Whether QUAD wants the quotient statement here is QUAD's
call. The cost side is the degree: `N` is already a triple product, so
`M` sits near `5p` per direction, against `QUOTIENT_MAX_DEGREE = 56`.
A row comparing the two readings' `f_dd` widths on
`rational_quarter_cylinder_brackets_the_closed_form`'s face would
size the gain before anything moves.

## Closed

PR 5582. `geom_core::spline::derivative::cell_measure_quotients` reads, per span cell in the cell's own frame:
- the area element `M̃/W³`;
- its slopes;
- the flux's second partials, as quotients on exact dyadic pieces.

`area_on` and `flux_second_on` restrict them to a quad cell. The rational lane's hand-written interval quotient rules are deleted. Floors fell 5–1430×, the quarter torus and the two-span half cylinder certify at the default ε, and the area bracket no longer moves under translation.

Soundness fix: the flux midpoint rule is read at an interval around the exact midpoint, clamped to the cell, and a cell's span is known by construction (`span_cells`). Dual review (DR row PR-5582): no unilateral MAJOR; both MAJORs fixed and pinned.

Residue rows:
- `the-integral-composite-reads-its-midpoint-rule-at-a-rounded-midpoint` (P1; not reproduced by either reviewer);
- `quad-rational-position-bound-is-a-quotient-of-hulls` (P3);
- `quad-rational-area-reads-the-statement-per-hull-block` (P3);
- `an-integral-patch-with-knots-hugging-its-edges-trips-the-area-gauge` (P2, pre-existing);
- TCOST's `curves-review-m5-pr4-f1-costs-50x-by-seed`.
