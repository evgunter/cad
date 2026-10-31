---
id: a-wide-sectors-subdivision-ray-along-the-planes-trace-reads-in-the-band
kind: issue
title: A wide sector's subdivision ray lying along the plane's trace reads its side in the band, so a cap crossed cleanly along a diameter refuses SliverSector near an On vertex
status: open
opened: 2026-10-11
priority: P2
cost: M
---


Found by the near-vertex family of
`a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it`
(branch `cleave/wrap-edge-split`).

## What

The one-segment cylinder (r = 1 about the z axis, z ∈ [0, 2], its
vertex at `(1, 0, 0)`) split by the plane through that vertex with
normal `(0, 0.8, 1)`, moved `+0.6ε` along its unit normal, refuses

`Reduce(SliverSector { vertex: VertexKey(3v1), face: FaceKey(3v1),
reading: Escalated(margin −1.2e-9, predicate split_bisector_side) })`

at ε = 1e-9 (and its scaled twins at 1e-6 and 1e-12). The two-arc
cylinder (vertices at `(±1, 0, 0)`) builds the same pose. Measured over
`a_split_through_a_wrap_edges_vertex.rs`'s split family:

- the poses whose normal leans across the vertex (`β = π/2`), through
  the bottom or the top vertex, both phases, both senses: 48 refusals,
  at ±0.6ε and ±0.9ε for `(0, 2.5, 1)` and at ±0.6ε for `(0, 0.8, 1)`
  (whose ±0.9ε rows both forms refuse on `split_conic_crossing_root`),
  the margin `∓2 × offset` whatever the lean. The two-arc form builds
  all 48;
- the boolean against the half-space `z ≤ z₀` through the vertex, the
  cylinder tilted about x by ±π/4, ±0.9 and ±1.3 rad (the same lean):
  64 refusals `Escalated { decision: Coincidence(SectorSide, Moot) }`
  on `enters_material`, margin `∓2 × offset`; the two-arc form refuses
  the same 64 on `bool_join_arc_ahead`, a reading not traced here.

## Reading (a theory, unmeasured)

The plane's trace on the cap runs along a diameter, `0.96ε` from the
vertex for `h = 0.8`. On the one-segment cylinder the far end of that
diameter is a vertex the reduction inserts at the computed crossing; on
the two-arc one it is the existing vertex `(−1, 0, 0)`. The cap's
sector there is straight, so `sector_shape` subdivides it along
`n × b`, the radius inward, and `split_bisector_side` reads that ray's
side at its arm (2, the diameter). On the two-arc form the ray is
exactly in the plane, a Zero; at an inserted crossing the ray leans by
the crossing's own `~ε` offset, and its tip lands `2 × offset` from the
plane, in the band. The cap is crossed cleanly along the diameter, its
two bounding edges definitely on opposite sides, so the escalation
reads the subdivision device rather than the geometry: any interior
direction is a valid subdivision (`neighborhood.rs` module docs), and
one clear of the plane would decide. Confirm which vertex `3v1` is
before acting on this.

Pinned only as "refuses typed in the band" by the near rows of
`a_split_through_a_wrap_edges_vertex.rs`; a fix adds its own rows.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a subdivision ray along the trace reads in band at ±0.6ε: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Also measured: the Boolean's coplanar overlap (2026-10-29)

Branch `splitdoor/arm-vs-reach`, while building
`crates/sweep/tests/a_long_bound_beside_a_short_arm.rs`: with the boss's
quad `(0, 0), 0.4·(cos θ, −sin θ), (0.4, 0.3), s·(cos 45°, sin 45°)`
(`θ = 10⁻²`, `s = 300ε`) on the plate holed by
`(0, 0), (0.5, 0), (0.5, 0.5), (0, s)`, every op at ε 1e-6 on that
branch's head (main refuses earlier, at the shared corner) refuses
`Escalated { decision: Coincidence(Sectors, Moot) }` on
`bool_sector_within`, margin `±6.58e-6`. The site is the vertex where
the boss's edge to `(0.4, 0.3)` crosses the hole's edge `y = 0`: the
plate top's corner there is straight, split at the bisector `(0, −1)`
(`Reach::Bisector(0.1)`), and `boolean/sectors.rs` `sector_overlap`
reads the boss's edge, `6.58e-5` rad off that bisector, against it at
the bisector's reach. The bisector is no geometry, so the in-band
reading refuses a pose nothing in the operands puts in band. Moving
the boss's vertex to `(0.35, 0.25)` clears it.
