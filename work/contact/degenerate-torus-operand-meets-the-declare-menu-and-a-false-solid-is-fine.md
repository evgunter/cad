---
id: degenerate-torus-operand-meets-the-declare-menu-and-a-false-solid-is-fine
kind: issue
title: contact: a torus operand outside the ring convention reaches the Boolean's front door as CurvedPierceUnsupported's declare menu or Containment's 'the solid itself is fine'
status: open
opened: 2026-09-30
---


(TOPO, the fix pass of PR 3506: the review's MINOR-3.)

## What

PR 3506 made the pierce normal (`topo::face_normal::face_outward_normal_at`)
refuse a torus outside the ring convention as `BooleanError::DegenerateTorus`,
ending in the convention's lever (`geom_brep::TorusConvention`). No
public door reaches that arm: a pierce point reaches the normal only
after the crossing layer has placed it in the torus face's trim, and
that placement reads the convention first. Through `topo::union` /
`topo::subtract` a torus operand outside the convention stops at one
of two other stories, identically on main and on PR 3506 (the review's
probe `zz_torusr_probe.rs`: the sweep crate's revolved donut, its torus
faces re-described with `Body::set_face_surface`, against a bar through
the hole and a bar across the tube):

- **Spindle, horn, horn in the zero band, ring in band**:
  `BooleanError::CurvedPierceUnsupported`, "an edge of the second
  operand touches or crosses a curved face of the other operand away
  from that face's edges, and the Boolean cannot yet settle where or
  whether it passes through. Recourse: declare the coincidence, move the
  geometry, or lower the tolerance". The route:
  `boolean::contain::torus_face_containment` →
  `boolean::solid_contain::point_on_torus_in_face`, whose
  `geom::require_ring_torus` gate refuses (or `torus_face_windows`
  does), and `boolean::reduce::wall_crossing` keeps the frontier door
  (`SpanVerdict::Unsettled`).
- **Nonpositive tube** (`r = −0.3` or `r = −0.5`): `BooleanError::Containment`
  (`PointInSolidError::PartialTorusFace`), "the solids do not cross, and
  the Boolean cannot tell what is inside the solid: one of its torus
  faces has an outline the inside/outside test cannot read. The solid
  itself is fine. Recourse: bound the torus face with circles …". The
  solid is not fine: its torus has no tube.

Both are D4 ¶1 (iv) forks of the convention's story, and the first
offers a declaration no door takes for it.

## Repair shape

Carry the convention's verdict out of the containment door
(`point_on_torus_in_face`'s gate and `torus_face_windows`: which half
refused and its `Decided`, through `geom::torus_tube` / `geom::ring_torus`
rather than `require_ring_torus`), and end it where it surfaces as
`BooleanError::DegenerateTorus` (definite) or
`BooleanDecision::Torus` (in band), the pierce's own two arms. The
`contain-escalation-carries-no-decision.md` row names the same gate
from the escalation side.
