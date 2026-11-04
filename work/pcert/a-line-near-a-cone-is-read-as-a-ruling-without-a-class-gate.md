---
id: a-line-near-a-cone-is-read-as-a-ruling-without-a-class-gate
kind: issue
title: A line within the band of a cone that is no ruling takes the ruling's closed form and refuses at check 4, rather than taking the general lane
status: closed
opened: 2026-11-03
priority: P1
cost: M
pr: 6504
closed: 2026-11-04
---



Filed by PCERT unit E4 (`pcert/circle-general-lane`), which gave a line
on a cone its general (projected) lane for the nappe reading in band
(`projected::has_lane`, `pcurve_cache::cone_line_rows`).

## What

The cone arm of `derive_class` (`pcurve_cache.rs`, the cone chart's
`Curve3::Line` case) reads only the line's nappe
(`pcurve_cone_chart_nappe`, on the anchor's height and then its slope)
and then images the line as the ruling at its anchor's azimuth
(`Derivation::ConeRuling`). No reading asks whether the line is a
ruling at all. A line the incidence read does not show off the cone
that is no ruling therefore takes the ruling's closed form, and check 4
refuses it as a miss, where D3's lane choice says a class not shown takes
the general lane.

Measured on E4's branch: on the cone of `cone_line_rows::cone(0.5)`,
the ruling of azimuth 1 anchored at height 2, shifted `3e-5` along the
cone's tangent ⊥ the ruling, over `t ∈ [0, 1]`, leaves the cone by under
`ε` (1e-9), so the incidence read shows it on; `derive_harmonic` answers
`ConeRuling { anchored: true, nappe: Pos }`, and `PcurveCache::certify`
refuses `ResidualExceeded { check: MapResidual }`. Shifted `1e-4`, the
incidence read escalates first (`ChartIncidence`).

This is the cone's twin of the cylinder fold E2 removed (a definite
non-Zero radial reading on a line now takes the general lane there).

## Fix

A ruling reading on the cone arm: the line through the apex (its
distance from the apex, or the angle of its direction off the generator
at its azimuth, in metres at the edge's reach), read as a lane choice:
shown, the ruling's closed form; in band or definite, the general lane,
which E4 wrote. Row: the probe above, minting its projected image and
certifying; and the line read at a pose where the apex is in its
interval ending "not yet" through the lane's own sector refusal.

## Band

P1, as Ev ruled the D3 lane units on `[ev]` PR 5186: a class a lane
choice reads has a general lane, and until it does the class is handled
by a special case. This one is worse than "not yet": its ending,
`ResidualExceeded { MapResidual }`, says the pcurve does not represent
the carrier.

## Also widen `chart_iso_family`

`pcurve_cache::chart_iso_family`'s in-band arm reads only the
cylinder's ruling predicate (`RADIAL_MOVING`): an in-band reading of any
other class keeps its image refusal. Its definite arm ("no family") was
guarded by `matches!(surface, Surface::Cylinder { .. })`; E4 widened that
guard to `projected::has_lane`, so a cone line shown off its rulings,
once the cone arm reads them as a lane choice, already answers no
family. The fix widens the in-band arm to the cone's ruling predicate.


## Closed

PR 6504. The cone's line arm reads `pcurve_cone_chart_ruling`: the
line's distance from the generator its anchor names, through the apex,
over the span (`ruling_miss`). Zero takes the closed form. In band or
definite, the line takes the general lane. `chart_iso_family`'s in-band
arm reads it too. E4's probe now takes the general lane and certifies,
at ε 1e-9, 1e-6 and 1e-12 (`ruling_reading_rows`), and is red under
the `cone-ruling-gate` mutant.
