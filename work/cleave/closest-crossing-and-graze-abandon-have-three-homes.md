---
id: closest-crossing-and-graze-abandon-have-three-homes
kind: issue
title: The ray-walk driver (schedule, graze, set-aside, exhaustion) and the closest-crossing fold have six homes
status: dispatched
opened: 2026-10-05
priority: P1
cost: H
branch: cleave/ray-walk
---

Found by the dual review of PR 4046 (both lanes, style Q1). PR 4046 added
the third instance.

## The three readers

Each casts rays from a query point over a fixed schedule, keeps the closest
crossing, reads the material side from the heading there, and abandons a
ray on a graze (a vertex hit, a tie, a tangency, a reading in band) before
the next ray is tried:

- `boolean::solid_contain::cast_ray`, in 3-D over a body's faces. Its
  graze vocabulary is `RayFault::{Abandon, Fatal}` with
  `ray_parity::Abandoned` collecting the first in-band reading.
- `splitting::containment` (`point_in_loop`, `walk_schedule`), in a
  face's plane over a loop's edges, with `ray_parity::Abandoned` and
  `PointInLoopError::RayExhausted`.
- `boolean::sphere_region::SphereFaceRegion::contains`, along great
  circles over a sphere face's arcs. It has its own `Ray::Abandoned`, its
  own `TARGET_SHARES` ladder of aimed rays, both-ways casting, and a
  first-diagnostic fold.

The geometry differs (line × surface, line × curve in a plane,
great circle × circle), and the protocol does not, yet it is written three
times. The protocol is: the schedule and its order, what counts as a
graze, the tie rule against the closest crossing, which in-band readings
abandon the ray and which refuse the query, and what an exhausted schedule
reports. Each copy has already diverged. Only `sphere_region` casts both
ways (an antipodal vertex meets every great circle), and the planar walk
and `cast_ray` name their exhaustion differently.

## What is open

The design question is one home for the protocol (a ray-walk driver
generic over a per-geometry "crossings along this ray" reader), and what it
does to the three readers' refusal types. That is the `design: true` on
this row.

## Built (branch `cleave/ray-walk`)

The framing above was partly wrong: `splitting::containment` counts
parity and reads no heading, and the repeated thing is the WALK DRIVER,
copied into `walk_schedule` (both planar loop walks),
`chart_region::point_in_polygon`, `chart_bound::parity`,
`solid_contain::point_in_faces` and `SphereFaceRegion::contains`, with
`splitting::order::in_plane_frame` a schedule ladder of the same shape.
The per-ray readings are two kinds and stay two.

- `topo::ray_walk` (renamed from `ray_parity`) holds the driver
  (`walk`), one ray outcome vocabulary (`RayFault`: `Graze`, `InBand`,
  `Blocked`, `Fatal`), the one exhaustion sentence (`RaysGrazed`, one
  recourse: move the geometry), the parity reading, and the
  closest-crossing fold (`Crossings`, `advance`) the solid sweep and the
  sphere region share. Exhaustion precedence is one rule, in `walk`'s
  docs: the first limit that blocked a ray, else the first in-band
  reading, else every ray grazed.
- Every reader above runs on it; `profile::validate::point_in_loop`
  is left as it is.
- The fold ties against the closest crossing, not the running best, and
  a boundary hit or a tangential incidence grazes only when it is the
  closest (PR 4046's sphere rule, now `cast_ray`'s too).
- The sphere region reads its arcs before its rays
  (`ConicArc::hit`, rows `bool_sphere_region_arc_*`) and has its own
  refusal (`RegionRefusal`), which the solid door and the pierce arm
  wrap.
- Inside `cast_ray` every in-band reading, every trim reading at a hit
  and the confined limits are about the ray; a ray that meets nothing
  where the body's volume cannot side the point is set aside.
- `PointInSolidError::inconclusive` holds the confined limits; the
  shell witness ladder tries the next witness past one and ranks the
  first as the refusal, as `join::loop_roles` does.
- `in_plane_frame` keeps its first in-band arm.

Measured. The sphere region's missing pre-pass reproduced: on the unit
sphere less a 60° cap, round 200 azimuths and both orientations, a
point 0.5ε off the arc answered `In`/`Out` at 88 of 800 queries and 0.9ε
off at 176 of 800 (ε 1e-9, K 10); 2ε off answered 796 of 800. All now
read on the boundary, refuse on `bool_sphere_region_arc_on`, or read
their side past the band (`sphere_region` tests). Rows re-baselined,
refusal → answer only: the torus suite's shell above the tube's top
circle is now the residual one (9.69e-9 at the default row, was
3.66e-4, a cube-root shell of refusing root counts); the tilted-cut
walls answer far more probes (the lens 62 → 514 of 720, each against its
truth, wrong 0 at every ε row), where a ray meeting nothing refused the
whole query on the props lane's uncertified volume.
The same set-aside opened more where a probe ray meets the boundary,
each held to its closed form: `conic_edge_curved_face`'s ball through
the cut face and two of its rim rods, the boss under a slab in every
member order (`reach_slab_cut_sector_side`), and the tour's two
tilted-cut walls, now held checks. The rows that pinned their refusals
were rewritten; CONTACT's two frontier rows and VACUITY's torus-shell
row are annotated.
