---
id: the-chart-speed-and-tube-pads-read-a-derivative-norm-off-a-per-coordinate-box
kind: issue
title: the plane x NURBS chart speed, transverse stretch and tube pad read a derivative's norm off a per-coordinate box, so a rigid map moves them
status: closed
opened: 2026-10-01
priority: P3
cost: M
closed: 2026-10-02
pr: 3916
branch: ssi/rotation-chart-speed
---


## Found (§5 sweep of the frame-invariant limb-2 lane, branch `ssi/frame-invariant-bound`, 2026-10-01)

**Inferred from the code, not reproduced.** The limb-2 fix reads the
residual composite's bound from each vector coefficient's norm,
`max_k |n_k| / |d_k|` over a one-signed scalar denominator, so a rigid
map moves it only by rounding (`geom_core::spline::compose::tensor`,
`coefficient_norm_bound`). Two more upper bounds on a vector's norm in
the plane × NURBS lane are still read off a per-coordinate box, both
from `NurbsBoxes::deriv_box` (`crates/geom-brep/src/ssi/enclose.rs`),
the quotient-rule hull of `∂S/∂u` or `∂S/∂v` over a rectangle:

- **The chart speeds**, `NurbsBoxes::chart_speeds` (same file):
  `norm_sup` of the whole-domain `deriv_box` in u and in v, minted once
  per wall as `ChartedNurbs` (`crates/geom-brep/src/ssi.rs`). They turn
  the floors stated in metres into parameter units, and the tube rung
  into chart pads.
- **The transverse stretch**, `graph_margin` (same file): `norm_sup` of
  `S_u·e⊥x + S_v·e⊥y` assembled on the two derivative boxes, the divisor
  of the tube's transversality margin.

Each is sound, and each reads between 1× and √3× the norm it bounds
depending on how the derivative field sits against the axes. A rotation
can therefore move the sweep's floors, the transversality margin and the
pads. On the limb-2 reproduction
(`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`) the tube rung,
the transversality margin (to 1e-6 relative) and the box count do not
move across 32 rotations; the pads are not compared. No verdict is
known to turn on them.

## Why the limb-2 lane did not fix them

- They are limb 3 and the tube, which PR 3683 (`ssi/chart-tube`) had
  in flight; it consolidated the speeds into `chart_speeds` and kept
  their box reading.
- `deriv_box` is a hull of the quotient rule `(A_d − S·w_d)/w`, with `S`
  a box, not a Bernstein form whose coefficients a norm can be read
  from. A norm reading needs the rational derivative's own coefficient
  form, `(A_d·W − A·W_d)/W²`, centre-shifted as the tensor composite is,
  so it is a new enclosure rather than a re-spelling.

## What is open

Give the three sites a derivative-norm bound read from vector
coefficients. Lower bounds stay box-assembled: a minimum of a convex
function is not at a coefficient, so a mignitude floor cannot be read
from coefficient norms.

**The translation half is closed** (row
`rational-chart-sup-speed-grows-with-translation`): `deriv_box` now
reads every control point as a difference with another, so a
translation moves these readings by their rounding width. A rotation
still moves them by the box-to-norm factor this row names. A cheap
candidate measured on that row's 1.8 / 0.7 wall: per cell,
`(max_i ‖ΔA_i − c·Δw_i‖ + max_j ‖P_j − c‖·max_i |Δw_i|) / w_min` with
`c` the cell's control centroid, read 5.91 against the box norm's
5.86 (true 2.04) and is rotation-invariant; the coefficient form above
would be tighter.

## Closed (2026-10-02, PR 3916)

The chart speeds, the pads and the transverse stretch read
`max ‖t(P_j)‖ / w_min` over the quotient numerator's pair terms `t` and
the cell's control points `P_j` (`CellNet::derivative_norm_sup`,
`CellNet::transverse_readings`), with each cell cut to the window and
met with its whole net. Each term is affine in `S`, so its norm peaks
at a control point. The old box holds every `t(P_j)`, so the reading
is never above the box's norm. On the 1.8 / 0.7 wall the `u` speed reads
5.853 in every frame (the box read 5.858 seated and 6.19 to 8.14 over
32 rotations; true 2.036). Across the SSI suite's mints, 58 of 60
speeds and 2799 of 2808 stretches read the same to six digits and the
rest are tighter, none looser. Pinned by
`a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube`,
`the_rotation_row_reads_red_on_a_per_coordinate_fold` and
`the_derivative_box_dominates_the_dense_sampled_true_derivative`
(`enclose.rs` tests). No golden moved. The sweep's further props sites
went onto `work/props/props-reads-vector-norm-bounds-off-per-coordinate-hulls.md`.
