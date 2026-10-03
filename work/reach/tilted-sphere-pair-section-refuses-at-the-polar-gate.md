---
id: tilted-sphere-pair-section-refuses-at-the-polar-gate
kind: issue
title: A sphere pair whose centre line is off either chart's polar axis refuses at the arc-side rule's polar gate
status: closed
closed: 2026-10-02
opened: 2026-10-01
refs: [sphere-union-sphere-refuses-though-the-section-is-closed-form]
pr: 3817
branch: reach/tilted-sphere-pair
---

Found by the `reach-snowman` lane. Once the crossing layer had circle ×
sphere roots and the boolean's join had a sphere-pair arm (the chord
rides the pair's RADICAL PLANE on both sides, `boolean::join`'s
`(Sf::Sphere, Sf::Sphere)` arm), the coaxial snowman builds — in its
coplanar-seam pose only; spun about the axis it stops at the pierce-ring
door (`work/tang/pierce-ring-has-no-join-arm.md`). A sphere
pair whose centre line is not along BOTH operands' chart polar axes
gets through the crossing layer and stops at the join.

## Measured

`demos/tour/src/lily.rs` wall 7 (a ball of radius 0.16 at
`(-2.80, 0, 0.90)` subtracted from the repaired lantern, whose sphere
zone is charted about the lantern's own axis):

```
Join(SectionInvariant { face: FaceKey(4v1),
  what: "plane×sphere section tilted against the sphere chart's polar axis — ..." })
```

raised by `chord_join::section_case`'s plane×sphere arm
(`split_sphere_section_polar` deciding `Positive`): the radical plane's
normal is the centre line, and the azimuth-anchored arc-side rule
(`select_arc`) premises azimuth MONOTONE along the section carrier, which
holds on a sphere chart only for a polar section.

## What a fix has to supply

A chart in which the section is polar. Two shapes are visible:

- **Re-chart a free ball** about its own centre so its polar axis lies
  along the centre line — the no-crossings sphere re-cut
  (`boolean::ops::apply_recuts`) already rotates a whole-sphere shell
  this way, but only on the path where the crossing layer saw no
  event. A ball × ball pair on any two centres would build that way.
- **An arc-side rule that does not read azimuth**: the lantern's sphere
  zone is not a free ball (its chart is fixed by its rims), so wall 7
  needs the section arc selected some other way — e.g. by the chord's
  endpoints and a midpoint containment on the face, as the circle is
  exact.

The second is the general one; the first covers the free-ball family
alone. Which to build is a design choice for whoever specs this.

## Evidence (2026-10-01, PR 3659 review)

- The refusal is now its own variant,
  `SplitJoinError::SectionNotPolar { face, band }`, shared with the
  plane×sphere arm.
- A Z offset of 0.2 at `y = 1.4` — ball(0.8) centred at `(0, 1.4, 0.2)`
  against ball(1.0) at the origin, both charted about `y` — does not
  reach the polar gate: it refuses `CurvedSectorSideUnsupported`, the
  sector-side sagitta charge of
  `work/reach/slab-cut-cylinder-refuses-sector-side.md`, one door
  earlier. Re-charting a free ball along the centre line would move
  this pose too.

## Closed (2026-10-02, PR 3817)

A sphere pair whose centre line is tilted against both charts now
builds, and so does the lily's wall 7 carve.

- **Arc selection.** The arc-side rule picks a section arc by the run's
  side at each run end (`chord_join::select_arc_by_run_side`), gated on
  a smooth or convex corner there (`ReflexRunEnd` otherwise).
- **Measurement.** A sphere face bounded by tilted circles measures by
  Gauss–Bonnet over its circle arcs (`props::curved::sphere_circle_loop`).
  The side is read off the loop, and a contradicting sense bit refuses.

The dual review (DR-45) found no wrong body in about 300 oracle bodies.
The delta review checked 480 bodies and 3,600 hand-built loops and
found 0 wrong.

What remains is filed: the pole-bearing sense flips (P2); the
near-pole `ArcNearPole` refusal; the clearance pre-filter; the PCERT
envelope target.

## Evidence (2026-10-02, the ellipse-rim lane): a plane face through a ball

The plane × sphere join reaches the same refusal when the plane is an
OPERAND face rather than a radical plane. The lower part of the tilted
drum cut (`crates/sweep/tests/conic_edge_curved_face.rs`: radius 0.5,
height 1, cut by the plane through `(0, 0, 0.5)` at 0.3 rad) against a
ball of radius 0.3 at `(0, 0, 0.5)` charted about `y`: once the rim's
ellipse clears the sphere, ∪, ∩ and ∖ refuse
`Join(SectionNotPolar { face: FaceKey(6v1) })` — the cut face's
section of the ball is not a latitude circle of the ball's chart. The
same ball charted about the cut's normal passes the join (and stops at
`work/contact/at-infinity-probe-measures-in-closed-form-only.md`). So
the free-ball re-chart this row names would cover this pose too: the
ball is free, and its one escape plane fixes the alignment. Pinned by
`a_ball_through_the_cut_face_clears_the_rim_and_stops_downstream`.
