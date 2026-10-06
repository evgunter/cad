---
id: point-in-solid-curved-arms-read-the-band-before-the-face
kind: issue
title: point_in_solid's curved arms read an in-band carrier residual, and a ray's in-band margins, before asking whether the face is near, as the plane arm no longer does
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Found by the sweep of the far-plane fix
(`point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face`).

## Measured (2026-10-05, PR 4046's dual review)

Every body carrying a trimmed sphere face reaches it since PR 4046 reads
such faces (`boolean::sphere_region`). The witness is the unit ball at the
origin ∩ the box `[−0.3, 0.2] × [0.6, 2] × [−2, 2]` (a slab across the
`+y` pole). Take the point on the sphere along `(0.126, 0.122, −0.985)`,
about 0.48 outside the body. Placed 5ε off the sphere, it refuses
`Escalated(bool_point_in_solid_sphere)` at ε 1e-9; placed on the sphere,
it answers `Out`. Reproduced on PR 4046's branch, which renamed the
sphere arms' residual from `bool_point_in_solid_plane` to
`bool_point_in_solid_sphere`.

The reviewer's ball × box and lens families gave the counts, per family
per ε: about 900 such refusals at points clear of every face, plus 65
`bool_ray_sphere_disc` in the lens family
(`analysis/reach-dual/4046-r1`, `review.md` MINOR 1).

That fix changed `point_in_solid`'s plane arm in two places
(`crates/topo/src/boolean/solid_contain.rs`):

- **Boundary pre-pass (`point_in_faces`).** An in-band
  `bool_point_in_solid_plane` elevation now refuses only where `q`'s
  foot on the plane is not definitely outside the face.
- **Ray sweep (`cast_ray`).** In-band readings on the plane arm, and
  at every crossing's advance and order, are `RayFault::Abandon`. Each
  abandons its ray (`ray_parity::Abandoned`), and the query refuses on
  the first of them only if no ray decides.
- **Parallel skips.** A ray parallel to a plane's carrier within the
  band, or to a wall's axis, skips the face only where `q` is
  definitely off the carrier (`clear_of_carrier`). Otherwise the ray is
  abandoned. That part covers the wall too.

The curved arms keep the old order on both counts.

1. **Pre-pass.** The cylinder, cone, sphere-patch and torus arms all
   decide the carrier residual (`bool_point_in_solid_plane`, or
   `bool_point_in_solid_sphere` on the sphere arms). An
   in-band residual escalates at once, wherever the face's trim lies.
   For example, a point a band off a cylinder's carrier but a metre
   past the wall's azimuth window refuses. The plane arm would set
   that face aside.
   - The fix is the plane arm's shape: ask the trim of the point's
     foot on the carrier.
   - The trims (`wall_hit`, `SphereFaceRegion::contains`,
     `point_on_torus_in_face`, `point_on_cone_in_face`) are written for
     on-carrier points. So the foot has to be projected per kind:
     radially for the wall, sphere and torus, and along the
     generator's normal for the cone. `point_on_cone_in_face` reads the
     slant as `h / cos α` off the point itself, which is not the foot's.
2. **Ray sweep.** These arms still escalate in-band margins with
   `.map_err(escalate)?` (`RayFault::Fatal`). Some are about the ray:
   the wall roots' discriminant and axis-parallel rung,
   `bool_ray_sphere_disc`, `bool_ray_cone_{lead,disc,apex,nappe,incidence}`,
   the torus root count and `bool_ray_torus_incidence`, and a trim
   reading at a hit point. Others are about the body: `bool_wall_trim_period`,
   the wall outline's class rows, `require_ring_torus`, the cone's
   period and nappe premises.
   - Each arm needs its predicates split that way before the ray-level
     ones can abandon the ray.
   - `PartialCone`'s set-aside is already the abandon shape.

## Part 2 landed on branch `cleave/ray-walk`

The ray-walk driver unit made every in-band reading inside `cast_ray`
a reading of that ray (`ray_walk::RayFault::InBand`): the wall roots'
rungs, `bool_ray_sphere_disc`, `bool_ray_cone_{lead,disc,incidence}`,
the torus root count (including `bool_ray_torus_count`'s
contradiction) and `bool_ray_torus_incidence`, and every trim reading
at a hit point (`at_hit`, which also sets aside a ray on
`WallOutlineUnsupported`, `EdgeCarrierUnsupported` or
`PartialConeFace`). The body-level rows the list above names
(`bool_wall_trim_period`, the outline class rows, `require_ring_torus`,
the cone premises) are reached inside those trim readings and set the
ray aside with them; setting a ray aside cannot make an answer wrong,
and where every ray meets that face the refusal is the same
`Escalated` it was. Part 1, the pre-pass reading the band before the
face, is still open.


## Evidence (PR 4083's verifier, 2026-10-06)

After PR 4083, the only refusals left near a trimmed curved face's carrier are of this row's shape.
The cylinder, sphere-patch and torus pre-passes refuse anywhere in their carrier's band before reading
the trim. A sphere example: 1,700 points radially ±3ε off the sphere, off every arc's continuation,
refuse `bool_point_in_solid_sphere`, the same as on base. Partial-arc points on the sphere now answer.

A related nested ε-dependence, read in the code and not demonstrated (it needs a spline-edged face):
`cast_ray`'s `at_hit` maps `EdgeCarrierUnsupported` to `Blocked`. That error comes from the in-face
loop walk, which ranks `Blocked(Uncrossable)` above an in-band ray of its own, so a tighter ε could
free the solid's ray.
