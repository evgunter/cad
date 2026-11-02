---
id: boolean-reads-two-faces-coplanar-three-ways-at-three-levers
kind: issue
title: the boolean decides 'these two faces are coplanar' three ways, at three levers
status: open
opened: 2026-10-10
priority: P3
cost: M
---

Filed by PR 4564's review (S1). One question, whether two faces meeting
at a vertex lie on one plane there, is decided at three sites, each
spelled its own way and levered at its own length:

- `crates/topo/src/boolean/sectors.rs` `pair_search`:
  `bool_faces_parallel`, `|n × n'|` (unsigned, so opposed normals read
  parallel) levered at the pair's shorter sector arm; a Zero sends the
  pair to the carrier ladder as a coincidence.
- `crates/topo/src/boolean/vtxfac.rs` `classify_vertex_on_face` (the
  pierce lump): `bool_sector_coplanar`, `|n × n_pierced|` levered at the
  sector's arm, after both bounds read On.
- `crates/topo/src/boolean/sectors.rs` `wedge_classes`:
  `bool_corner_one_plane`, `|n − n'|` (signed: opposed normals stand two
  apart) levered at the farthest reach the reading classes.

## Shape of a fix

Decide which lever each site's verdict is used at (the shorter arm
where the verdict governs only the corner, the farthest reach where it
governs edges reaching past it), and whether opposed normals are one
plane for that site's purpose. Then give the question one door that
takes both as arguments, so a fourth site cannot invent a fourth
spelling. Measure what moves at each site first: `pair_search`'s
unsigned reading feeds the coincidence ladder, which reads orientation
itself.


## The third site retired (2026-11-01, ONEREAD)

`wedge_classes` and its `bool_corner_one_plane` reading are deleted by
ONEREAD's `the-boolean-reads-the-other-operand-through-one-cone`
(branch `oneread/boolean-one-cone`): the boolean's vertex-vertex rows
now read each edge through one gathered cone (`topo::link::Cone`),
whose signs are points' distances from planes. Two sites remain,
`pair_search`'s `bool_faces_parallel` and the pierce lump's
`bool_sector_coplanar`.
