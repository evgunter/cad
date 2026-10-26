---
id: a-micron-thin-cylinder-wall-runs-the-quadrature-out-of-budget-on-its-map-residual-pad
kind: issue
title: A micron-thin cylinder wall runs the cylinder quadrature out of budget at 1e-12 on its map-residual pad
status: closed
closed: 2026-10-26
opened: 2026-10-12
priority: P2
cost: M
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

## Update (PR 5628, measure door unit B)

`classify_shells` no longer measures a shell to the reporting target:
it reads the role, and the enclosure that read reached. So the pose now
classifies `Outer` both ways round, and the row's pin
(`runs_out_of_budget`) is deleted. The floor itself is unchanged, and so
is the number door's refusal: `mass_properties` on this disc at 1e-12
still refuses `QuadratureBudget` until unit C hands back the bracket.

## Closed

Closed by PR 5734 (`flux/cylinder-sliver-exact-bracket`), which undercuts the
diagnosis above. The floor was not the map-residual pad. The cylinder lane
read each trim edge's end `(cos t, sin t)` off the end's vertex, padded by
`ε/r`, which over `r = 1e-3` is `1e-9` of the trig at 1e-12, and that width
rode every piece of the edge. The lane now forms each end's trig at the
parameter it integrates from (`geom_brep::props::quad::trig_at_limit`),
3.4e-13 wide at most, and does not read the vertex.

At 1e-12 the pose classifies, upright and inside out, and measures
`4.5092040e-12 ± 2.83e-18` m³ against the exact `π r² h / cos 0.8 =
4.5092040e-12`, `6.8e-20` off. Every other pose of the row holds its exact
volume within its pad at 1e-12 and 1e-9. So the number door no longer
refuses it, and the row asserts each upright disc's volume against
`π r² h / cos tilt` (`far_tilted_cut_discs_are_read_by_their_exact_volume`).

