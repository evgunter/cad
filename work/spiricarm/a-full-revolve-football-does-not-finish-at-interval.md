---
id: a-full-revolve-football-does-not-finish-at-interval
kind: issue
title: A fully revolved football does not finish at Interval: the apex turn between its π-bands' meridians, half a turn, is undecided (props_sheet_apex_turn)
status: open
opened: 2026-11-25
priority: P1
cost: M
---


## Finding

Found by lane `embedwalk-spindle` (2026-11-25) while writing
injectivity rows for a spindle sheet at both scalars.

The lemon of `R = −0.5, r = 1` about the sketch `y` axis (the arc from
`(0, −h)` to `(0, h)` through the circle's widest point, closed down the
axis, `Revolution::Full`) builds at `Interval`, but
`sweep::test_support::taken::<Interval>` refuses it:

```
NotFinished { findings: [VolumeUncomputable { solid: SolidKey(1v1),
  source: Face { face: FaceKey(1v1), source: Escalated { cause:
  Indeterminate { margin: MarginDiag(Enclosure { lo: -1.570796326794903,
  hi: 1.570796326794903, .. }), .. predicate: Some("props_sheet_apex_turn") },
  check: Inventory } } }] }
```

At `f64` it finishes (`sweep::all the_football_revolves::the_football_and_the_apple_revolve`
reads it at `f64` only). `geom_brep::props::curved`'s
`sheet_apex_jumps` decides the turn between the arriving and departing
meridians' half-planes at an apex (`turn_about`, metered by the lesser
of their radial levers) and refuses a turn in the band of zero. Untraced past the
payload: the margin's enclosure is `[−π/2, π/2]`, straddling zero. A
full revolve's two π-bands meet at each apex on meridians half a turn
apart, which is where an angle's sign is least determined, so that is
the first place to look; the enclosure's width says the margin is not
the bare turn.

Whether every Interval full revolve of a sheet from apex to apex meets
it is not measured; this lemon does.
