---
id: shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow
kind: issue
title: shell_open's rim-stage lift on the klein elbow takes the per-chart door, whose re-anchor leaves a curved corner 0.88 mm off its carrier
status: open
opened: 2026-10-01
refs: [equator-seam-reauthor-refuses-the-hollowed-elbow, spiric-rim-window-reads-its-inner-equator-end-on-the-branch-cut]
priority: P1
cost: M
---

## What

Measured on `curved/equator-seam` (PR 3626). Once the klein elbow's
equator seams re-author and its spiric rims' windows read forward,
the SEALED hollow reaches tier 3's check 7 (props door). The OPENED
arm (`topo::shell_open` with both meridian caps designated) stops a
stage earlier, at the rim stage's lift:

`ShellError::Lift { face: FaceKey(1v1), error: ReanchorOffCarrier {
edge: EdgeKey(9v1), gap: 0.0008774631373884567 } }`

It is pinned by
`crates/sweep/tests/verbs_shell.rs:the_klein_wall_pair_seals_to_the_props_door_and_opens_to_the_lift`.

## Where, by reading (cause unmeasured)

`crates/topo/src/shell.rs`'s lift stage (the `lift_door =
offset_door(&out, &lift_scope, band)` block) offsets the counterpart
chart back by choosing a door for the lifted solid's scope:
- `offset_door` returns `ChartsTogether` only when
  `offset_axial::is_axial_in` accepts the scope;
- otherwise it returns `PerChart`, and `replace_faces_offset` re-anchors
  each corner per chart (`replace_face.rs`, predicate
  `offset_reanchor_on_carrier`), which is wrong at a curved junction
  by the gap above.

So the lifted solid's scope is not classified axial even though the
operand was. It is unmeasured which face or kind `is_axial_in`
declines there; a candidate is the spiric-bounded faces or the
cavity's moved tube. The first act is to measure that.

## Home

SHELL (`shell.rs` lift stage; `replace_face.rs` is SHELL's ground).
