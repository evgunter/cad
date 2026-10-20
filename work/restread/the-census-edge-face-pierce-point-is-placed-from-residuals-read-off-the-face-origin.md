---
id: the-census-edge-face-pierce-point-is-placed-from-residuals-read-off-the-face-origin
kind: issue
title: The census's edge-face pierce point r0/(r0 − r1) is placed from residuals read off the face's stored origin, uncharged: at a grazing edge its rounding is about |p − o|·u·len/|r0 − r1|
status: closed
opened: 2026-10-18
priority: P2
cost: M
closed: 2026-10-20
---


Found by the sweep of `restread/no-shared-key-crossing` (closed forms
read from an arbitrary origin).

## What

`crates/topo/src/census.rs` `pair_edge_face` reads each end's residual
as `(p − f.origin)·f.normal` and `EfPair::on_signs` places the pierce
point at `e.p0 + (p1 − e.p0)·(r0 / (r0 − r1))`, then reads `contain`
there. Each residual's `f64` rounding is about `|p − f.origin|·u`, and
`f.origin` is the plane's stored origin, which can stand far from the
edge. Where the edge grazes the plane, `|r0 − r1|` is only a few `Kε`
(decided, or `pm_census_ef_pierce_lever` under `hypothetical`), so the
point moves by about `len·|p − f.origin|·u / |r0 − r1|`: `3·3·u / 2Kε`
≈ 5e-5 at ε = 1e-12. A pierce near the face's boundary can then be read
`Out` (a missed pierce) or `In` (a false one). `p1` is also rebuilt as
`p0 + dir·len` rather than read as `e.p1`.

## Owed

Measure it first: a grazing edge through a face whose origin is metres
away, at ε = 1e-12, read at `f64` against `Interval`. If it reproduces,
read the residuals from a point of the face near the edge, and charge
the pierce point's rounding before `contain` reads it.

## Closed

Measured first, on main `f4fb9355`: an edge up a prism's tilted side
face, its ends `1.5e-11` to `1e-10` off the plane, aimed `1e-5` to
`1e-6` either side of the face's top edge, the face's stored origin
moved up to 36 m along its plane, at ε = 1e-12, `f64` against
`Interval` and an exact crossing. 31 of 324 poses read wrong at `f64`,
the pierce point moved up to 1.7e-5 along the edge. Origin 3 m across and 3 m up: a crossing 1.4e-6
inside the top edge read no pierce, and one 6.3e-8 outside it read
one. The new fuzzer on main's census, at effort 20 (seed `0x1234`):
46 wrong readings of 1 198 at 1e-12.

Reading the residuals from a point of the face near the edge does not
help: a point computed on the plane is off it by the same rounding, and
a vertex of the face is off it by up to the band's zero, which the lever
divides alike. So the placement is charged instead, by the prior art's
machinery (`geom_core::rounding_charge`, `both_enclose`,
`enclosure_reading` through the magnitude door). `pierce_span` reads
each residual as an enclosure charged `|pᵢ − o|`. It reads the lever
both as the residuals' difference and off the chord, and the fraction
from both ends (`r₀/ℓ`, `1 + r₁/ℓ`), each pair intersected. The lever
is decided on its enclosure (`pm_census_ef_pierce_lever`, straddle
`…_lever_straddle`), for decided and undecided residuals alike.
`pierce_inside` reads the face at both ends of the fraction's
enclosure: agreeing ends answer, disagreeing ends refuse through the
enclosure `[0, L + ε]` of the pierce's distance from the boundary
(`pm_census_ef_pierce_place`, straddle `pm_census_ef_pierce_straddle`).
`p₁` is read as `e.p1`. Effort 20, same seed: no wrong reading at either
scalar, 406 of 1 198 refused at `f64` at 1e-12.

Pinned by
`census::pierce_fuzz::a_grazing_edge_refuses_near_a_far_origined_faces_boundary_and_reads_away_from_it`
(the two reproduced poses refuse, and 0.3 m in and out read a pierce and
none) and the fuzzer
`census::pierce_fuzz::a_grazing_edge_is_read_pierced_where_it_pierces`
(slow set). The sweep's hits are filed:
`the-census-plane-residual-decisions-read-off-a-stored-origin-are-uncharged`
and `the-depth-gaps-least-point-is-placed-over-axis-dot-n-uncharged`.
