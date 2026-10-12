---
id: quad-rational-patch-cross-grows-with-translation
kind: issue
title: quad: the rational patch lane (rational_patch_face, Ladder::cross_num over Collapse::Over) assembles A = w·P in the absolute frame from decorrelated hulls, so a rational face's area and flux enclosures grow with its translation (area width 0.124 at 0, 7.49 at 100 m, 66.5 at 1 km)
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [a-translated-arc-prism-panics-the-area-gauge-through-mass-properties, 3863]
---


## Found (review of PR 3863, `ssi/rational-chart-speed`, 2026-10-02)

Measured by that PR's reviewer; not reproduced by the filer. PR 3863's
own §5 sweep missed it.

`crates/geom-brep/src/props/quad.rs`: the rational lane
(`rational_patch_face` → `Ladder::cross_num`, read over
`Collapse::Over` windows) forms the homogeneous net `A = w·P` from the
control points as given and assembles the area element's cross
product from separately-hulled terms. Each term carries the net's
distance from the origin times the weight derivative, and the hulls
subtract decorrelated, so that distance stays in the width instead of
cancelling. Its caller, `crates/topo/src/props/quad_lane.rs`
(`trimmed_patch_face_rounds` call site), passes `payload.control()`
unshifted; the `center` that file subtracts elsewhere is the
circle/ellipse trig bracket's, not a recentring of the net.

On a cubic × linear wall weighted 1.8 / 0.7 (the `rational_wall`
fixture in `crates/geom-brep/src/ssi/enclose.rs`'s tests), translated:

| translation | area enclosure width | flux enclosure width |
|---|---|---|
| 0 | 0.124 | 0.023 |
| 100 m | 7.49 | — |
| 1 km | 66.5 | 2.04e5 |

`nurbs_patch_face` refuses `QuadratureBudget` from 1 m.

## What is open

Assemble the cross from translation-free terms, either a cell-local
origin for `A` (as `patch_bound`, `mesh::chords` and `mesh::nurbs_cert`
do) or per-pair control-point differences (as
`NurbsBoxes::cell_quotient_numerator` does after PR 3863), so a
translation moves the area enclosure by its rounding width. The flux
integrand itself depends on the origin, so its width should be judged
against the flux's own magnitude after the fix. Settle whether
`a-translated-arc-prism-panics-the-area-gauge-through-mass-properties`
is this degradation.

## A candidate home for the fix (ENCL `encl/one-chart-speed`, 2026-10-12)

`crates/geom-brep/src/patch_bound.rs` now holds one per-cell rational
quotient-numerator form, `QuotientForm`, which `rational_cells` (the
O3 meters) and `ssi::enclose`'s `CellNet` (the SSI chart speeds) both
read. It is `S_d`'s numerator about a cell-local centre `c`: the
terms `k_i − g_i·q`, with `k_i` the coefficients of
`Ã_d = A_d − c·w_d`, `g_i` those of `w_d`, and offsets
`q_j = P_j − c`. No term holds an absolute coordinate. That is the
translation-free assembly this row's "What is open" asks for, and
`props/quad.rs`' `Ladder::cross_num`
(`w·(A_u×A_v) − w_v·(A_u×A) − w_u·(A×A_v)` over `A` in the absolute
frame) is a candidate consumer. `S_u × S_v` is the cross product of
two such forms over `w²`, and the `(S − c) × (S − c)` term is
identically zero. `patch_bound`'s own `m_sup` bound (the field's
docs) already reads it that way, though through the triangle
relaxation. `cross_num_u`/`cross_num_v` would need the forms' second
partials as well, which `QuotientForm` does not carry yet.
