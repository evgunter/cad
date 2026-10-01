---
id: cell-curvature-joins-its-assemblies-in-f64-to-nearest
kind: issue
title: cell_curvature joins certified ends with f64 sums rounded to nearest (h.hi() + root, w11.hi() + w12.mag()), so its curvature bound can land an ulp inside
status: open
opened: 2026-10-01
---


## The finding

Found during the review of `linalg/certification-gains-a-sqrt-door`
(PR #3727). In `crates/geom-brep/src/offset_meters.rs`,
`cell_curvature`, the certified pieces are intervals: `h`, `k`, the
door root of `H² − K`, and the Gershgorin entries `w11`…`w22`. They are
then joined in `f64`, rounded to nearest:

- assembly A: `a_hi = h.hi() + root` and `a_lo = h.lo() − root`;
- assembly B: `w11.hi() + w12.mag()`, `w22.hi() + w21.mag()`,
  `w11.lo() − w12.mag()` and `w22.lo() − w21.mag()`.

Each sum can round half an ulp toward the inside of the curvature
range, which is the unsafe side for a principal-curvature bound. The
range then feeds the offset radius headroom. The fix is to keep each
sum in interval arithmetic, `(h + root_iv).hi()` and so on, or to step
each `f64` sum outward. The error is ulp-relative, so it matters only
where a headroom decision is at its threshold.
