---
id: a-conics-incidence-is-read-over-its-full-turn-not-the-edges-span
kind: issue
title: A conic's incidence is read over its full turn, not the edge's span, so an arc on its face reads as a definite miss
status: open
opened: 2026-10-28
---



Found by review lane b on PR 5889 (E1c); filed by E1c's fix pass. The
full-turn read is older than E1c on the sphere, cone and torus arms. On
the plane arm E1c made it: before E1c the plane mint read no incidence
and refused such an arc as a residual; since E1c it is the read's
definite miss (measured below). The orchestrator left the repair out of
E1c.

## What happens

The mint's incidence test samples a closed conic over its whole turn
`[0, τ]`: `chart_incidence` in `crates/geom-brep/src/pcurve_cache.rs`,
which `incidence::incident` and `cone_conic_incidence` call. It reads
the carrier, not the edge: it does not read the placement's span
(`Placement::span`), which a line's test already samples
(`line_incidence`).

So an arc that lies on its face is read off the face when the rest of
its circle leaves it. The read is a definite miss: the kernel's defect
at a build, and the stored culprit at rest.

## Measurement (E1c fix pass, f64, ε = 1e-9)

The chart is the plane `z = 0`. The carrier is the unit circle about the
origin, its axis tilted `1e3·ε` about `x`, edged over `t ∈ [−1e-4, 1e-4]`.
- The arc's farthest point is `1.0e-10` off the plane (`0.10·ε`).
- `chart_pcurve_over(circle, −1e-4, 1e-4, plane)` and `EdgeCurve::certify`
  over that span both refuse `CarrierOffChart { margin: Past(1.0e-6) }`,
  which is the full turn's farthest point.

Before E1c the plane mint read no incidence; the review measured base
refusing the arc as a residual. Since E1c it is the read's definite miss.

## Repair shape

Sample a conic over the edge's span where the placement carries one, as
a line's test does. A carrier with no span (a whole-turn mint) is read
over its turn.
