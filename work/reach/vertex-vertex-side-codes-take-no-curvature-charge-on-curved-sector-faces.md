---
id: vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces
kind: issue
title: The vertex-vertex lane reads a bound's side against a curved sector face's tangent plane with no curvature charge, where the pierce lane charges it
status: open
opened: 2026-10-01
priority: P3
cost: M
---

## What

Two lanes read a sector bound's side against a face through the same
primitive, `boolean::sectors::side_code`, and disagree about whether
the face's curvature is charged.

- The pierce lane (`boolean::vtxfac`, the `side_code` call that builds
  `entries`) passes the pierced face's smallest radius of curvature
  (`geom_brep::min_radius_of_curvature`). A definite first-order
  verdict stands only where some point of the bound clears the band
  after the sagitta (`side_code`'s doc).
- The vertex-vertex lane (`boolean::sectors::pair_codes`) passes
  `NO_CURVATURE` against the other sector's `normal`, and
  `NO_CURVATURE`'s doc calls that reference "a FLAT datum". But
  `sector_face` admits `Cylinder`, `Sphere` and `Torus` sector
  carriers, so on a curved face that normal is the tangent plane's,
  and no charge is taken.

So a bound departing a curved sector face within about
`2·sqrt(band·R)` of tangent reads a definite side in the vertex-vertex
lane, and refuses `CurvedSectorSideUnsupported` in the pierce lane.

## Unmeasured

Found by reading the code (the REACH slab-cut sweep, 2026-10-01). No
fixture has been built that reaches `pair_codes` with a curved sector
face and a near-tangent bound, so it is not known whether the missing
charge produces a wrong body, a refusal elsewhere, or nothing. A
definite first-order sign is the bound's correct side near the vertex
whatever the curvature, so what is in question is whether that side
is resolved at the band, which is what the pierce lane's charge
certifies.

## Home

REACH (it claims `crates/topo/src/boolean/sectors.rs`, with CLEAVE,
GERM and HONE). First step: build a vertex-vertex contact between a
curved face and a bound that leaves it nearly tangent (a box corner
touching a cylinder wall, the box's edge tilted to within a few `1e-5`
of the wall's tangent plane) and read where it lands.
