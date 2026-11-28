---
id: the-blend-arms-mint-conics-before-a-predicate-reads-an-oversized-radius
kind: issue
title: The blend arms mint their circles before any predicate reads the radius: a finite radius past 1.5e308 panics a fillet, and the cylinder-sphere centre takes a root the room does not decide
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Filed by GERM's `germ/conic-constructions-audit` lane (2026-11-23), from
`work/germ/conic-constructions-from-computed-sizes-unaudited.md`.

## Measured

`geom::Circle::new` panics on a radius that is negative or not finite.
`sweep::blend::battery::run_battery_for` resolves each link's arm
(`resolve_link` → `classify_arm`) before predicates 1 to 3 (headroom,
clearance, spine regularity) read the radius, and the arms
(`sweep::blend::arms::plane_sphere_blend`, `Meridian::blend`,
`Ruling::blend`, `chamfer_frustum`) mint their trim circles as they go.
They are documented "total arithmetic, public, and callable bare"
(`battery.rs`, `BallRoom`), which held while a conic could carry poison
to the predicates.

**A finite radius panics a fillet.**
`fillet_edges(&finished(lantern), &rim_arcs_at(&body, 0.2, 1.2), r, tol)`
panics in `Circle::new: radius NaN` at `r = f64::MAX` and `1.5e308`
(at `1e200` it refuses `RadiusHeadroom`). The lip is a cone × cap rim:
two straight traces, for which `room_at` decides nothing ("they cross
at every radius"), and `sheet_center`'s `rim − V·r/(1 − d²)` overflows.
The stack ends `curved_arm → BallRoom::meridian → Meridian::blend →
Curve3::circle`. The infinite literal is closed at
`nonpositive_size_gate` by the audit's PR; the finite ones are not.

**The cylinder × sphere centre takes a root the room does not decide**
(a float model of `arms.rs`, not run in the kernel). At a rim,
`sheet_center` takes `√(1 − D/b²)`, and the room `fillet3_ball_room`
decides is `offset − |(centre − line)·normal|`: the same quantity in
real numbers, a different expression in floats. With a sphere `R = 10`
capping a coaxial cylinder `a = 9.99999987393` at
`z₀ = 0.0015878916814147106` and `r = 9.999999880233501`, the room is
`+1.13e-7`, `Positive` at ε = 1e-9, and the root's argument is
`−5.6e-14`, a NaN centre.

**Smaller.** `chamfer_frustum`'s cylinder-side foot radius,
`R_rim + distance·(inward·radial)`, is decided by nothing (its plane
foot is `fillet3_face_clearance`'s own expression). The bare arms panic
on inputs their docs call total: `plane_sphere_blend` with
`offset < |h|`.

## What a fix has to supply

Where the size of a circle an arm mints is decided: the arms returning
a refusal, a predicate that bounds the radius before the arm runs, or
the root re-posed on the room's own factors. The first two are the
design question `work/germ/no-sign-decision-gives-a-conic-a-finite-size.md`
states.
