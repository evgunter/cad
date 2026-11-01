---
id: ssi-window-check-reads-the-walls-control-hull
kind: issue
title: ssi: plane x NURBS refuses WindowShortOfWall from the wall's control hull, which can reach four times past the wall, and says the wall reaches that far
status: open
opened: 2026-10-04
priority: P3
cost: M
---


(SSI investigation lane `ssi/dome-remeasure`, 2026-10-04, from
re-measuring `plane-nurbs-ssi-does-not-certify-a-curved-dome`.)

## What

`plane_nurbs_ssi` and the uncertified door first ask whether the
plane's chart window holds the wall (`boundary::window_holds_wall`,
`crates/geom-brep/src/ssi/boundary.rs:1001`, called from
`crates/geom-brep/src/ssi.rs:2432` and `:2645`). It reads the wall's
**control points**, whose hull contains the wall, so a window that
holds the hull holds the wall. That is sound, but the hull can reach
far past the surface, and the check refuses wherever the hull does
not fit, even when the wall does.

On the curved dome `W(d)` of that row (control point `(½, −d, ½)`,
surface `y = −4d·s(1−s)·t(1−t)`, so the surface reaches only
`y = −d/4`), with a window of half-extent 2 m centred at
`(0.5, −d/8, ·)`:

| cut | d | hull reach (refused) | the wall's own reach |
|---|---|---|---|
| zcut `z = 0.2` | 3 | 2.625 m | ≤ 0.5 m |
| oblique `x + z = 1` | 3 | 2.625 m | ≤ 0.75 m |
| zcut, oblique | 4 | 3.5 m | ≤ 0.75 m |
| tilt `x + y = 0` | 4 | 2.47 m | ≤ 1.0 m |

With the window widened to 4 m every one of them is answered (the
tilt at d = 4 by a different refusal, filed beside this one).

The refusal's prose also states the wrong thing: `SsiError`'s Display
(`ssi.rs:961`) says *"the wall reaches {reach} m across it"*, and
`reach` is the control hull's, not the wall's.

## Fix shape

- **The prose (E):** name the control hull, not the wall.
- **The bound (M):** tighten the reach before refusing: subdivide the
  wall's Bézier patches (knot insertion converges the hull to the
  surface) until the hull fits or a piece that does not fit is
  certified to reach past the window. Weigh against simply keeping the
  conservative check and saying so; the recourse (widen the window)
  works either way.
