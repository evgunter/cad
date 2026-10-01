---
id: the-chart-speed-and-tube-pads-read-a-derivative-norm-off-a-per-coordinate-box
kind: issue
title: the plane x NURBS chart speed, transverse stretch and tube pad read a derivative's norm off a per-coordinate box, so a rigid map moves them
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## Found (§5 sweep of the frame-invariant limb-2 lane, branch `ssi/frame-invariant-bound`, 2026-10-01)

**Inferred from the code, not reproduced.** The limb-2 fix reads the
residual composite's bound from each vector coefficient's norm, so a
rigid map moves it only by rounding (`geom_core::spline::compose::tensor`,
`coefficient_norm_bound`). Three more upper bounds on a vector's norm in
the plane × NURBS lane are still read off a per-coordinate box, all of
them from `NurbsBoxes::deriv_box` (`crates/geom-brep/src/ssi/enclose.rs`),
the quotient-rule hull of `∂S/∂u` or `∂S/∂v` over a rectangle:

- **The chart speed**, `plane_nurbs_ssi` (`crates/geom-brep/src/ssi.rs`):
  `norm_sup` of the whole-domain `deriv_box` in u and v, which turns the
  floors stated in metres into parameter units.
- **The transverse stretch**, `graph_margin`
  (`crates/geom-brep/src/ssi/enclose.rs`): `norm_sup` of
  `S_u·e⊥x + S_v·e⊥y` assembled on the two derivative boxes, the divisor
  of the tube's transversality margin.
- **The tube pad's chart speed**, `certify_branch`
  (`crates/geom-brep/src/ssi/certify.rs`): `norm_sup` of the whole-domain
  `deriv_box`, which converts the tube radius into chart pads.

Each is sound, and each reads between 1× and √3× the norm it bounds
depending on how the derivative field sits against the axes. A rotation
can therefore move the sweep's floors, the transversality margin and the
pads. On the limb-2 reproduction
(`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`) none of them moves
a printed digit across 32 rotations, so no verdict is known to turn on
them.

## Why the limb-2 lane did not fix them

- They are limb 3 and the tube. PR 3683 (`ssi/chart-tube`) is changing
  that limb and the certificate's tube field in flight.
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
