---
id: boolean-readers-at-a-cone-apex-vertex-escalate-under-their-own-reading
kind: issue
title: two boolean readers of a face's normal at a vertex read a cone's apex as NaN and escalate under their own reading, not NormalAtConeApex
status: open
opened: 2026-10-28
priority: P3
cost: E
---


## What

Found by the sweep of `splitdoor/cone-apex` (every reader of a face's
normal at a vertex in `crates/topo/src`). The shared sector walk now
reads a cone face at its apex as its rulings (`sector_face::VertexFace`)
and the Boolean's sector lane refuses it `NormalAtConeApex`. Two
Boolean readers take a cone's normal at a point that can be the apex
without going through that walk or `face_normal`'s gate:

- `boolean/edge_join.rs` reads `geom_brep::classify_dihedral` at the
  vertex `w` two edges meet at; at a cone's apex the gradient is `0/0`
  and the reading escalates `dihedral_wedge` (`NoTangentPlane`), under
  the join's regularity reading;
- `boolean/contact_verify.rs` samples a contact carrier at its ends
  (`schedule_param` includes `t0` and `t1`) and reads
  `implicit_outward_normal` there; a contact along a ruling that ends
  at the apex reads a NaN normal.

Both refuse, but under the reading's name rather than the apex's.
Unmeasured: no fixture is known to reach either at an apex.

## Where to look

Whether each should read the apex arm (`VertexFace::normal_along` along
the edge or carrier) or refuse `NormalAtConeApex` by name.

A third reader, found in the fix pass of PR 5956:
`boolean/ruled_region.rs` `outward` takes the normal at the point where
a leg lands on a boundary edge (`region::offer`, which reads it as the
landing's `face_side`). That point is not a vertex, but a landing exactly
at a cone's apex would read a NaN normal there. Unmeasured.
