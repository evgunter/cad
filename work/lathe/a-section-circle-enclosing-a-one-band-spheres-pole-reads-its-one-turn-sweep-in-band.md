---
id: a-section-circle-enclosing-a-one-band-spheres-pole-reads-its-one-turn-sweep-in-band
kind: issue
title: A section circle enclosing a one-band sphere's pole reads its one-turn azimuth sweep a rounding short, and escalates at ε 1e-12
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Found by the PR 7414 re-review (R1, 2026-11-25) as a row red on head
only at ε 1e-12; measured by the LATHE one-band lane on
`lathe/full-revolve-one-band` at 05ea73ec93.

## What

`crates/sweep/tests/tilted_sphere_pair.rs`
`a_section_passing_near_a_pole_or_through_it_builds`, the union at
`δ = 1e-4`, refuses at ε 1e-12:

```
Pcurves { Certify { error: Escalated { check: AzimuthPeriod,
  margin 1.2496670365180762e-12, band { zero: 1e-12, escalate: 1e-11 } } } }
```

At ε 1e-9 and 1e-6 it builds, and at 1e-12 δ = 0, 1e-5 and 3e-4 build;
base builds every δ at every ε.

## Cause, measured

At `δ = 1e-4` the radical circle encloses the first ball's south pole.
On base the ball was two half-bands, so the circle was split at the
π meridian into two edges of about π of azimuth each. On the one band
it is one closed edge, a full-period circle (`t` over `(−π/2, 3π/2)`),
whose projected image winds once round the pole and sweeps **exactly
one turn**. The period gate of the projected lane
(`crates/geom-brep/src/pcurve_cache/projected.rs`, check 2,
`one_turn.not_over(PERIOD_ROWS, azimuth, Bounded::new(inf, u_arm), band)`)
reads that sweep off `ProjectedImage::sweep_box`: `u` from
`3.12e-13` to `6.283185307178649`, `2π − 1.25e-12`. The box reaches
`v = −1.5707033`, `9.3e-5` from the pole, where an azimuth amplifies a
position's rounding about `10⁴`-fold. The headroom is read at the sup
arm (`1.0`), so a one-turn tie reads `1.25e-12` m: Zero at ε 1e-9 (not
over, passes), in band at ε 1e-12, where the sup reading itself is
undecided and the turn rule escalates (`TurnUndecided`, `sup: None`).

## Question

A closed carrier's image round a pole is one turn by construction, so
the tie is structural, not a near miss. Either the image's sweep is
read so a one-turn loop reads exactly one turn (its ends on the
carrier's own seam crossing rather than re-derived near the pole), or
the gate reads a closed carrier's winding instead of the sweep's
extent. Typed and loud as it stands, so P2: no body is shipped wrong.

## Pinned

The row pins the escalation at exactly this pose and ε (δ = 1e-4, ε
1e-12) with this row cited, and every other δ and ε building.

## Ground

`crates/geom-brep/src/pcurve_cache/projected.rs` (the projected lane's
period gate and `sweep_box`): lathe.
