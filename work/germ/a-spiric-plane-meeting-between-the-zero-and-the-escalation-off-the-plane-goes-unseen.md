---
id: a-spiric-plane-meeting-between-the-zero-and-the-escalation-off-the-plane-goes-unseen
kind: issue
title: A spiric × plane meeting whose oval point sits between the band's zero and its escalation off the cutting plane is answered without a refusal: the line × torus ladder's root accuracy near its own graze
status: open
opened: 2026-11-23
priority: P2
---


Filed by GERM's `germ/co-surface-table` lane (PR 7431), from that PR's
confirmation review at `7f72931e9d`. The case was present at the old
head (`f5d7ed9762`) too.

## Finding

At `CAD_TOLERANCE_EPS=1e-6`, planes nearly square to the torus axis
that cut thin tubes gave 6 meetings in 3 cases out of 3251. Each meeting
sits about 1.2e-6 off the cutting plane: past the band's zero but
inside its escalation. The spiric × plane door
(`crates/geom-brep/src/curve_face/spiric_plane.rs`, `on_the_oval`)
answered them without a refusal.

The roots come from the line × torus ladder
(`geom_brep::curve_face::line_quadric::line_torus_roots`,
`crates/geom-brep/src/curve_face/line_quadric.rs:666`). So this is the
ladder's root accuracy near its own graze: a near-double root's position
is read to about the square root of its residual.

## Settles when

Either the door certifies each meeting's height off Q within the zero,
or it escalates when that height reads between the zero and the
escalation. A row on thin tubes cut nearly square to the axis at 1e-6
must read 0 such meetings unrefused.
