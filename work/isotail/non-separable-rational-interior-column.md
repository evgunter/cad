---
id: non-separable-rational-interior-column
kind: issue
title: An interior column of a chart whose weight net varies along both parameters has no exact class - the composite bound without a tube
status: open
opened: 2026-09-04
priority: P3
cost: M
---


## The residue

TRIM-1's extractor (`crates/geom-brep/src/nurbs_iso.rs`,
`interior_iso_u`) collapses a described chart's net at an interior
`u*` into a curve in the chart's own `v` space. The seam class's
control-difference hull then needs the collapsed row's weights
`W_j = Σᵢ Nᵢ(u*)·wᵢⱼ` to be STRUCTURE a stored carrier can share
bitwise (C6: weights are `f64` forever). That holds exactly when the
weight net factors in a way an exact `f64` test can see — constant
along `u` (row 0's weights are `W`) or constant along `v` (`W` is
constant in `j` and cancels; the row is polynomial). Both cover every
wall this kernel builds and every imported cylinder wall.

A net varying along BOTH parameters has a computed `W`, so no carrier
shares its rational space and the seam class's hypothesis genuinely
fails. The extractor refuses `IsoRowError::WeightsNotSeparable`; the
certifier turns it into `PcurveCertifyError::IsoUnsupported` naming
the weight net (`pcurve_cache.rs`, the seam class's interior route;
pinned by `geom-brep/tests/interior_iso_column.rs::a2b_*`).

## The honest route, not built

A composite bound: `EnvelopeStatement::MapResidualComposite` through
`geom_core::spline::compose::tensor::surface_curve_residual` (today
consumed only by `ssi/certify.rs`), stated WITHOUT a uniqueness tube —
a `Chart` description names one surface, so there is no operand pair
for a tube to be about. That is a new certificate statement, not a
widening of the iso hull, and it needs its own spec. No construction
in the tree mints such a column today.

## Home

TRIM's: `pcurve_cache.rs` and `nurbs_iso.rs` are in this program's
territory. Not scheduled.

## Measured, and re-banded P1 → P3 (2026-10-24, ISO measurement lane, main `bacd840c77`)

**No real body reaches the refusal today.** Every pin of
`WeightsNotSeparable` is a hand-built net:
- `interior_iso_column::a2b_*`, which takes dm1's wall with one weight poked;
- `interior_iso_review`'s `Net::Both` product net;
- the extractor's own unit test.

Every body-level interior-column fixture gets its column from the test-only
re-chart, on polynomial nets.

Rational-in-both-directions nets do exist:
- **Lofts.** `sweep::loft_body` builds genuinely non-separable walls. A square whose
  arc side's bulge differs per section (bulges `[0.2, 0.4]` at v-degree 1, or
  `[0.2, 0.4, 0.2]` at v-degree 2) has a wall that `interior_iso_u` refuses in both
  directions. Its seams and rims sit on boundary rows and mint fine.
- **Product nets.** A STEP sphere or torus (`w_ij = a_i·b_j`) is refused by the
  extractor too (a 180° × 90° torus patch at `u* = 0.5` refuses `IsoUnsupported`).

What is missing is a door that places an interior column on such a wall:
- a cut at a section plane refuses earlier, at
  `CurvedPairUnsupported { Subtract, RevertRoster, Nurbs, Plane }`;
- step-import's edge ladder states no `Chart` description;
- no sphere or torus has a line on an interior column;
- the 77 STEP files in the tree hold one rational surface family, and it is constant
  along one parameter.

**It rises to P1** when any of these lands:
- NURBS × plane booleans;
- a STEP adoption rung that states `Chart` images;
- face splitting on one chart.

**A cheaper route than the composite certificate.** The seam hull
`|B−C| ≤ max|b_j−c_j|` needs one rational basis, and a uniform scale `w → λw`
leaves that basis unchanged. An exact rank-1 test fails on real 90° nets, though,
because `s·s` is not representable. The sound widening is a weight-perturbation
slack:
- write `W_j = λ·c_j·(1+ε_j)` with `|ε_j| ≤ δ`;
- then `Σ|R^W_j − R^c_j| ≤ 2δ/(1−δ)`;
- so `|B−C| ≤ max_j|Q_j−c_j| + (2δ/(1−δ))·diam{Q}`, with `δ` taken from `W`'s
  enclosures at the interval scalar (about 4e-18 m on the torus probe).

This serves any net, product or not, against a carrier storing `fl(W)`. It is a
widening of the iso hull inside `seam_envelope`, not a new statement.

The extractor's doc claimed every arc-profile loft wall is separable. That is false
for lofts whose arcs differ per section, and is corrected in the same commit as this
note.

Re-priced H → M in the same change: the slack widening above is one term in an existing hull, not a new certificate statement.
