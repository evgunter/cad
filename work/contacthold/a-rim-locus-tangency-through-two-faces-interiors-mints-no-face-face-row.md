---
id: a-rim-locus-tangency-through-two-faces-interiors-mints-no-face-face-row
kind: issue
title: A glued tangency along a circle (a rim locus) that runs through both faces' interiors mints no face–face row: the interiors read runs on line loci only
status: open
opened: 2026-10-13
priority: P2
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
