---
id: closest-crossing-and-graze-abandon-have-three-homes
kind: issue
title: The closest-crossing ray reading and its graze-abandon protocol live in three places: cast_ray, splitting::containment and sphere_region
status: open
opened: 2026-10-05
priority: P1
cost: M
design: true
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

