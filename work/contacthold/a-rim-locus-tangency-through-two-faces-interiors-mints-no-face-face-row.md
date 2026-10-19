---
id: a-rim-locus-tangency-through-two-faces-interiors-mints-no-face-face-row
kind: issue
title: A glued tangency along a circle (a rim locus) that runs through both faces' interiors mints no face–face row: the interiors read runs on line loci only
status: parked
opened: 2026-10-13
priority: P2
blocked_on: [dev1-cylinder-sphere-circle-locus-arm]
---


Found on PR 4921's review (`contacthold/face-face-touch-row`), disclosed
there as a seam.

## What

The glue door's tangency (`boolean::verify_tangency`) reads whether its
locus runs through both faces' interiors
(`boolean::locus_through_both_faces`) only where the locus is a line:
`overlap` is set in the line arm alone, and the rim arm (the
`TangentLocusError::Unsupported` fallback through
`rim_wedge::shared_rim`, `rim_wedge::Locus::Rim`) leaves it `None`, so
`interiors` is `Ok(false)` and `verify_declared_contacts` puts no pair
in `VerifiedGlue::touches`. A glued tangency along a circle whose
interior lies inside both faces (two coaxial surfaces of revolution
kissing along a parallel, e.g. a torus resting in a cylinder's bore
away from either face's rim) mints no face–face row
(`ReductionContacts::ff`), and C7 (`crates/topo/README.md`) records
every curve touch through a face's interior as a face–face record.

Not measured on a fixture; read from the code. Most rim tangencies the
door reaches today run along a face's boundary circle (the edge form,
carried by edge records), which is why the line arm came first.

## Owed

A fixture whose rim tangency runs through both faces' interiors; the
read over the circle (the certificate schedule over `[0, 2π)` or the
faces' common azimuth span); and the face–face row minted from it as
the line case does.

## 2026-10-19 — the gap is earlier than the interiors read

Read on main `ebe4a2b2b2` while starting this unit (`contacthold/rim-locus-ff-row`, nothing pushed). The rim arm's `interiors: Ok(false)` is right, not a skipped read:

- `rim_wedge::shared_rim` matches only where each face has a **boundary** circle on the shared carrier, and `glued_tangency`'s `route_rim` admits only `RimRouting::Seam` with `rim_wedge::departures == Opposite`, which needs an edge along every stretch where the faces touch. A locus through a face's interior is `Departure::NoEdge`, refused `Unrouted(UnsupportedDeclarationClass)`; opposed normals (a kiss) are `RimCuspArmUnbuilt`. So no glued rim tangency has a point inside both faces.
- A circle through both interiors has no witness locus at all: `geom_brep::tangent_locus` derives lines only (plane×cylinder, parallel cylinders), and `crates/geom-brep/src/locus.rs` keeps the coaxial circle arm out until a consumer exists.

Poses, all refusing before any record could be minted:

- A torus (major 2, minor 0.5, axis y) resting on a plate whose top is `y = -0.5`: ∪ in both orders, ∖ and ∩ refuse `CurvedBooleanUnsupported { kind: Torus }`.
- A ball of radius 1 in a bore of radius 1: ball ∪ tube escalates `ArcCylinderRoots` (`bool_conic_quadric_disc`); tube ∪ ball refuses `CurvedPierceUnsupported`.
- A torus in a bore of radius 2.5: both orders refuse `CurvedPierceUnsupported` (the seam meridian crosses the contact circle).

Parked behind TANG's `dev1-cylinder-sphere-circle-locus-arm`, which holds the general form (a tangent locus over a `Curve3`). Once that arm derives the circle, the face–face row is the overlap read over the circle (`locus_through_both_faces` already walks a certificate schedule) at the pair door and the fold step, and the census's `tangent_pair_at_rest` gains the same arm so op and census keep one reading.
