---
id: point-in-solid-curved-arms-read-the-band-before-the-face
kind: issue
title: point_in_solid's curved arms read an in-band carrier residual, and a ray's in-band margins, before asking whether the face is near, as the plane arm no longer does
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Unmeasured: no fixture is known to reach it. Found by the sweep of the
far-plane fix (`point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face`).

That fix changed `point_in_solid`'s plane arm in two places
(`crates/topo/src/boolean/solid_contain.rs`):

- **Boundary pre-pass (`point_in_faces`).** An in-band
  `bool_point_in_solid_plane` elevation now refuses only where `q`'s
  foot on the plane is not definitely outside the face.
- **Ray sweep (`cast_ray`).** In-band readings on the plane arm, and
  at every crossing's advance and order, are `RayFault::InBand`. Each
  abandons its ray, and the query refuses on the first of them only if
  no ray decides.

The curved arms keep the old order on both counts.

1. **Pre-pass.** The cylinder, cone, sphere-patch and torus arms all
   decide `bool_point_in_solid_plane` on the carrier residual. An
   in-band residual escalates at once, wherever the face's trim lies.
   For example, a point a band off a cylinder's carrier but a metre
   past the wall's azimuth window refuses. The plane arm would set
   that face aside.
   - The fix is the plane arm's shape: ask the trim of the point's
     foot on the carrier.
   - The trims (`wall_hit`, `point_on_sphere_in_face`,
     `point_on_torus_in_face`, `point_on_cone_in_face`) are written for
     on-chart points. So the foot has to be projected per kind:
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
