---
id: point-in-solid-ray-denominators-are-not-lengths
kind: issue
title: The point-in-solid ray test decides dimensionless and 1/m denominators against the metre band, so at eps 1e-6 a rod a thousand kilometres across refuses to build
status: closed
opened: 2026-10-01
closed: 2026-10-01
branch: reach/opensign-red
refs: [an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed]
---


## What

`crates/topo/src/boolean/solid_contain.rs` decides its ray-cast
denominators under `bool_point_in_solid_denom` through
`decide_flagged(.., band, "F2")`. These are the plane arm's `d·n` (a
cosine), `line_wall_roots`' `a2 / two_r` (`sin²/2r`, in 1/m) and the
cylinder hit-outward `d·rad / radius` (a cosine). The band is a length
(ε), so these comparisons are not dimensionally sound.
`docs/predicate-dimension-audit.md` row **F2** records this as "deferred
to a coordinated unit". No slate row carried it until now.

Measured (2026-10-01, REACH lane `reach-epsfix`): at
`CAD_TOLERANCE_EPS=1e-6`, `subtract(rod, cutter)` refuses on a rod of
radius 0.5·s and height 4·s, cut by a box tilted 20° at z = 3.5·s
(`crates/sweep/tests/reach_volume_backstop.rs`, `rod` / `cutter`), for
s ≈ 1e6:

    Containment(Escalated { face: FaceKey(3v1), diag: Indeterminate {
      margin: 1.0000000000000002e-6, band: (1e-6, 1e-5),
      predicate: "bool_point_in_solid_denom" } })

At s exactly 1e6 and 1e7 the same rod builds. At s = 999999.9999999999
and 9999999.999999998 it refuses. The margin sits one ulp above the
band's floor, so whether the rod builds depends on the last bits of the
scale. At 1e-9 the same body in units of ε (s = 1e3, 1e4) builds.
`an_open_sign_beyond_the_band_at_the_last_round_refuses`, as PR 3636
restates it, builds these rods at s = 1e12·ε and 1e13·ε. At ε = 1e-6
those products are the scales that build, so the row is green by the
last bits of its scale. A neighbouring spelling of the same scale, such
as `1e3 · (ε / DEFAULT_EPS)`, refuses as above.

## The shape of a fix

The audit names the model: the sphere-disc arm meters its comparand
with `over_lever`. Meter all four F2 sites the same way, in one unit, as
the audit asks. The acceptance margins that quote these predicates move
with it.

## Closed (2026-10-01)

All four sites are metered in metres. The ledger rows read FIXED and
`decide_flagged`'s census drops from 8 to 4. Beyond refusing, the
axis-parallel rung answered wrong. On a wall of radius `≥ 1/(2ε)`, any
ray read as parallel and skipped the wall, so a point on a wide rod's
axis read `Out`. That wrong verdict is what PR 3716's loop-role
agreement check turned into `SectionLoopMixed` on `reach_volume_backstop`
(`work/reach/an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed.md`).
The fix and its rows:

- The two skip questions are levered by the selection's reach
  (`solid_contain::selection_reach`, a decision-free ball over every
  loop, `containment::loop_extent_from`). The plane arm's lever is the
  reach. The wall's is the reach plus `2r`. For an edge, the lever is
  the edge's own span.
- The disc is `over_lever(disc/|d⊥|², 2r)`.
- The hit-outward sign is read off the root order.
- Rows: `solid_contain::wall_root_rows`, and `rim_dim_boolean_twins`,
  which allows no nonlinear predicate now and pins
  `bool_point_in_solid_denom` as firing.
- The edge-sweep refusal of the axis-parallel rung offers a tolerance,
  as a sized ending, with an executed case in `offer_rows`. A
  penetrating edge's clearance (`Coincidence(EdgeOnCurvedFace)` /
  `(VertexOnCurvedFace)`) offers one on both sides, because the wall
  roots it goes on to are a length.
