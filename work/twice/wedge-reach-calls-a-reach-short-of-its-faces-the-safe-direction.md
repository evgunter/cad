---
id: wedge-reach-calls-a-reach-short-of-its-faces-the-safe-direction
kind: issue
title: The wedge reach calls a reach short of its faces the safe direction, but the dihedral acts on a Zero
status: open
opened: 2026-10-27
priority: P2
---


`crates/topo/src/wedge_reach.rs`'s module docs say the wedge reach,
read at each face's boundary vertices, "falls short of the boundary by
at most a curved edge's bulge past its ends, and of a curved face that
stands farther inside its boundary than on it, both the safe
direction". The reach levers the dihedral between the edge's faces
(`geom_brep::folded_lever_arm`'s `reach`), and the dihedral's Zero is
acted on: two faces read Smooth (the edge a tangent seam, or a sliver
refused as one) where their normals' angle times the reach is within
ε. A reach short of the faces lets an angle Positive at their true
reach read a definite Zero, so short is the unsafe side for that
reading, as `the-wedge-reach-counts-a-faces-run-along-its-edge` finds
long is for the other one.

The curvature arm caps the lever at the face's radius, which bounds
the shortfall's effect on a curved face only where that radius is the
smaller arm. Where it is not (a plane face bounded by a long shallow
arc, whose bulge past its end vertices is the sagitta of the arc), the
reach stands short by up to that bulge. Same class as OFFSET's
`edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`
(the edge's own extent, now `geom_brep::edge_extent_inf` and
`geom_brep::edge_extent_sup` by the side each reading needs): the fix
reads each curved boundary edge at a bound from above (its
`edge_extent_sup` ball about a point of it, or its sound box), and
corrects the doc. Found by OFFSET's sweep for the "shorter is safe"
claim.
