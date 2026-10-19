---
id: a-tangency-of-a-spline-edge-runs-the-census-search-past-its-budget
kind: issue
title: A spline bounds no acceleration for the census's second-order clear and tangency floor, so an edge-edge record at a tangency of a spline edge refuses on the budget
status: open
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
