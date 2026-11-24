---
id: singular-at-reads-a-spheres-north-pole-first-where-both-poles-are-in-band
kind: issue
title: singular_at reads a sphere's North first where a vertex is decided on both poles, a sphere whose radius is within the band
status: open
opened: 2026-11-23
priority: P3
cost: E
refs: [a-horns-cusp-reads-as-its-upper-apex-so-a-face-below-it-turns-the-wrong-way]
---


Found by CHARTTAIL's horn-cusp unit (PR 7511, 2026-11-23) sweeping for
readers that take the first of several singular points at one place.

`topo::pcurves::decision::singular_at` returns `Singular::On` for the
first point of `geom_brep::chart_singularity::singular_points` that
`vertex_at_singular_point` decides the vertex on. PR 7511 made a spindle
sheet's two apexes one reading (`AnalyticPoint::SpindleApexes`, its side
read off the joint's chart ends). A sphere's North and South are the
other pair the list states: both decide on one vertex only where the
sphere's diameter is within the band, and then the vertex reads `North`
and `face_above` steps the face as below it whatever side it is on.

No row builds such a sphere, and no surface door was found that refuses
one (the sweep looked for a radius guard at the sphere's construction and
found only operation-local ones, such as `intersect.rs`'s
`kos_sphere_radius`). What closing it takes: either show that no door
admits a sphere whose radius is within the band and say so at
`singular_at`, or refuse a vertex decided on both poles typed (no side of
a sphere of no size is the face's). It is a latent wrong reading, not one
any suite reaches (P3).
