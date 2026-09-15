---
id: sense-sign-multiplies-fold-onto-outward-normal
kind: unit
title: The ten hand multiplies of normal by sense_sign fold onto OutwardNormal, and Face::sense_sign retires
status: open
opened: 2026-09-15
---


## What

`D6`'s ruling (PR 2457), second unit, after
`sense-sign-doors-take-the-bit`. The ten `normal * face.sense_sign()`
sites (`emit_topo.rs` `face_plane`, `blend/build.rs` `outward_of`,
`blend/battery.rs` `outward`, `boolean/join.rs` `ring_run_ccw`,
`boolean/rest.rs` `face_carrier`, `boolean/solid_contain.rs`
`face_plane` and `face_geo`, `validate.rs` check 6, `merge_faces.rs`
`merged_outline_ring` and `planes_declared_equal`, `mesh/walk.rs`
`loop_polygon`) fold onto `OutwardNormal::from_chart` /
`face_outward_normal`; where a scalar (an area, a winding) is genuinely
signed, the bit is spelled as a conditional negation. Then
`Face::sense_sign<T>()` retires with `face_normal.rs`'s hand-kept
census. Pin: D9 bit identity over the touched crates' suites. Many
programs' ground (TOPO, BLEND, WIRE, S-MESH, BOOL); announce each.
