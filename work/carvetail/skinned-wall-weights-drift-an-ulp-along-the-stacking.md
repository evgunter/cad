---
id: skinned-wall-weights-drift-an-ulp-along-the-stacking
kind: issue
title: a skinned wall's weights differ by an ulp along the stacking where every section's are equal, so its interior row is not exact structure
status: open
opened: 2026-10-08
priority: P3
cost: E
refs: [plane-nurbs-limb-two-refuses-every-rational-wall]
---


Filed by SHELL's `shell/oblique-corner-derives` lane (unit 9).

## Measured

The vase (`crates/sweep/tests/encl_curved_loft_shell.rs`'s `vase`:
circle sections of radius 1, 1.3, 1 at z = 0, 1, 2, lofted at degree 2)
skins two walls with a 5 × 3 net. Every section's arc has the same
weights (1, √2/2, 1, √2/2, 1), so in ℝ the net's weights are constant
along `v`. Stored, the middle column of the second weight row reads
**0.7071067811865477** where the other two read **…476**: the skin's
interpolation along `v` rounds the middle control row's weight.

`geom_brep::interior_iso_u`'s separability test is bitwise on stored
structure (`nurbs_iso.rs`), so it refuses `WeightsNotSeparable` on the
wall's row at any `v`, and the row the moved cap's plane cuts is not
exact structure. SHELL's section lane then marches instead of taking
the row (`topo::offset_derive`'s `level_row`).

## Shape of a fix

Where every section's weights are bitwise equal, the skin copies them
rather than interpolating them, so the net's weights are constant along
`v` by construction.
