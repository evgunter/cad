---
id: shell-clearance-footprint-reads-vertices-not-arcs
kind: issue
title: wall_clearance's planar footprint box is folded over boundary VERTICES only, so an arc bowing out of a face's vertex hull is outside its footprint and a crossing pair can read as separated
status: open
opened: 2026-09-26
---


Found by reading, during CONTACT-3's sweep for regions folded from
boundary vertices (the shape of the cylinder wall trim's defect). Not
yet witnessed by execution.

`wall_clearance` (`crates/topo/src/shell.rs`) skips an antiparallel,
non-adjacent planar pair when `footprints_may_overlap` finds their
projected boxes definitely separated. Each box is `planar_faces`'
fold over `face_boundary_points`, and that function returns the start
VERTEX of every half-edge — nothing on an edge's interior. A planar face
bounded by a circle or ellipse arc is not inside its vertex hull: an arc
bowing outward carries region past the vertices, and an extruded disc's
cap has only two vertices, so its box has zero width across the chord.
The gate's doc claims it "cannot miss a planar pair that crosses";
that holds only for faces whose footprint IS their vertex hull (every
edge a line).

A witness would put two antiparallel arc-bounded planar faces of one
solid across a gap under `2t`. Their vertex boxes would be separated by
more than `2t`, but their arcs would bow into each other's footprint. A
fix could fold each edge's own extreme into the box, using its carrier
the way `point_in_carrier_loop`'s reach does
(`crates/topo/src/splitting/containment.rs`, `loop_reach`). It could
also count any non-line edge as "may overlap", which is the conservative
(#571) direction.
