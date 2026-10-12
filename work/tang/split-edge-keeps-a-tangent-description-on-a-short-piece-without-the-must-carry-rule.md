---
id: split-edge-keeps-a-tangent-description-on-a-short-piece-without-the-must-carry-rule
kind: issue
title: an edge split keeps a TangentIntersection on a piece the must-carry rule calls under-determined, so a cut near a tangent seam's end refuses at the certificate
status: open
opened: 2026-10-09
priority: P2
cost: M
refs: [split-tangent-chord-mints-tangency-without-the-must-carry-rule]
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
