---
id: a-round-tube-standing-on-a-plate-refuses-seam-orientation
kind: issue
title: A round tube standing on a plate refuses SeamOrientation in every member order
status: closed
opened: 2026-09-30
priority: P0
cost: H
closed: 2026-10-06
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

## Closed (ZIP, 2026-10-06)

It builds. Rebuilt in the kernel through `profile::circle_split` and
`sweep::extrude`, the scene refused at the probe branch's base
(`90c5fb0bcc`): `plate ∪ tube` with
`SeamOrientation { a_face: FaceKey(9v1), b_face: FaceKey(15v1) }`,
`tube ∪ plate` with
`SeamOrientation { a_face: FaceKey(15v3), b_face: FaceKey(3v5) }`, and
the slab orders with one of those or `CurvedSectorSideUnsupported`. On
`eab9149b17^1` (2026-10-01) both pair orders refused
`Join(SectionLoopMixed { face: FaceKey(9v1) })` instead, and from
`a454967521^1` (2026-10-02) on they build. The change that cleared the
`SectionLoopMixed` lies in the first-parent window
`eab9149b17..a454967521^1`. The `SeamOrientation`-to-`SectionLoopMixed`
step lies in `90c5fb0bcc..eab9149b17^1`. Neither window was bisected
further.

On main, both pair orders and all six orders with the slab (top at
`z = 1.03` and at `z = 1.5`) build through tiers 2 and 3′ and the at-rest
certificate, at the closed form `6 + 0.2π` plus the slab's share
(`crates/sweep/tests/round_tube_on_plate.rs`). The editor's
`Boolean` and `Union` nodes over the same members build at the same
volume and mint names in both orders. The engraved annular sector's
row closed under JOIN-3 and its pinned row
(`an_engraved_one_arc_c_builds_at_every_sweep`) is green on main, so
nothing is left to compare at a first wrong step.

A sweep over rim splits 2 to 5, bore splits 2 to 4, four rim phases,
three bore phases and three heights (the tube on the top, through the
plate, through the bottom), under ∪ both ways, ∖ both ways and ∩,
gave 2160 builds, every one at its closed form. The 144 `plate ∖ tube`
cuts right through the plate leave a holed plate and a loose plug, and
tier 3′ refuses each of them with `CensusUndecidable`, while the 16-gon
twin passes. That is the census's cross-solid curved gap, and it is
recorded on CONTACT's
`census-cross-solid-curved-pairs-undecidable-on-shell-results`.
