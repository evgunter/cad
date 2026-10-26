---
id: the-plane-nurbs-transversality-reads-a-jet-on-a-crease
kind: issue
title: The plane x NURBS lane's transversality hook reads the wall's jet at a foot with no crease check
status: open
opened: 2026-10-26
priority: P2
cost: E
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral]
---



Filed by the SPLICE lane that built `geom_brep::dihedral_side` (PR 5764),
from its sweep. The same lane's review found the surface form of a
crease defect there: a foot on a C⁰ knot line reads the jet of
whichever span `ders` locates, which need not be the face's.

`plane_nurbs_limbs` in `crates/geom-brep/src/edge_nurbs.rs` (~:615) reads
`wall.ders(u, v)` (~:655) at each interior schedule foot for its
transversality sine (`normal_angle_sine`), with no check that the foot
is off the wall's crease runs (`KnotStructure::crease_runs`). Its
interior samples avoid the edge's ends, but not an edge that rides a
crease, or crosses one, where a sample's foot lands on the line. A
wall's normal from the wrong span there can read a sine past the band
for an edge that is not transverse on the face's own side, or the
reverse.

Not measured: no fixture yet puts a plane × NURBS edge on a C⁰ wall
crease. The guard `dihedral_side` uses is `on_a_crease` in
`crates/geom-brep/src/dihedral_side.rs`, with `irregular_reach` for an
irregular foot,; the same reading, or a typed
refusal for a foot within the band of a crease, would close it.
`nurbs_pair_limbs` (~:1253, the sine at ~:1297) reads two walls' jets the
same way. The class is `splice/a-derivative-is-read-at-a-foot-no-one-placed`.
