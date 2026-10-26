---
id: torus-walk-steps-over-null-edges-where-the-cone-refuses
kind: issue
title: The torus chart walk steps over an uncertified boundary edge and guards the gap, while the cone walk refuses the same shape as CorruptFace
status: closed
opened: 2026-10-08
priority: P3
cost: E
closed: 2026-10-26
---

Found by CONTACT-11's review (Style 3).

Two containment trims meet the same shape, a boundary edge with no
certified curve, and answer it two ways:

- `solid_contain::torus_chart_windows` steps over it as a zero-length
  coincident copy. The walk's continuity and closure checks
  (`bool_torus_chart_closure`) then decide that it was one, and a
  non-degenerate gap refuses as `PartialTorusFace`;
- the cone trim's walk (`chord_join::face_azimuth_images`, under
  `cone_trimmed_window`) refuses the face as `CorruptFace`.

**Why it is not unified in CONTACT-11.** Either change moves a
behaviour outside that unit's question. A torus that refuses at a null
edge would refuse faces on the boolean's working copies that the
join-lane walk steps over today. A cone that steps over one would need
the cone's own continuity check, inside `chord_join.rs`, which is TANG's
territory. The choice is the owner's: which answer a working copy's
null scaffolding should get from a containment trim, and whether the
two walks share it. Either way the rows are
`section_cert_rows.rs` `a_gap_in_the_torus_walk_refuses` and the cone's
equivalent, which does not exist yet.

## Closed (2026-10-26, tracker sweep)

Fixed by `6f920b62` ("topo: fold the sphere ray, the circle leg, the leg
end and the restricted carrier into the region skeleton"), which gave
every containment trim one boundary gather: `region::boundary` with its
`Scaffolding` choice (`crates/topo/src/boolean/region.rs:224-262`).
The torus and the ruled (cylinder and cone) region readers now answer
the shape alike. Each steps over null scaffolding (a zero-length copy of
a vertex), and each reads an uncertified edge between two points as a
gap with no region (`torus_region.rs:254-258`, `ruled_region.rs:372-374`;
the torus pin is `torus_region.rs:1251`,
`a_walls_null_edge_bounds_nothing`). The two walks the row named,
`solid_contain::torus_chart_windows` (with `bool_torus_chart_closure`)
and the cone trim under `cone_trimmed_window`, are gone from
containment, and so is the pin `a_gap_in_the_torus_walk_refuses`.
