---
id: SHELL-6
kind: unit
title: the cone nappe has one home — face_nappe decides once, both offset doors and the displacement read it
status: closed
opened: 2026-09-08
branch: shell/6-nappe-home
refs: [mint-offset-ignores-cone-mirror-nappe, shell-offset-three-followups, 1199, 1180]
pr: 2178
closed: 2026-09-08
---


`geom_brep::offset_surface` on a cone slides the apex along the
OPENING nappe's normal field, so a mirror-nappe face's material moves
`−d` along its own chart normal — a consumer obligation the tree
discharges in one door (`offset_axial::nappe_signed`), skips in the
other (`replace_face::mint_offset`, issue record 1199), re-decides in
the apex-window gate, and reads per point in
`ConeOffset::displacement`'s `copysign`. This unit gives the fact one
home (`face_nappe`, decided from the face's own corner stations) that
every reader reads, and pins that the two doors agree on both nappes.
Closes `mint-offset-ignores-cone-mirror-nappe`. Spec
`docs/SHELL-6-SPEC.md`. Pre-draw difficulty **S–M**, task class
**STRUCTURAL-NUMERIC** (one decide moved and shared; a sign that
three consumers must agree on) — logged BEFORE block SHELL-B2's byte
is drawn.
