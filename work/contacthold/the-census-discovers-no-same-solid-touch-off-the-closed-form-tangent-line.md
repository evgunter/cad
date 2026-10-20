---
id: the-census-discovers-no-same-solid-touch-off-the-closed-form-tangent-line
kind: issue
title: Inside one solid the census discovers a touch with a curved cell only along a closed-form tangent line: point touches, rim loci and distinct-key conformal pairs go unread
status: open
opened: 2026-10-20
priority: P2
cost: H
refs: [the-census-discovers-no-same-solid-tangent-touch-through-face-interiors, a-rim-locus-tangency-through-two-faces-interiors-mints-no-face-face-row, a-two-carrier-curve-touch-has-no-bounded-locus]
---



Found sweeping for the class while building
`the-census-discovers-no-same-solid-tangent-touch-through-face-interiors`
(`contacthold/same-solid-discovery`).

## What

That unit built C2's same-solid discovery for the line lane only
(`census::sweep_same_solid_tangent_lines`: two faces of one solid on
distinct carriers whose closed-form tangent line runs through both).
Every other touch a curved cell takes part in inside one solid is
confirmed where a record names it and discovered nowhere:

- **A point touch.** A sphere on a plane, two spheres (face–face), or a
  curved edge's interior on a face (a cap circle kissing a plane) are
  read by `census::point::face_face` and `census::point::edge_face`,
  which only `confirm_face_pair` and `confirm_edge_face` call. Ev's
  ruling (`[ev]` PR 5395) makes a shared crescent at a point legal
  inside one solid, so a point touch's sides never refuse, but a void
  one no record names still owes `UnrecordedContact`.
- **A vertex or a line edge on a curved face.** `sweep_vertex_face` and
  `sweep_edge_face` walk `Geo::faces`, the planar snapshot, and
  `sweep_edge_face` line edges only.
- **A rim locus** (a tangency along a circle through both interiors):
  no closed-form line, so `boolean::tangent_pair_at_rest` answers
  `RestTangent::NoLine` and the new sweep skips it. The op side is
  `a-rim-locus-tangency-through-two-faces-interiors-mints-no-face-face-row`.
- **Two distinct-key faces on one geometric carrier** (the conformal
  question with two descriptions): `sweep_conformal_patches` groups by
  chart key, and the new sweep reads only a closed-form tangent line.

Cross-solid, each of these is the backstop's (`sweep_cross_solid_backstop`,
which returns at once for a one-solid body).

## Owed

The rest of C2's C9 ring for same-solid pairs (`crates/topo/README.md`
C2 and Open): discover each class above and refuse what no record backs,
reading sides only along a line (a point's crescent is legal, C4).
