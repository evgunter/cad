---
id: a-micron-thin-cylinder-wall-runs-the-quadrature-out-of-budget-on-its-map-residual-pad
kind: issue
title: A micron-thin cylinder wall runs the cylinder quadrature out of budget at 1e-12 on its map-residual pad
status: open
opened: 2026-10-12
---



## What

`crates/sweep/tests/far_thin_disc_sign.rs`'
`far_tilted_cut_discs_are_read_by_their_exact_volume`, pose `h 1e-6,
tilt 0.8, 5e3 m, turned 0.7`, at `CAD_TOLERANCE_EPS=1e-12`:
`classify_shells` refuses, upright and inside out alike, with
`Props { Face { face 12v3, QuadratureBudget { width_len 1.0728e-9,
target_len 1.024e-9, rounds 13 } } }`. Check 7 passes on both.

Measured (FLUX, `flux/cylinder-band-quad`, instrumenting
`cylinder_cut_face_rounds` per edge): the enclosure stops narrowing by
round 6 at about 7e-12 of `A_s` on a four-edge loop. The edges' pcurve
envelope is `2.8e-13`, and the map-residual pad, `(metric length)·env/r`
with `r = 1e-3`, accounts for that floor. The wall is 1 µm tall, so the
floor's width over the face's tiny area meters just past
`QUAD_TARGET_LEN_FACTOR·ε`. The `∮ u dv` form carried the same pad plus
a rule floor about 100× higher on the same edges. The pose was out of
reach on main: the cylinder ∩ slab that makes it did not build at
1e-12 there (the fixture stood it down), and it builds once the
quadrature reads the cut.

The row pins the refusal by type at that pose and ε
(`runs_out_of_budget`, asserted by `assert_runs_out_of_budget`).

## The shape of a fix

The module docs of `geom_brep::props::quad` argue the target factor
against faces of ordinary proportion. A wall whose height is
`10⁶·ε` is not one. Either the map-residual pad is charged where it
belongs (the region moves by `env`, not the integral by
`length·env/r` on both sides), or the target meters against something
that does not divide by a micron.
