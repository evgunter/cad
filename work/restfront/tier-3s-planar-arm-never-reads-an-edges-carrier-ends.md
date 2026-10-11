---
id: tier-3s-planar-arm-never-reads-an-edges-carrier-ends
kind: issue
title: Tier 3's planar arm reads an edge's ends at its vertices and its carrier only between them, so a carrier end up to 2ε off an adjacent plane passes
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

Found by the sweep of `cleave/chord-end-sample`
(`work/cleave/a-chords-end-sample-reads-carrier-t1-not-its-vertex.md`),
which keeps `EdgeCurve::certify`'s check 4 reading every sample, the two
ends included, on the carrier: the certificate bounds the carrier (D4
¶2), and a vertex on a surface does not put a carrier end ε from it on
that surface (DESIGN.md Q1: "No decision is inferred from two others").

Tier 3's planar arm reads the other way. In `topo/src/validate.rs`
`validate_geometric`:

- check 3 (planar-face vertex residuals, `ValidationError::PlanarFaceResidual`)
  reads each vertex of a planar face against its plane;
- check 5 (planar-boundary containment, `ValidationError::PlanarBoundaryResidual`)
  reads the edge's carrier against each adjacent planar face's plane,
  but only at `geom_brep::interior_stations` (samples 1…7): "the
  between-vertices counterpart of check 3".

So nothing at rest reads a carrier's END against an adjacent plane. Where
the edge's description is stated against that plane (an `Intersection`
with it, a `Chart` on it), certification already covers the end. Where it
is not (a `Chart` on the other face's surface, say), the end is bound to
the plane only by the inference above: vertex ≤ ε off the plane (check 3)
plus carrier end ≤ ε off the vertex (certification's check 3), up to 2ε.

`topo/src/attach.rs` `edge_on_plane` (TOPO's ground) asks the same
question the same way before a re-chart (ends at the vertices, interior
at `CERT_SAMPLES` 1…7), by its doc "tier 3's planar residual checks";
it should follow whatever this row decides.

Unmeasured: whether any minting path reaches an edge whose carrier end
lies in band or definitely off an adjacent plane while its vertex and
interior stations read zero. The shape a witness would take, not yet
built: an edge between a planar face and a curved one, described by a
`Chart` on the curved surface, its carrier's end pulled off the plane
within certification's endpoint pin.

## Owed

Read the carrier's two ends in check 5 as well (all nine stations, the
ends against the plane like the interior), and in `edge_on_plane` with
it; or argue that the vertex reading is what the planar arm certifies.
Pin with a hand-built witness at each ε row.
