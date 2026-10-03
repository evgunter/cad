---
id: project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry
kind: issue
title: PROJECT_EPS_POINT = 1e-13 m is absolute and below ulp(2048), so NurbsSurface::project refuses inconclusive on a km-scale model for a reason that is not geometric
status: open
opened: 2026-10-01
priority: P2
cost: M
---


(SSI measurement lane, from `work/ssi/plane-nurbs-certificate-bound-does-not-refine-with-eps.md`,
2026-10-01. Measured.)

`crates/geom/src/projection_policy.rs`'s `PROJECT_EPS_POINT` = 1e-13 m
is an absolute tolerance, and it lies below ulp(2048) ≈ 4.5e-13. On the
m8_4 seam fixture scaled to 1024 m, `NurbsSurface::project` returns
`SurfaceProjectionInconclusive` at 9 of 32 off-schedule points (u pinned
at 1.0, last `orthogonality_v` 2.3e-10, `last_distance` 1.1e-13). With
`PXN_FIT_SAMPLES` ≥ 65 the plane×NURBS lane itself refuses
`FootPointInconclusive` (sample 27, 2.27e-13) on a seam that is
geometrically exact. A kilometre-scale model therefore refuses for
f64 resolution, not geometry. The stopping test wants a scale-relative
term (ulps of the point's coordinates), or the refusal should name the
resolution.

## The SSI plane × NURBS lane reaches it too (PR 3707, 2026-10-01)

Measured by the PR 3707 review and its fix pass. `ssi::plane_nurbs_ssi` on
`tests/m5_pr7_ssi.rs`'s cutting plane × certifiable wall, both
translated along x, refuses `FootPointInconclusive` in limb 1's
certified foot points (`ssi/certify.rs` `nurbs_limbs`, through
`NurbsSurface::project_from_seed`):

- a km-scale pair at the default ε refuses with `last_distance`
  1.27e-13, on the merge base and on the PR head alike;
- at ε = 1e-9 every translation from 1e4 m to 3e5 m refuses this way
  on the PR head. On the merge base the march lost its branch first
  (`StepRefinementFailed`) from 3e4 m, which hid it.

The march no longer answers first at these scales, so the absolute
`PROJECT_EPS_POINT` is now the first refusal a far plane × wall pair
meets.
