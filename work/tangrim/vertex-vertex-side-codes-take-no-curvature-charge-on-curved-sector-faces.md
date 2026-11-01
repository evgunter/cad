---
id: vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces
kind: issue
title: The vertex-vertex lane reads a bound's side against a curved sector face's tangent plane with no curvature charge, where the pierce lane charges it
status: open
opened: 2026-10-01
priority: P3
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
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
`2·sqrt(band/R)` radians of tangent reads a definite side in the
vertex-vertex lane, and refuses `CurvedSectorSideUnsupported` in the
pierce lane.

## Measured, partly

Found by reading the code (the REACH slab-cut sweep, 2026-10-01). The
PR's dual review then ran 16 vertex-vertex touches (a bar corner on a
cylinder's rim vertex, one edge leaving within `1e-2` to `1e-6` of the
wall's tangent plane): every one that built was correct, and from
`1e-4` down they refused (`CurvedPierceUnsupported` or an escalation).
Which of them reached `pair_codes` was not traced, so whether the
missing charge can produce a wrong body is still unknown. A
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


## Lane finding (2026-11-01, `tangrim/sector-passes-descend`, PR 6253)

Built and measured, then backed out. With `BoolSector` carrying its
face's lever (`PierceLever::of` at the vertex) and `sectors::sector_code`
charging a definite side as `pierce_side` does, three rows of
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs` that build on
main refused `CurvedSectorSideUnsupported { verdict: Zero(..) }` at the
default ε, the charge's margin decided zero:

- `a_boss_inside_a_hole_sharing_a_vertex_beside_the_tip`: `s = 1e-6`,
  `φ = 0`, tip, A ∪ B, margin `3.0e-13`;
- `short_arcs_at_a_crescents_tip`: both, `s = 1e-5`, A ∪ B, margin `1.0e-11`;
- `short_arcs_at_a_turned_tip_at_the_finest_band`: inside, `φ = 0`,
  A ∪ B, margin `9.0e-15`.

(Measured in the first minutes of the full run; the rest of the suite
was not run with the charge.) The charge is direction-agnostic on
purpose (`bound_side::read`'s doc): it refuses a bound leaving within
about `2·sqrt(band/lever)` of tangent, or too short to witness its slope,
whichever way the face bends. On these short arcs at a crescent's tip
that refuses bodies main builds valid. So the design's item 2, first
bullet ("so the vertex-pair pass gets the MAJ-1 charge too"), is not
landable as specified: it needs either a charge that reads which way the
face bends (no charge where the face bends away from the verdict's side),
or a first-order verdict the charge does not clear read one order down
as a tie is, which is a change to the design and goes to the
orchestrator first.
