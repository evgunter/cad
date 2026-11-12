---
id: two-cones-apex-lobes-on-two-carriers-refuse-a-valid-body
kind: issue
title: Two cones' apex lobes on two carriers in one self-touch link refuse a valid body
status: dispatched
opened: 2026-11-12
priority: P1
cost: H
---


Found by the remainder-refuses lane (PR 6854), which makes every
self-touch point the reading cannot decide refuse.

## What

`rest::at_apex` (`crates/topo/src/link.rs:4746`) reads a link holding a
cone's apex only where its lobes make one nappe whole (`whole_nappe`,
`crates/topo/src/link.rs:4839`). The nappe must be one spur corner, or
two corners of one carrier. Lobes of two cones' apexes in one link (the
"second apex" class) are no nappe, and refuse `Curved(NoTangentPlane)`.
Before PR 6854 the sweep skipped them; now they refuse
(`sweep_self_touch_points`, `crates/topo/src/census.rs:8888`).

A valid body reaches this. In
`crates/sweep/tests/a_cone_apex_meets_a_vertex.rs`,
`two_cones_through_one_apex_build_across_the_pose_families`, both forms
(tipped and revolved), at default ε and at 1e-12:

- 20/50 near-coaxial, `b ∖ a`;
- 45/30 near-coaxial, `a ∖ b`.

Each refuses `BoundaryCrossesItselfUndecided` at the shared apex
`(0, 1, 0)`, `why: Curved(NoTangentPlane)`. The row now pins that
refusal, citing this file.

## The fix

Read two nappes at one apex: two convex cones `C₁`, `C₂` about one point.
Each is the face itself at every scale its corner reaches, so the link
is decided by how the two cones and the remaining sectors sit, not by a
tangent plane. One possible approach is to hold both nappes off every
plane and off each other, and to read the windings as `at_apex` reads
one. When it lands, the pinned row goes back to asserting pass.

## Unit: the valid bodies the refusal reaches

Spec (RESTREAD orchestrator, 2026-11-12). One unit reads both valid
bodies PR 6854 pinned as refusals: this row's two apex lobes on two
carriers, and `a-ridge-end-on-an-arc-wall-tangent-to-its-floor-refuses-unreadable`'s
ridge ends. Branch `restread/self-touch-valid-bodies`.

- **Letter H, tier dual** (a GRANT in the self-touch reading: a point
  read `Rest` where the germ crosses is the worst outcome), so the
  model comparison's concurrent pair.
- Where a reading does not decide, the point keeps refusing with its
  true cause.


## Read (branch `restread/self-touch-valid-bodies`)

**What the bodies hold.** At the shared apex `(0, 1, 0)` the link is two
whole nappes and nothing else: the wider cone's, its material inside (a
thorn), and the narrower cone's, its material round it (a needle of
void), the needle strictly inside the thorn, their axes 0.02 rad apart.
Tipped, each nappe is one spur corner; revolved, two half-turn corners
of one carrier. The arm that refused was `whole_nappe`'s match on the
lobes, which took one nappe only.

**The reading** (`rest::whole_nappes`, `rest::rulings_against`,
`rest::at_apex`; the argument is in `crates/topo/README.md` beside the
one-nappe clause). The lobes are grouped into whole nappes, each about
its own axis, all read at one arm. Each nappe is read against every
sector's plane as one nappe was. Each two nappes are read by one's near
and far rulings against the other's cone, each turned about that cone's
axis into the meridian plane of its ruling and read against the tangent
plane there (`link_plane`, the ties' slack charged): apart, nested, or
across each other (a crossing); anything else refuses `InBand`. A nappe
inside no other reads its axis against the rest as before; a nappe
inside another must be the other kind from the least nappe round it, and
is a crossing otherwise.

**Still refused**, filed as `apex-links-the-nappe-reading-still-refuses`:
a nappe of three or more lobes, a horn beside a nappe, a half-plane
cutting one, a needle inside no nappe with no sector round it, and two
nappes tangent along a ruling.

The row `two_cones_through_one_apex_build_across_the_pose_families`
asserts pass again on all four ops of every pose. Counts are in
`a-ridge-end-on-an-arc-wall-tangent-to-its-floor-refuses-unreadable`.
