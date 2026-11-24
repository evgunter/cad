---
id: a-spiric-plane-near-miss-shift-is-measured-in-the-ovals-plane-not-off-the-cutting-plane
kind: issue
title: The spiric × plane door's near-miss reread shifts L by the band's escalation inside the oval's plane, which is a height off the cutting plane only times the tilt sine s: a sub-ε miss decides once s is below about 0.1
status: closed
closed: 2026-11-23
opened: 2026-11-23
priority: P1
cost: M
---


Filed by GERM's `germ/co-surface-table` lane (PR 7431). This is the
residual of its review's MINOR 2, from the confirmation review at
`7f72931e9d`.

## Finding

`geom_brep::curve_face::spiric_plane::spiric_plane_method`
(`crates/geom-brep/src/curve_face/spiric_plane.rs:59`) rereads the
oval's count on `L = P ∩ Q` shifted by `±band.escalate()` along
`across`. `across` is L's normal **inside the spiric's plane P**. A
shift `d` across L in P moves the line only `d·s` off the cutting plane
Q, where `s` is the sine of the tilt between P and Q. So the reread
tests a miss of `escalate·s` off Q, not a miss of `escalate`.

The review ran one case at the default ε. The plane leaned to `s ≈ 0.2`
with the oval 3e-9 off Q, between the band's zero and its escalation.
It answers `Ok` with 0 meetings on one side and 2 on the other, so the
sub-band miss decides rather than refusing. A plane square to P
(`s = 1`) refuses at the same shift. By the same arithmetic, a miss
below ε decides once `s` falls below about 0.1.

The row `a_plane_missing_the_oval_by_less_than_the_band_refuses` (same
file) leaves this open: it sets its tangents square to P.

## Settles when

The reread shift is a height off Q: `±escalate/s` along `across`, or an
equivalent height. A row over `s ∈ {1, 0.2, 0.05, 0.01}` must refuse a
miss of the oval off Q between zero and escalate at every `s`.

## Progress (germ/spiric-near-miss-off-q, 2026-11-23)

The reread now reads the two lines of the spiric's plane a height
`escalate` off the cutting plane: `L` moved `escalate/s` across itself,
`s` the pose's tilt sine, carried on the form (`Formed::SpiricPlane`'s
`tilt`). The move stays under the radius of the span's cover ball,
because the pose's parallel rung has `s` times that radius past the band.

The row `a_leaned_plane_missing_the_oval_by_less_than_the_band_refuses`
(`curve_face::spiric_plane`) reads `s ∈ {1, 0.2, 0.05, 0.01}` at four
points of the oval, a miss and a dip either side, against an oracle that
finds the oval's extreme height off the plane by sampling and ternary
search on `Curve3::eval`. Gaps of 0.001, 0.1, 0.3 and 0.9 of the
escalation must refuse; gaps of 2 and 10 must answer the oracle's
crossings.

In-band gaps that decided, of 32 per lean: on main 0, 12, 24, 24 for
`s` = 1, 0.2, 0.05, 0.01; with the change 0 at every lean. Gaps between
1 and 2 escalations are not pinned either way.

## Progress (fix pass, 2026-11-23)

The guarantee is narrowed to what the reread does: it refuses where one
extreme of the oval's height off the cutting plane lies in the band. An
oval wholly inside the band on one side keeps a count of none on all
three lines and reads as a miss; that residual is the row
`a-spiric-oval-wholly-inside-the-band-on-one-side-of-the-cutting-plane-reads-as-a-miss`.

The row now leans down to `1e-5` and to 1.05 times the smallest sine the
parallel rung admits on its fixture (about 4.9e-9), and adds the
inner-equator point `v = π`. Its oracle reads the gap over the oval's
whole turn. An in-band gap must refuse on this door as
`Unsettled(Tangency)` or as the ladder's own escalation, which comes
first where the line or a moved line reads in the ladder's band; no other
refusal passes. In-band gaps that decided, per lean `1, 0.2, 0.05, 0.01,
1e-5, 4.9e-9`: with the in-plane shift 0, 15, 30, 30, 40, 42; with the
change none.

## Closed

2026-11-23, by the GERM orchestrator. Closed by PR 7571 (merge `81162c9be9`). The re-read lines are `P ∩ (Q ± escalate·n̂)`: a gap off the cutting plane with one extreme inside the band refuses at every lean down to the smallest sine the parallel rung admits. The case of an oval wholly inside the band on one side of Q is filed as `a-spiric-oval-wholly-inside-the-band-on-one-side-of-the-cutting-plane-reads-as-a-miss` (P1).
