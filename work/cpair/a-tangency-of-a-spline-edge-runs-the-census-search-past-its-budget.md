---
id: a-tangency-of-a-spline-edge-runs-the-census-search-past-its-budget
kind: issue
title: A spline bounds no acceleration for the census's second-order clear and tangency floor, so an edge-edge record at a tangency of a spline edge refuses on the budget
status: closed
closed: 2026-10-26
opened: 2026-10-13
priority: P3
cost: M
---

Found by FUSE's review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

`census::curved`'s meeting search clears near a tangency by its
second-order bound and stops its pieces at the floor
`√(escalate/κ)/8`, both of which need a bound on `|C″|`
(`carrier_speed::accel_hi`). A spline has none at every scalar
(`geom::nonrational_second_derivative_sup` is `f64`-only and refuses a
rational net), so its pieces stop at the band itself and an edge-edge
record at a tangency of a spline edge runs past the budget and refuses
typed. A generic second-derivative bound on the control net would give
splines the floor too.

## What changed (2026-10-26)

SPLICE's PR 5715 (branch `splice/touch-doors`) gave splines a bound.
`carrier_speed::accel_hi` reads `NurbsCurve3::accel_upper_bound`, and
`curvature_hi` reads it over the speed floor; both return a value, not
an `Option`. So an uncreased spline now gets `curve_pair::walk`'s
second-order clear and its tangency floor, and this row's premise no
longer holds for it.

A spline with a crease (an interior knot of multiplicity at least the
degree) still reads no finite bound and still walks to the band. That
is filed as `work/splice/a-creased-spline-gets-no-second-order-clearing.md`.
Whether the census's edge-edge record at an uncreased spline's
tangency now stays within the budget is unmeasured. Status left as is.

## Closed (2026-10-26, tracker sweep)

**Superseded.** The row's premise, that a spline bounds no
acceleration, is gone on main: `geom::carrier_speed::accel_hi`
(`crates/geom/src/carrier_speed.rs:165`) reads
`NurbsCurve3::accel_upper_bound` (`crates/geom/src/curves/nurbs.rs:1240`),
SPLICE's PR 5715, so an uncreased spline gets the walk's second-order
clear and its tangency floor (the 2026-10-26 note above). The creased
case, where the bound is still infinite, is owned by
`work/splice/a-creased-spline-gets-no-second-order-clearing.md`. No
census row is known to run past its budget on an uncreased spline; a
lane that meets one files it with the measurement.
