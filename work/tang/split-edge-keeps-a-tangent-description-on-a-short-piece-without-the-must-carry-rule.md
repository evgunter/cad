---
id: split-edge-keeps-a-tangent-description-on-a-short-piece-without-the-must-carry-rule
kind: issue
title: an edge split keeps a TangentIntersection on a piece the must-carry rule calls under-determined, so a cut near a tangent seam's end refuses at the certificate
status: closed
opened: 2026-10-09
priority: P2
cost: M
refs: [split-tangent-chord-mints-tangency-without-the-must-carry-rule]
closed: 2026-10-12
branch: tang/split-edge-must-carry
pr: 4815
---

## Finding

Found by the §5 sweep of the PR that routed the split's tangent
section chord through `geom_brep::must_carry_over_edge`
(`split-tangent-chord-mints-tangency-without-the-must-carry-rule`).

`Body::split_edge` (`crates/topo/src/split.rs`, the geometry gate)
takes both children's specs from `EdgeCurve::split_specs`
(`crates/geom-brep/src/certify.rs`), which restricts a
`TangentIntersection` to each child by keeping its surfaces and
re-minting the witness. The rule is never asked over the child. Its
second-order margin is the sagitta `|κ_rel|·arm²/2` with
`arm = min(curvature arm, extent)`, so a short child can be
under-determined, or in band, where its parent was jet-determinate.
The certificate then refuses the child, and the split refuses through
`SplitReduceError::CrossingInsertion`.

`shell.rs`'s `along` reads `split_specs` too (the open-face rim seam);
not measured there.

## Measured

The rounded shoulder of `crates/sweep/tests/wedge_end_doors.rs`
(`shoulder(1.0)`, cut by `shoulder_cut()`), then its `y < 1` piece,
whose seam is `TangentIntersection` along `x = 0, y = 1, z ∈ [0, 1]`
(`κ_rel = 1`), split again by `z = 1 − d` (both normals) and `z = d`,
at `Tol::witness()`:

| d | outcome |
|---|---|
| ≥ 2e-4 | cuts, both pieces tier-3 valid |
| 1e-4, 5e-5 | `Reduce(CrossingInsertion { source: Certification { Escalated { check: TangentSecondOrder, .. } } })`, sagitta 5e-9, 1.25e-9 |
| ≤ 3e-5 | `Reduce(CrossingInsertion { source: Certification { NotSecondOrderSeparated { verdict: Zero(..) } } })` |

The zero-side rows are refusals the rule would describe as a chart
image; the in-band rows are the rule's own refusal reached through
the certificate.

## Fix shape

The child's description is the rule's over the child:
`must_carry_over_edge(s1, s2, child carrier, t0, t1, extent, band)
.description(..)`, `Conventional` mapped to a chart image in one of
the two adjacent charts (which one is the caller's, as at the split's
section boundary), the in-band and refuted verdicts refused typed at
`split_edge`. `split_specs` is total arithmetic on one description
today and has no surfaces to ask with, so the rule goes in the caller
or the restriction takes the surfaces.

## Closed (2026-10-12, TANG)

`Body::split_edge` routes each child of a tangent edge through
`geom_brep::must_carry_over_edge` over the child's own interval and
extent (`Body::tangent_piece`, `crates/topo/src/split.rs`):
jet-determinate keeps the restricted tangency, under-determined stores
an image derived in the chart of the face the parent's `he_plus`
bounds, and the in-band and refuted verdicts refuse typed through
`MustCarryRefusal::route` as `EulerOpError::SplitTangentEscalated`,
`SplitTangentBendEscalated` and `SplitTangentRefuted`. On the fixture
the zero-side rows (`d ≤ 0.99·√(2ε)`) now cut tier-3 valid at the
closed-form volumes with the short seam's image on the ruling, the
in-band rows refuse `SplitTangentBendEscalated`, and
`d ≥ 1.01·√(2Kε)` is unchanged; `wedge_end_doors.rs`'s
`a_cut_across_a_tangent_seam_describes_each_piece_by_the_rule` pins
all three at every ε row.

`shell.rs`'s reader of `split_specs` is `re_anchored` (not `along`,
which mints a fresh chart image). Its seams are interior edges of one
chart group, so both faces wear one surface key and the rule demands
a chart image there; measured, its 44 calls across the sweep and
editor-core shell suites all carry `Chart`.

The sweep's sibling is filed:
`shelf/offset-reanchor-keeps-a-tangent-description-on-a-moved-interval-without-the-must-carry-rule`.
