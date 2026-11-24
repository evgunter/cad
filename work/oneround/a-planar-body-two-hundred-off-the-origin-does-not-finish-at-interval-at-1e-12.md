---
id: a-planar-body-two-hundred-off-the-origin-does-not-finish-at-interval-at-1e-12
kind: issue
title: At Interval and eps 1e-12 a planar body 200 m or more off the origin does not finish: incidence enclosures as wide as the zero band refuse it inside the session box
status: open
opened: 2026-11-23
priority: P2
cost: M
design: true
refs: [a-station-far-from-the-origin-reads-indeterminate-on-its-own-carrier-and-panics]
---


## Finding

Filed by `charttail/held-station`. `sweep`'s plate whose void touches
its top (`void_touching_plate_at`,
`crates/sweep/tests/self_intersection_proves_injectivity.rs`), built at
`Interval` under `CAD_TOLERANCE_EPS=1e-12`, finishes up to 150 m off the
origin along `(1, 1, 1)` and does not finish further out, each a typed
refusal the fixture unwraps:

| offset | refusal | predicate | enclosure |
|---|---|---|---|
| `(200, 200, 200)` | the void's pyramid is `NotFinished`, `PlanarBoundaryEscalated` on five face/edge pairs (`topo::test_support::meeting::posed_pyramid_at`) | `pcurve_line_chart_incident` (`geom-brep/src/pcurve_cache.rs`) | `[0, 1.04e-12]` to `[0, 1.13e-12]` |
| `(300, 300, 300)` | `DescriptionUncertified`, `SurfaceResidual` (`topo::test_support_fixtures::describe_as_intersections`) | `carrier_on_surface_2` (`geom-brep/src/certify.rs`) | `[-1.03e-12, 1.04e-12]` |
| `(900, -900, 900)` | `Escalated` at vertex 0 (`topo::test_support_fixtures::plane`, `newell_plane`) | `newell_plane_residual` (`geom-brep/src/newell.rs`) | `[-1.63e-12, 1.45e-12]` |

Every enclosure holds zero and is about as wide as the zero band (1e-12):
the incidences hold, and the scalar's dependency width, which grows with
the coordinates, is what the band cannot hold. At `f64` every placement
builds and asks, and at ε = 1e-9 and 1e-6 both scalars do.

The same width is what made the region door's held check panic at 100 m
(`work/charttail/a-station-far-from-the-origin-reads-indeterminate-on-its-own-carrier-and-panics.md`,
cured in `topo::held::Held::check`: an enclosure that does not read a
definite sign is consistent with a construction). These sites are
decisions, not debug checks, and refuse typed, which `[tolerance.recourse]`
permits.

## The question

`[tolerance.units]` (D4 ¶4) says the session box (1e3 m) and K fix the
smallest admissible ε, an argument about `f64` rounding. At `Interval`
the widths of these incidence reads reach the zero band at 1e-12 about
200 m off the origin, a fifth of the box, so a planar body inside the box
at an admitted ε does not finish at that scalar. Whether that is the
scalar's honest refusal (the admission argument is per scalar, and says
so), or the enclosures are owed tighter forms at these three predicates
(evaluated about a local origin, as a station's residual could be), is
open; measure the widths' growth at each predicate before choosing.

The row
`a_body_that_does_not_pass_is_told_one_finding_or_another_and_never_that_it_holds`
in that test file skips the 900-off placement at `Interval` below
ε = 1e-9 on this row's account.
