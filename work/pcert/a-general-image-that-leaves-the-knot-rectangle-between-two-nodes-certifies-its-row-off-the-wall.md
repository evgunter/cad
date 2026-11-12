---
id: a-general-image-that-leaves-the-knot-rectangle-between-two-nodes-certifies-its-row-off-the-wall
kind: issue
title: certify_general certifies a pcurve row whose image leaves the wall's knot rectangle between two nodes: the carrier up to 1e-5 m (1e4 eps) past the rim reads on the wall's polynomial extension
status: open
opened: 2026-11-12
priority: P0
cost: M
---

Filed by the fix pass of PR 6879 (`sectarms/spline-curved-at-rest`), from
a finding of that PR's review. Probed on that branch; the door probed is
main's, unchanged by the PR.

## What

`PcurveCache::certify_general` (`crates/geom-brep/src/pcurve_cache.rs`)
certifies a `Pcurve::General` row on a spline chart by limbs 1 and 2:
the incidence read (`incidence_ended`), the schedule's residuals and the
composite hull of `S(P(t)) − C(t)` (`run_fitted_checks`). None of the
three holds the image `P` to the wall's knot rectangle. Where `P` leaves
it, the composite reads the boundary cell's polynomial extension
(`geom_core::spline::compose::tensor`, "Domain posture"), and the foot
schedule's nodes stand either side of the excursion. So a carrier that
leaves the wall past its rim between two nodes of the image, and stays
on the wall's extension, gets a certified row: the row says the carrier
is within ε of the wall along its whole span, and it is not.

## Measured

The flat bilinear wall `y = 1`, `x ∈ [−1, 1]`, `z ∈ [0, 2]` (`v = z/2`),
and a circle of radius 0.3 in the wall's plane whose lowest point is
`d` below the rim `z = 0`, at fraction 100.5/256 of a span of one
radian. The image is the mint's own (`general_image_lane`), certified
through `certify_general` with `FittedLane::certified()` at `f64`:

| ε | d (carrier past the rim) | `certify_general` | image's lowest `v` (sampled) |
|---|---|---|---|
| 1e-9 | −1e-6 (inside) | Ok | +5.0e-7 |
| 1e-9 | 3e-9 … 4e-7 | **Ok** | −1.5e-9 … −2.0e-7 |
| 1e-9 | 1e-6 | **Ok** | −5.0e-7 |
| 1e-9 | 1e-5 | **Ok** | −5.0e-6 |
| 1e-9 | 1e-4 | `Incidence(…)`: the carrier's points are not on the chart | |
| 1e-6 | 3e-6, 1e-5 | **Ok** | −1.5e-6, −5.0e-6 |
| 1e-6 | 3e-5 and up | `Incidence(…)` | |
| 1e-12 | 3e-12 … 4e-10 | **Ok** | +3.4e-11 … −1.6e-10 |
| 1e-12 | 1e-6 and up | `Incidence(…)` | |

At the default ε the row certifies with the carrier 1e-5 m, ten
thousand ε, off the wall. The incidence read is what ends it: it
refuses once the dip is wide enough for one of its own samples to land
in it.

The image's control hull is no test of this: with the arc a micrometre
inside the rim its lowest control `v` is already −2.2e-5.

## Reach, and what was not probed

- A witness-selected `Intersection` edge on a residual × NURBS pair is
  refused all the same: limb 3's tube is cut to the knot rectangle and
  refuses `TubeNotOneArc { Count { solutions: 4 } }`
  (`spline_implicit::rest_rows::an_arc_that_dips_past_the_walls_rim_between_two_nodes_is_refused_by_the_tube`,
  PR 6879). So the edge certificate of that class is not what is open.
- What is open is every reader of the row alone: an edge that owes no
  limb 3 (a structural selector, an `OnPair`, a `Chart` locus; README C2),
  and the consumers that integrate or tessellate a face through its
  stored rows (the trimmed quadrature lane, the mesher's trimmed path).
- **Not probed:** whether a body can be built whose stored row does
  this (it needs a carrier that crosses a wall's rim and comes back
  within one image span, on an edge that is not held by limb 3), and
  whether tier 3 or the mint's walk (`topo::pcurves`) refuses such a
  row by some other check. No domain check was found there by reading.

## Candidates

- Hold the image to the knot rectangle in the fitted certificate: a
  certified bound of the image's two coordinates over each span
  (subdivided, not the control hull, which over-refuses as above),
  refused where a span's bound leaves the domain by more than ε over
  the chart speed.
- Or decline the extension in `compose::tensor` for this reader: a
  window that leaves the domain refuses instead of being served by the
  boundary cell.
