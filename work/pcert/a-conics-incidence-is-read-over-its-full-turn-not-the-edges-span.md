---
id: a-conics-incidence-is-read-over-its-full-turn-not-the-edges-span
kind: issue
title: A conic's incidence is read over its full turn, not the edge's span, so an arc on its face reads as a definite miss
status: open
opened: 2026-10-28
priority: P2
cost: M
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

## 2026-11-02: the planar tier reads the edge; the mint keeps the turn

E1c's delta review (N3) found that M4 (the planar tier deciding through
the mint's read) brought this into the planar tier. Before M4 the tier
read the edge's own stations. After it, an arc-bounded flat face whose
circle leaves the plane past the arc's ends validated at rest as the
stored culprit (the review's P4, this item's own example, both lanes
`CarrierOffChart` `Past(1.0e-6)`).

**Fixed for the planar tier.** `geom_brep::read_edge_incidence` reads a
conic over the edge's span, and the planar tier reads by it. Row:
`chart_incidence::an_arc_on_its_face_is_read_on_it_by_the_edges_own_read`,
f64 and Interval.

**Kept for the mint and the certification**, by the orchestrator's
ruling: they still read a conic over its whole turn. Here is the
measured cost of reading the span there. Wall time is at CI's
opt-level 1, `--test-threads 1`, main `19032ea883` against the head
with E1c's one-read-per-carrier change (N2):

| reading of a conic | m10_9 pad | sym11 | plate `Plain/Decision` forms |
|---|---|---|---|
| main | 2.32–2.46 s | 4.79–5.18 s | 20618 |
| whole turn (E1c head) | 3.30–3.77 s | 7.18–7.75 s | 60682 |
| span, 64 samples | 5.55–5.67 s | 10.3–10.6 s | 59793 |
| span on plane charts only | 3.52–4.17 s | 7.65–8.22 s | — |

Over the turn, every arc of one circle samples the same points and
decides the same farthest distance, and the symbolic tier's memo
shares both. Over each edge's span, each arc builds and decides its own.

Reusing the turn's sample grid inside a span would keep the sharing,
but selecting the grid points inside a span reads the parameter's
enclosure bounds, which the Bounds gate forbids. Fewer samples are a
soundness and coverage trade, not a cost fix, so they were not taken.

**Open for the mint and certify**, on every chart: an arc on its face
whose circle leaves the face past its ends is still read off at a build
and in a stored row's certification.

## 2026-11-03: `topo::chart_region`'s plane path disagrees with the planar tier

E1c's round-N verification found a third reader of a flat face's arc.
`topo::chart_region`'s plane path (`loop_uv_polygon`,
`crates/topo/src/chart_region.rs`) images a flat face's trim edge on
demand by `geom_brep::chart_pcurve_over` over the edge's span. That is
the mint's imaging, so it reads a conic over its whole turn.

On this item's arc (the unit circle tilted `1e3·ε`, edged over
`[−1e-4, 1e-4]`, on `z = 0`) it refuses `CarrierOffChart` at the full
turn's farthest point, `Past(1.0e-6)` at ε 1e-9, at both lanes and every
ε the row runs.
`chart_region` ends that as `TrimOffFace`: the stored culprit at rest.
The planar tier's `read_edge_incidence` reads the same arc on its face.
So on a flat face the two at-rest readers disagree.

Under the orchestrator's ruling (a) this is the mint's imaging, which
keeps the whole turn, so `chart_region` was not changed in E1c; the fix
rides this item. The disagreement is pinned by
`chart_incidence::chart_regions_plane_path_reads_an_arc_over_its_whole_turn_where_the_planar_tier_reads_its_span`,
which this item's fix has to flip.
