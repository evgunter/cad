---
id: point-in-solid-ray-denominators-are-not-lengths
kind: issue
title: The point-in-solid ray test decides dimensionless and 1/m denominators against the metre band, so at eps 1e-6 a rod a thousand kilometres across refuses to build
status: open
opened: 2026-10-01
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
