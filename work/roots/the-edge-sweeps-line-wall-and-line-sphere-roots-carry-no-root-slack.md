---
id: the-edge-sweeps-line-wall-and-line-sphere-roots-carry-no-root-slack
kind: issue
title: The edge sweep's line × wall and line × sphere roots carry no rounding slack, where its line × cone roots do
status: open
opened: 2026-10-31
---


Found by FLUSH's `flush/aimed-ray-reach`. That branch gives
`line_wall_roots` and `line_sphere_roots` (`boolean/solid_contain.rs`)
a first-order bound on each root's rounding, `WallRoots::Two(roots, slack)`.
It also lets them read the quadratic about the line's closest approach
(`About::Nearest`, the rays' form). The edge sweep reads it about the edge
line's own origin (`About::Start`), as before, bit for bit. Moving it
would widen an `Interval` enclosure by the shift's own: the measured
witness is `verbs_germarms2_interval`'s skew pair at 1e-12, whose
re-posed twin then escalated on `pcurve_map_residual` instead of
`carrier_matches_mapped_source`.

The edge sweep's door (`boolean/reduce.rs` `line_wall_roots_of`, the
two `WallRoots::Two(ts, _) => two(ts)` arms) drops the slack: a line
edge's wall and sphere roots go on as exact. Its cone arm does meter its
roots (`line_cone_roots`, `bool_line_cone_root_slack`).

**The start form's discriminant carries its start's rounding too**
(PR 6174's review). Read `About::Start` from far starts, 7 line × sphere
cases decided Two roots where the exact discriminant is `≤ 0`. The disc
rung's margin carries the `u·D²` of a residual read `D` from the sphere,
and the band alone meters it. The edge sweep's lines start at the
body's scale, where `u·D²` is the body's own rounding, so it never meets
such a start; the rays, which do, read `About::Nearest` and charge the
shift's rounding into the margin (`Centred::charged`).

Owed: carry the wall and sphere slack through the edge door, as the cone
arm carries its own. Then state the start form's disc margin's rounding,
or charge it as `Centred::charged` does for the nearest form.
