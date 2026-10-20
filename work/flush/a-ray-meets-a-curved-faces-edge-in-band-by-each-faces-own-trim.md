---
id: a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim
kind: issue
title: A ray in band of an edge a curved face bounds, or of a planar face's curved edge, is still set aside by each face's own trim walk
status: open
opened: 2026-10-19
priority: P2
cost: H
---


Filed by the edge-meeting ray unit
(`a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`), which
built D10's edge-meeting ray for line edges between two planar faces.
The curved half of that row is unbuilt.

## What is left

- `boolean::solid_contain::ray_crossings` decides an edge once, for both
  faces, only where it is a line edge and both of its faces are planar
  (`edge_meets`, through `census::line_passes`). A planar face with a
  curved edge (a cylinder's cap), or one beside a curved face, still
  takes its own in-face walk (`point_in_face`, `point_in_loop_projected`),
  and every curved face its own trim (`wall_hit`,
  `point_on_cone_in_face`, the sphere and torus regions). A crossing in
  band of such a boundary sets the ray aside, as before.
- The row's two curved readings are owed:
  - two faces on one carrier (a seam split) share one comparison of the
    root against the seam parameter;
  - an edge between two carriers needs one signed offset of the ray from
    the edge near the crossing, per edge and ray.
  Where a carrier pair cannot give that number soundly, the ray refuses
  typed, never approximates.
- The curved aims' exact ties at a nominal stay with it
  (`the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`):
  `cut_cylinder`'s reader still reads 90 exact zeros at its nominal
  (`curve_foot_branch` 12, `point_in_arc_loop_side` 6, the rest structural
  wall and chart reads), measured by
  `crates/editor-core/tests/revalue_corpus.rs`
  `reader_ties_at_the_nominal`.

## The rule that carries over

A face the ray meets at two edges, or two met edges sharing a vertex, is
a wedge and reads nothing; an edge's half crossings span its faces'
plane crossings (`RayHit`, `Crossings::push_span`). A curved edge's
reading has to keep both: the planar build read a wrong side on an
inside-out needle before the wedge rule, and a wrong material side near
a pyramid's vertex before the span.
