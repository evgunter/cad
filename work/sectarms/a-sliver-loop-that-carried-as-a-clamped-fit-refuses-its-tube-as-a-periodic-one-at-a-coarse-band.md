---
id: a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band
kind: issue
title: a cylinder × sphere loop a millimetre short of tangency carried at ε = 1e-6 as a clamped fit and refuses its tube limb as a periodic one
status: open
opened: 2026-11-17
priority: P1
cost: M
---


Found by the periodic-loop carrier unit
(`a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses`,
branch `sectarms/periodic-loop-carrier`), running its own rows at the
three ε.

## Measured

`PairSection::carry` of a cylinder of radius 0.5 about `z` against a
sphere of radius `R` centred `d` from the axis (`geom-brep`'s
`tests/pair_section.rs` fixtures), through `SsiLane::marched()`, at the
witness band of each ε:

| `(d, R)` | ε = 1e-6 | ε = 1e-9 | ε = 1e-12 |
|---|---|---|---|
| `(0.5, 0.3)`, `(0.35, 0.3)`, `(0.1, 0.7)`, `(0.62, 0.15)`, `(0.2, 0.69)` | carries | carries | carries |
| `(0.2, 0.699)` | **refuses** | carries (2458 knots) | carries (13828 knots) |
| `(0.2, 0.6995)` | **refuses** | carries | carries |

The refusal is `CarryRefusal::March(SsiError::TubeStraddles { verdict:
Zero, margin 0.0, boxes: 437 })`: limb 3's transversality clearance over
the loop's tube boxes is not zero-free on any rung of the ladder. The
sphere reaches within `0.7 − R` of the far wall, `1e-3` and `5e-4` on
the two refusing poses.

On main (`72981a1b00`), where a closed branch is fitted clamped,
`(0.2, 0.699)` carries at ε = 1e-6: `pair_section::a_seated_loop_is_the_loop_point_for_point`,
which carries all five poses of the first two rows, passes there at
`CAD_TOLERANCE_EPS=1e-6` (run). `(0.2, 0.6995)` was not run on main.

So this is a pose the periodic fit lost at one band.

## What differs (read, not run)

The clamped fit interpolates on *averaged* knots
(`geom::curves::fit`, `averaged_knots`: each interior knot the mean of
three consecutive chord parameters), so each knot span is a three-step
mean of the march's steps. The periodic fit
(`geom::PeriodicNurbs3::interpolate`) puts its knots at the chord
parameters themselves, so a knot span is one march step. Limb 3 reads
one box per knot span; where the march's steps are uneven, the periodic
fit's longest span is longer than the clamped fit's, and its box's
enclosure of the transversality wider. Whether that is what refuses
here is not measured: the refusing box, its span and the same box on
main were not read.

## What would answer it

Read the refusing box on both fits at this pose. If it is the span
length: a periodic fit on cyclically averaged knots (a cyclic banded
collocation, the open fit's own scheme made cyclic), or refinement of
the samples where limb 3 straddles, as `refine_for_tube` refines where
the tube is narrow.

`pair_section::a_seated_loop_has_no_start` pins the pose as measured:
carried below a band of 1e-7, `TubeStraddles` above it.
