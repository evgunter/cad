---
id: a-torus-or-cone-corner-at-a-vertex-alone-reads-no-quadrant
kind: issue
title: The vertex-alone quadrant reading covers a plane's and a cylinder's charts only: a torus's corner of a parallel and a wall image, and a cone's or sphere's corner, at a vertex the pair shares alone, still read Structural
status: open
opened: 2026-11-17
priority: P2
cost: M
refs: [departure-reads-no-curved-corner-off-its-shared-edge, curved-presentation-reads-plane-and-cylinder-images-only]
---

## Finding

`topo::injective`'s `vertex_corner` admits a corner at a vertex the
pair shares alone on two charts: a plane's line edge and conic, and a
cylinder's coordinate line and harmonic chart curve.
`present_vertex_corner` reads it as the quadrant of an affine chart
about the vertex. Its first axis is the straight edge's direction, and
its second leans off the curve's tangent. The curve is carried into
those coordinates by an affine map of its harmonic form.

Three kinds of chart fall outside it, and `beside_curve` still says
`Structural` for each:
- **A torus's parallel against a spiric wall image.** The line
  readings (`reading`, `Reading::Line`) read this corner with the
  parallel shared. At a vertex alone the quadrant's lean needs the
  curve in the leaned coordinates. A `ChartCurve::Wall` is not closed
  under an affine map: its `u` is an `atan2` of its parameter, and its
  `v` is linear in it. Reading it needs `leaves_line`'s two rates taken
  in chart coordinates and combined, not a mapped curve.
- **A cone's corner.** The chart collapses at the apex, so `R`'s
  affine image must be kept off it.
- **A sphere's corner.** Its curved edges off the axis are neither
  coordinate lines nor harmonic in the chart (`curved-presentation-reads-plane-and-cylinder-images-only`).

The reading is sound on any chart regular over `R` whose edge images
are exact. The cylinder's argument (the PR that landed it) states it
in that form. The guard is `vertex_corner`'s surface match.

## Evidence

None measured. On the tour walk, the f64 cavity rows and the
injectivity suite, no refusal lands on these corners today. The
remaining vertex-alone refusals are `A`'s wall against `B`'s cap in
the tee, where the wall's corner is two arcs of one ellipse
(`departure-reads-no-curved-corner-off-its-shared-edge`'s second
bullet). Filed so the reading's edge is a row, not a sentence.
