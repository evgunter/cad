---
id: a-path-reading-offers-no-detour-on-a-sphere-a-torus-a-spline-wall-or-to-an-island-chord
kind: issue
title: A path reading offers no detour round run vertices on a sphere, a torus or a spline wall, nor from an outer-loop point to an island's closing chord
status: open
opened: 2026-11-20
priority: P2
cost: E
refs: [a-union-of-two-axis-placed-revolves-refuses-ring-homing-ambiguous-since-rehome-by-path]
---


Filed by `sect/coaxial-rehome` from its sweep.

## What

A path ([`ring_path::Carrier::paths`]) that runs through a loop vertex
reads in the zero band and says nothing. Ring re-homing on a cylinder
or a cone now asks detours after the plain paths
(`ring_path::Carrier::detours`, `Quadric::turns`). Each detour turns at
the azimuth or the height of a run edge's midpoint, so it goes round
vertices that share the ring vertex's and the outer loop's azimuths and
heights (`chord_join::path_ring_side`). Two readings of the same shape
have no detour:

- **Ring re-homing on a sphere, a torus or a spline wall.**
  `Quadric::turns` returns none there. A sphere offers its one
  great-circle arc, a torus its two meridian/parallel paths, and a
  spline wall its iso paths. A ring vertex whose every path to every
  outer-loop point passes a run vertex reads `Undecided` and refuses
  `RingHomingAmbiguous`. A sphere revolved about the axis that its
  neighbours share puts its seam and section vertices on common
  meridians and parallels, as the coaxial cylinders did.
- **An island's winding** (`chord_join::IslandReading::read_paths`).
  It asks each outer-loop point's plain paths to the closing chord's
  midpoint `q` and no detour. An outer-loop point whose every path to
  `q` passes a run vertex says nothing. A face whose every point says
  nothing refuses `SectionInvariant` ("no outer-loop point ... reads
  which side of a ring-lane island it is on").

No body has been seen to reach either case.

## Candidate

On a sphere, a turn is a point of the great circle through the guide
and `to`. On a torus, a turn is `to`'s meridian at the guide's
azimuth, or `to`'s parallel at the guide's minor angle. For the island
winding, chain `Carrier::detours` after each reference's plain paths in
`read_paths`, steered by the run edges' midpoints. The arrival is the
onward path's, so the lean reading is unchanged. A test needs an island
whose every outer-loop point is aligned with a run vertex on the way to
`q`.

## Re-banded P2 (PR 7181's review, 2026-11-20)

`sect/rimside-retires` puts every one-site loop that is not on a plane
onto the island reader's paths. Those paths have no detour, and on a
sphere there is one great-circle path per reference. On a coaxial
sphere, a closed loop can have every reference aligned through the
site vertex. That is the same shape as the P0 this row was filed
from. The review judged it likely reachable, but no body has been
seen to hit it.
