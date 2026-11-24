---
id: the-section-certificate-reads-a-plane-and-a-torus-on-its-own-rows
kind: issue
title: Three readers of a plane against a torus read its closed-form pose at three levers, so one pose can read closed to one reader and tilted to another
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
closed: 2026-11-24
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

## Closed (2026-11-24, PR 7713)

Settled the first way: one lever. The question was asked at five
levers on main, not three: the row's three readers, plus the two chord
lanes at each face's reach. `plane_torus_closed_form` is now the only
spelling of the closed-form question, at the section value's lever (the
torus's ball). `PlaneTorusPose` holds its answer, the section table
holds closed plane×torus poses (`geom_brep::pair_section_held`, one
decide site), and the frame and the chord lanes read that answer.
`GermSectionSpiric`, `FrameExtent::Reach`, `SectionReach::Torus`,
`PT_TOUCHING_UNDER_A_GERM` and `invalid_margin` retire with the
disagreement they served. On main, torus(R2,r1) ∩ a brick holding the
axis at tilts 3e-11 to 1.5e-10 refused `RoutesToGeneralRung`; it now
builds at half the torus's volume, and a sweep row pins it (red under a
lever mutant at 30×). Single full review: APPROVE-WITH-FIXES, 0 MAJOR;
its lever probe over reach ×1.2–×300 and ε 1e-6 to 1e-12 found no wrong
body. Left open: `section_cert::torus_plane` still reads the square
half Structural at the ball lever, which errs toward escalation (ORBIT
log, 2026-11-24).
