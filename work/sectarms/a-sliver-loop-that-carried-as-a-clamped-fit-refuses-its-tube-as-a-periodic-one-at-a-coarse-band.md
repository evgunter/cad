---
id: a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band
kind: issue
title: a cylinder × sphere loop a millimetre short of tangency carried at ε = 1e-6 as a clamped fit and refuses its tube limb as a periodic one
status: dispatched
opened: 2026-11-17
priority: P1
cost: M
branch: sectarms/sliver-loop-tube
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

## What differs (measured by PR 7052's reviewers)

On the same 438 marched points at ε = 1e-6:

| `(d, R)` | periodic fit | clamped fit |
|---|---|---|
| `(0.2, 0.69)` | carries, tube radius 1.08e-2 | carries, tube radius 1.08e-2 |
| `(0.2, 0.699)` | refuses `TubeStraddles` (longest span 3.0e-3) | certifies at tube radius 1.7e-4, 60 times narrower than a normal pose's (longest span 4.9e-3) |
| `(0.2, 0.6995)` | refuses | refuses |

So the refusal boundary moved from between `R = 0.699` and `0.6995` to
between `0.69` and `0.699`: the clamped fit was already at its last rung
on this pose. The periodic fit's longest knot span is the *shorter* of
the two, so the span length this row first guessed at is not the cause.
**The cause is not found.**

## What would answer it

Read the box where limb 3's transversality enclosure holds zero on the
periodic fit at `(0.2, 0.699)`, and the same stretch of the clamped
fit's chain at its 1.7e-4 rung.

`pair_section::a_seated_loop_has_no_start` pins the pose as measured:
carried below a band of 1e-7, `TubeStraddles` above it.
