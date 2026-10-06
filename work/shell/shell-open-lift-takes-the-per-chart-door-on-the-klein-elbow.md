---
id: shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow
kind: issue
title: shell_open's rim-stage lift on the klein elbow takes the per-chart door, whose re-anchor leaves a curved corner 0.88 mm off its carrier
status: open
opened: 2026-10-01
refs: [equator-seam-reauthor-refuses-the-hollowed-elbow, spiric-rim-window-reads-its-inner-equator-end-on-the-branch-cut, spiric-bounded-face-area-is-unimplemented]
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

## Where (measured)

The lift's door reads the lifted solid on the RESULT body
(`shell.rs`, the `lift_door = offset_door(&out, &lift_scope, band)`
block), and that solid holds the cavity as well as the operand. Every
face of the lift scope, classified by `offset_axial::is_axial_in`
(probe at commit `2c7aded3`, reverted in the next commit):

| face | surface | verdict |
|---|---|---|
| 1v1 | plane `x = 1.2`, normal `+x` (operand meridian cap) | axial |
| 2v1 | plane `z = 0`, normal `−z` (operand meridian cap) | axial |
| 3v1, 4v1 | torus R 1.2, r 0.275, coaxial | axial |
| **5v1** | plane `x = 1.15`, normal `−x` (cavity cap) | **`TogetherNotAxial`: "a plane parallel to the axis but not through it"** |
| **6v1** | plane `z = 0.05`, normal `+z` (cavity cap) | **same** |
| 7v1, 8v1 | torus R 1.2, r 0.225, coaxial | axial |

(axis: point `(1.2, 0, 0)`, direction `−y`.)

So the scope is **genuinely not axial under the gate's definition**:
the cavity's meridian caps were TRANSLATED one wall inward, which is
what the cavity door does to a meridian cap, so they stand parallel to
the axis but `0.05` off it. The gate is not mis-reading the body. The
cavity door met no such plane because it classified the operand's
caps (through the axis) as `structural` and read only the numbers off
the moved ones (`classify`'s structural/moved split). The lift has no
operand surface to read: it classifies the result's surface as both
(`is_axial_in` passes `surface, surface`), and so does
`offset_charts_together`'s own `classify` call.

**Admitting the plane is not enough.** As an experiment (not
committed), with `classify`'s plane arm taking an off-axis
axis-parallel plane as `Constraint::Meridian { m, c }`, the lift takes
`ChartsTogether` and refuses one stage later:
`TogetherAxialEdge { edge: EdgeKey(7v1), what: "an edge between a
torus wall and a meridian cap whose carrier is not a circle" }`. The
together door's torus × meridian-cap edge arm mints a spiric FROM a
meridian circle (the cavity's way in). The lift is the way back, from
a spiric old rim to a circle, and that arm does not exist.

## The design question

Taking the together door on this lift needs two new arms in
`offset_axial.rs` (ground shared with OFFSET and CURVED):

1. the gate (`classify`'s plane arm, read by both `is_axial_in` and
   `offset_charts_together`) admits a plane parallel to the axis that
   does not pass through it, as a `Meridian` constraint with `c ≠ 0`.
   Either everywhere (a change to the roster the door takes, which
   also changes the door other bodies get: the box-beside-a-vessel and
   cylinder-void-in-a-box rows decline on exactly this verdict today),
   or only on the lift, by letting the lift classify against the
   designated face's own plane as `structural`;
2. the torus × meridian-cap edge arm takes a spiric old rim, not
   only a circle.

The alternative, refusing earlier and typed rather than at the
re-anchor, gains nothing over today's `ReanchorOffCarrier`, which is
already typed and names the edge. Which way to go is the orchestrator's
call (a designer pair, per the brief).

The end-to-end payoff still also waits on FLUX's
`spiric-bounded-face-area-is-unimplemented`: the sealed arm stops at
`PropsError::Unimplemented`.

## Home

SHELL (`shell.rs` lift stage; `replace_face.rs` is SHELL's ground).
