---
id: a-round-tube-standing-on-a-plate-refuses-seam-orientation
kind: issue
title: A round tube standing on a plate refuses SeamOrientation in every member order
status: open
opened: 2026-09-30
---


## What

A hollow round tube standing on a plate's top refuses in the kernel, so
no name is ever minted for it. The plate is `[0,3] × [0,2] × [0,1]`;
the tube is `circle_split(1.5, 1.0, 0.6, 2, 0.0)` with the bore
`circle_split(1.5, 1.0, 0.4, 3, 0.3)`, extruded 1.5 from z = 0.5. The
union of the two, in both member orders, and the union of those two
with a slab `x∈(1.45,1.55), y∈(−1,3), z∈(0.47,1.5)` in all six orders,
refuse:

`Boolean(SeamOrientation { a_face, b_face })`: "seam cycles of faces
… are not antiparallel (orientation chain broke — kernel bug)".

The obstacle-mechanism measurement (branch
`emit/borders-mechanism-probe`, `crates/editor-core/tests/borders_probe.rs`,
fixtures `tube_on_plate`, `tube_slab`, `pair_union_round_tube`) met the
same refusal, and in some of its variants `RingHomingAmbiguous`. The
16-gon prism tube of the same size builds in every order
(`crates/editor-core/tests/emit_union_borders.rs`,
`a_tube_islands_the_top_and_a_slab_divides_tube_and_top`), so the
refusal is the curved lane's, not the configuration's.

The error's own words call it a kernel bug, raised in
`crates/topo/src/boolean/zip.rs` where the seam cycles are paired. It
also leaves the naming layer's curved unions untested: no curved
divider reaches `Borders` today.
