---
id: check-6-planar-arm-skips-ellipse-and-nurbs-loops
kind: issue
title: check 6's planar arm does not examine a planar loop riding an Ellipse, spiric or NURBS carrier
status: open
opened: 2026-09-24
priority: P2
cost: D
refs: [sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts, m6-sense-gate-recorded-residuals]
---


## The residue

Filed by ATREST-4, which widened tier 3's check 6 planar arm
(`crates/topo/src/validate.rs`, the `LoopRoleInverted` arm) from
`Line`-only loops to loops of `Line` and `Circle` carriers. What the
arm still does not examine, by carrier:

- **`Ellipse`.** The shared winding (`crates/topo/src/loop_winding.rs`,
  `Body::planar_loop_winding`) decides an ellipse-bearing loop exactly
  — the bulge `axis · major·minor · (Δ − sin Δ)` is the circle's
  affine image — and the merge's role assigner answers on it
  (`LoopCarriers::Elliptic`). The checker does not: its reach is
  `LoopCarriers::Circular`. Two things stand between: the perimeter
  lever is an arc-length UPPER bound for an ellipse (`|Δ|·major`), so
  a thin elliptic region escalates where a circular one decides; and
  no ellipse-bounded body has been measured under this refusal.
  Producers today: `topo::splitting::split` by a plane oblique to a
  cylinder (the cut face of `crates/sweep/tests/common/mod.rs`'s
  `tilted_cut_upper`; `step-export`'s `cut_cylinder`), and any STEP
  import stating an ellipse edge on a plane.
- **NURBS and spiric.** No closed-form area; the shared winding
  answers `None`. The honest remainder.

A planar face bounded so carries a stored `sense` bit no at-rest check
falsifies. Pinned by
`an_ellipse_bounded_planar_face_stays_outside_the_planar_arm`
(`crates/sweep/tests/m5_s10_face_sense.rs`): the tilted cut's
ellipse-bounded face, inverted through the public
`Body::set_face_sense`, is `Ok(())` at rest while the same body's
circle-bounded cap refuses.

## The flip

Widen the reach to `LoopCarriers::Elliptic` — one argument at the arm
— with the measure-first step ATREST-4 ran for circles: the refusal
surface over every body the corpora build, and in particular what
newly ESCALATES on the upper-bound lever (escalation is exempt, so it
would not refuse, but it is the in-band population the posture has
to be read against). NURBS rides a closed form that does not exist.
