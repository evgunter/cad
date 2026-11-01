---
id: departure-graph-separator-weighs-each-heading-by-its-cells-offset-along-the-edge
kind: issue
title: The departure arm's graph separator takes its normal across two headings normalised before their along-edge part is dropped, so for two cells offset along a seam the farther face weighs less and, at an acute dihedral, both faces read one side: every seam of the tour's twisted_loft refuses at depth
status: dispatched
opened: 2026-11-01
priority: P0
cost: E
---


## Finding (SEAMDIAG, 2026-11-01)

The twisted-seam locus of
`injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams`.

`topo::injective`'s `graph_of` (`crates/topo/src/injective.rs`) takes
the graph separator's normal as `m = unit(perp(hx, t) − perp(hy, t))`.
Here `hx` and `hy` are `heading`'s unit vectors from the shared edge's
middle toward each presented rectangle's middle, and `t` is the edge's
tangent. Each heading is normalised before its along-edge part is
dropped. So when the two cells are offset along the edge, the face
whose rectangle middle lies farther along `t` keeps the shorter
perpendicular. That happens for every pair `(v_k, v_{k+1})` of
adjacent cells across a seam, which touch at their common end.

`m` separates the faces when `m·hx⊥ > 0 > m·hy⊥`. With unequal
lengths, `m·hy⊥ ∝ cos θ |hx⊥||hy⊥| − |hy⊥|²`, so it fails once
`cos θ > |hy⊥|/|hx⊥|`, where `θ` is the dihedral between the faces'
perpendiculars.

At `twisted_loft`'s seam `3v1 × 4v1`, the depth-40 cells are
`u ∈ (0.99902, 1) × v ∈ (0.60840, 0.60938)` on `3v1` and
`u ∈ (0, 0.00098) × v ∈ (0.60938, 0.61035)` on `4v1`. Both present
with one zero side (`XHi` and `XLo`). Neither face has an implicit, and
`plane_through` has no line or conic. There `|hx⊥| = 0.586`,
`|hy⊥| = 0.239` and `cos θ = 0.632` (about 51°), so `sign_on` reads
`Positive` on both faces (`across U` is `−0.00213` into `XHi`, and
`+0.00030` into `XLo`), and `opposite` fails.

`s_duct`'s seams are near 90° (`cos θ ≈ 0`), so the condition never
fails and they hold at depth 6. The shared iso being curved in space
plays no part, and neither does the dihedral's variation along the
seam beyond what it does to `cos θ`. Nor is the dihedral near π.

## Fix shape

Normalise each perpendicular before subtracting:
`m = unit(unit(perp(hx, t)) − unit(perp(hy, t)))`, the bisector, for
which `m·ĥx⊥ = sin(θ/2) > 0 > m·ĥy⊥` at any dihedral off 0. It goes in
`graph_of`, and in `plane_through`'s one-line-edge case
(`[d] => unit(perp(hx, *d) − perp(hy, *d))`), which has the same shape.
`heading` chooses the separator and decides nothing (its own doc), so
nothing new needs arguing: `sign_on` still decides both signs. Two
lines, plus rows.

**Measured** with that change applied locally to `graph_of` alone, all
four `twisted_loft` seams `Hold`, at depth 21–24 in 0.17–0.23 s each,
down from refusing at depth 40 in 5–8.4 s. `s_duct`'s seams are
unchanged (depth 6). The pinned row that flips is
`a_twisted_loft_refuses_departure_along_a_seam_at_both_scalars` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`.

It shares nothing with SLIVER: these are cross pairs, and the sliver
knots are a wall against itself. The whole-body `twisted_loft` (43 s)
and `twisted_tube` refusals in the price row are likely mostly this.
