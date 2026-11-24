---
id: a-cone-faces-corner-of-two-arcs-of-one-section-is-read-by-none
kind: issue
title: A cone face's corner of two arcs of one section ellipse is read by none: Reading::Conic is shut on a cone, since no body has such a corner to argue it on
status: open
opened: 2026-11-24
priority: P3
cost: C
---



## Finding

From `embed/cone-image` (2026-11-24, the last pass). `reading`
(`crates/topo/src/injective.rs`) gives a corner `Reading::Conic` where
the edge read is a curve `present_curved` reads and the other edge is
its continuation through the vertex along one conic (`continues`). Once
a cone's chart read a section ellipse's focal image (`reads_curve`),
that arm was open on a cone face too. No body reaches it there:

- a section ellipse split by the revolve door's two rulings puts ONE arc
  on each half of the wall; the two arcs meet at the rulings, on
  different faces;
- measured on a scratch merge with `embed/conic-symmetry` (the measuring
  branch `embed/cone-image-measure`): 0 firings of the arm on a cone
  face in 21 cut cones and in the 24 cone/cone census tests, against 69
  to 287 per body on the plane cap.

So the reading on a cone had neither a positive nor a negative row, and
it is shut: the arm excludes `Surface::Cone`, and the corner falls to
the arms below it, which read none. Pinned by
`injective::focal_rows::a_cone_faces_corner_of_two_arcs_of_one_section_is_read_by_none`.

**Band P3**: latent, and in the safe direction. Nothing reads wrong and
no body that exists is refused by it. The day a body has two arcs of one
section meeting at a vertex inside one cone face (a third edge ending
on the section there: a cone face split along a curve that is no
ruling, or a one-face cone whose section is cut by another body's edge),
that body refuses `Structural` at the corner, and a blocker then takes
the band of what waits on it.

## What a fix needs

The body first. Then the arm opened on a cone with its argument made on
the cone's chart: `present_curved`'s run through the vertex along the
other arc, both arcs focal images of one stored section, the cell off
the apex and under a turn wide. A positive body row and a negative (two
arcs of two different ellipses at one vertex of a cone face, which must
keep reading two) land with it.
