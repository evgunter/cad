---
id: a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected
kind: issue
title: A solid touching itself at a corner on a spline crease finishes with the touch undetected: corner_normal reads every crease Unreadable, which the at-rest census routes to SelfTouch::SecondOrder and skips
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-10
branch: onecopy/crease-self-touch
---


Found by the full review of PR 6652 (`onecopy/boolean-standing-2`), F1.

## What

`census::corner_normal` reads a face's corner normal through
`sector_face::resolve`. Where the corner sits on a spline crease
(`SectorFaceError::Crease`), it reads `TouchUnread::Unreadable` whatever
the crease's reading (`reading`: in band, or decided zero).
`self_touch_verdict` dispatches on that variant: `Unreadable` goes to
`Link::tangent_of`, then `SelfTouch::SecondOrder`, which the at-rest census
skips as undetected. The census module's own list says so ("a spline face
held off a corner", `census.rs` module docs; `SecondOrder`'s and
`tangent_of`'s docs agree).

The crease reading has a standing of its own
(`geom_brep::ladder::Refusal::of_crease`): ranked by its margin in band,
and a limit when decided zero. PR 6652 first routed the corner by that
standing (in band → `TouchUnread::InBand`, decided → `Unanalysed`). That
made tier 3 refuse such a body with
`BoundaryCrossesItselfUndecided`: an at-rest gate change landed as a side
effect of a standing refactor. The review stopped it, and the corner is
back on the `Unreadable` route (comment at `corner_normal`).

## The decision

Whether the at-rest gate refuses a solid touching itself at a spline
crease corner, or finishes it with the touch undetected:

- **Refuse loudly.** Route the corner by `Refusal::of_crease`, so in band
  is `InBand` and decided is `Unanalysed`. The census and tier 3 refuse
  the body, and the census's undetected list, `SecondOrder`'s doc and
  `tangent_of`'s doc lose the case.
- **Keep it undetected.** Leave the route, and state at `corner_normal`
  why a crease corner is read like a curved face held off it.

Either way, pin it with a row: a body touching itself at a crease corner,
and the gate's outcome on it.

## Resolution

Neither option as written: the census held the sector's side through
its half-edge, and the chart reader dropped it. A spline corner on a
crease now reads the normal on its sector's side
(`geom_brep::chart_normal_beside`, the side decided off the half-edge's
pcurve box), and what still refuses (a side the band does not read, an
irregular point) routes by `Refusal::of_crease` in `census::corner_normal`.
Pinned by `census::self_touch_curved::a_solid_touching_itself_at_a_spline_crease_corner_is_read`.
The `Unsupported`/`Unplaced` arms are their own row:
`work/onecopy/a-spline-corner-with-no-placed-chart-reading-finishes-undetected.md`.
