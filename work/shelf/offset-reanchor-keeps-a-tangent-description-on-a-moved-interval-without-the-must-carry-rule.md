---
id: offset-reanchor-keeps-a-tangent-description-on-a-moved-interval-without-the-must-carry-rule
kind: issue
title: an offset's re-anchor keeps a TangentIntersection on a moved interval the must-carry rule calls under-determined, so pulling a cap close to a tangent seam's end refuses at the certificate
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [split-edge-keeps-a-tangent-description-on-a-short-piece-without-the-must-carry-rule]
---


## Finding

Found by the §5 sweep of the PR that routed `Body::split_edge`'s
children through `geom_brep::must_carry_over_edge`
(`split-edge-keeps-a-tangent-description-on-a-short-piece-without-the-must-carry-rule`).

`plan_reanchors` (`crates/topo/src/replace_face.rs`) moves the
parameter at each moved end of an edge that ends at a moved vertex,
then keeps a `TangentIntersection`'s pair and re-mints its witness at
the new mid-parameter. The rule is never asked over the moved
interval. Its second-order margin is the sagitta `|κ_rel|·arm²/2`
with `arm = min(curvature arm, extent)`, so a shortened tangent edge
can be under-determined, or in band, where the edge was
jet-determinate. The certificate then refuses it at the re-attach.

## Measured

The `y < 1` piece of `crates/sweep/tests/wedge_end_doors.rs`'s
shoulder (`shoulder(1.0)` cut by `shoulder_cut()`), whose seam is
`TangentIntersection` along `x = 0, y = 1, z ∈ [0, 1]`
(`κ_rel = 1`). `topo::replace_face_offset` moves its top cap
(`z = 1`) by `−(1 − d)`, so the seam keeps length `d`. At
`Tol::witness()` (ε 1e-9, K 10), `d = c·√(2ε)`:

| c | outcome |
|---|---|
| 0.25, 0.5, 0.99 | `Op { Certification { NotSecondOrderSeparated { verdict: Zero(..) } } }`, sagitta 6.25e-11 … 9.8e-10 |
| 1.01, 1.5, 0.99·√K | `Op { Certification { Escalated { check: TangentSecondOrder, .. } } }` |
| 1.01·√K, 4, 8 | builds, tier-3 valid, volume `(2 + π/4)·d` |

The zero-side rows are refusals the rule would describe as a chart
image; the in-band rows are the rule's own refusal reached through the
certificate.

## Fix shape

Through `geom_brep::must_carry_description`, as `split_edge`'s
tangent children are (`Body::tangent_piece`, `crates/topo/src/split.rs`):
the moved edge's description is the rule's over the moved interval,
`must_carry_description(SmoothJoin { surfaces, carrier, interval,
chord, witness }, band, chart, …)`, with `Conventional` resting in the
adjacent chart the door names through `chart`, and the in-band (first-
and second-order), refuted and foreign-chart outcomes refused typed at
the offset door through the constructors it passes. The pair's surfaces are the
ones the re-attach certifies against (unmoved here, since the edge
does not lie on the replaced face's boundary).
