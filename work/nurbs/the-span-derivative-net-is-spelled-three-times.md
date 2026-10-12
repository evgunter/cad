---
id: the-span-derivative-net-is-spelled-three-times
kind: issue
title: The span derivative net is spelled three times in nurbs.rs, and the span control centroid twice
status: closed
opened: 2026-10-12
priority: P4
cost: E
branch: nurbs/one-speed-meter-arm
closed: 2026-10-12
---

## What

`crates/geom/src/curves/nurbs.rs` forms the derivative coefficients
`Qᵢ = p·(Pᵢ₊₁ − Pᵢ)/(uᵢ₊ₚ₊₁ − uᵢ₊₁)` (f64 positivity check on the
structure knots, difference over the held knots) in three places:

- `speed_lower_bound_uncached`, the integral arm's `coeffs`;
- `rational_piece_assembly`, its `da` / `dw` rows over the
  centroid-translated homogeneous net;
- `rational_span_bound`, its numerator and `w′` hull terms.

The span control centroid (`origin + Σ(Pⱼ − origin)/count`) is formed
twice: `rational_piece_assembly` and `rational_span_bound`.

Found by PR 4804's FULL review; that PR kept the fix out of its frozen
delta. One helper per formula, generic over the coefficient type like
`blossom`. The integral arm's reading is pinned bit for bit
(`piece_coverage_meter.rs`), so the shared helper must keep its
operation order.

## Closed (2026-10-12)

`derivative_net` (generic over the coefficient type, like `blossom`)
forms every span derivative net, and `span_centroid` every span
centroid, in `crates/geom/src/curves/nurbs.rs`. Each site keeps its
operation order: at the old 16-piece schedule the whole geom suite,
`piece_coverage_meter.rs`'s bit pins included, passed unchanged on the
refactor alone (its own commit).
