---
id: the-section-certificate-reads-a-plane-and-a-torus-on-its-own-rows
kind: issue
title: Three readers of a plane against a torus read its closed-form pose at three levers, so one pose can read closed to one reader and tilted to another
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


Filed by `orbit/tilted-torus-rim` (PR 7617), which serves a plane tilted
against a torus's axis through the section value
(`geom_brep::PlaneTorusPose`, `crates/geom-brep/src/pair_section/plane_torus.rs`).

## Folded in PR 7617

The tube rows, the null loop's witness and the elliptic touch had two
homes: the pose's, and the section certificate's own `plane_cut` and
`torus_plane` (`topo::boolean::section_cert`, rows
`section_torus_plane_{near_tube,far_tube,between}`,
`section_torus_plane_touch_elliptic`). Both now read one
`geom_brep::PlaneTorusMeridian` (`pt_near_tube`, `pt_far_tube`,
`pt_between_tubes`, `pt_touch_elliptic`).

## What is left: the closed-form question at three levers

Whether a pose is the closed form's (a plane holding the axis, square to
it, or parallel to it short of the inner equator) is
`geom_brep::plane_torus_section`'s axis rows, levered, and three readers
ask it at three levers:

- the section value (`PlaneTorusPose::decide`) over the torus's ball
  (`torus::lever(center, ball)`);
- the join's germ frame (`topo::boolean::join::pt_germ_frame`) at the
  torus face's reach (`FrameExtent::Reach`);
- the C5 table's pose question (`geom_brep::route_pose`) at the
  consumer's reach from the torus's centre (`extent()`), and then, where
  that reads the general rung, the section value at the ball's lever.

The rows are lane choices (`closed_form_shown`), so a tilt in the band
of one lever and past it at another reads closed to one reader and
tilted to the other. Where the value reads closed and the frame tilted,
the operation's section table holds nothing for the pair and the germ
refuses `BooleanError::GermSectionSpiric`; where `route_pose` reads
tilted and the value closed, it answers `implemented` off the value's
`Elsewhere`.

Settles when the three readers read the closed-form question at one
lever (the value's, held by the section table and handed to the frame
and the gate), or a row straddles each pair of levers and pins the
refusal.
