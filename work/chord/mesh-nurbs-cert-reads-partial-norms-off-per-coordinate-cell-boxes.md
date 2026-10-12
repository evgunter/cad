---
id: mesh-nurbs-cert-reads-partial-norms-off-per-coordinate-cell-boxes
kind: issue
title: nurbs_cert's cell_readings folds PatchCell's per-coordinate boxes into the partials' norms, so a rigid map moves the face bounds
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## Found (ENCL §5 sweep, `encl/offset-cert-coefficient-norms`, 2026-10-08)

`crates/mesh/src/nurbs_cert.rs`, `cell_readings`, reads each partial's
norm as `norm_sq` of `geom_brep::patch_bound::PatchCell`'s signed
per-coordinate boxes (`s_uu`, `s_uv`, `s_vv`, `s_u`, `s_v`), whose `√hi`
is the sup bound `NurbsFaceBound` carries. A box of one vector field
reads between 1× and √3× its norm depending on how the field sits
against the axes, so the same face rotated rigidly gets different
second-derivative bounds and hence different tessellation sizing — the
shape DESIGN.md D4 ¶2 now rules out for certified vector upper bounds
("read from the Euclidean norm of each coefficient, never from a
per-coordinate box folded into a norm").

`PatchCell` now carries `s_u_sup`/`s_v_sup`, read from the derived
control vectors' norms on the integral arm (`window_norm_sup` in
`crates/geom-brep/src/patch_bound.rs`, through
`geom_core::spline::compose::tensor::coefficient_norm_sup`). The same
reading applies to the second-derivative nets on the integral arm
(`integral_cells_on` holds them as `DNets`); `nurbs_cert`'s
`NurbsFaceBound` documents itself as non-rational, so the integral arm
is the one it reads.

**Inferred from the code, not measured** on a tessellation; the
magnitude on PatchCell's own speeds is `1.9e-3` of scale across 16
rotations of the bowed fixture (ENCL's pin
`the_vector_upper_bounds_do_not_move_under_a_rotation`).

## What is open

Whether the face bounds should read coefficient norms (PatchCell would
carry `s_uu_sup` and friends, filled by patch_bound's owner), and what
the k-lint baselines do when they tighten.
