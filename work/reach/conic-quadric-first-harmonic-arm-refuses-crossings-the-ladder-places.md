---
id: conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places
kind: issue
title: The conic x quadric first-harmonic arm charges the dropped second harmonic to definite crossings the ladder places
status: open
opened: 2026-10-04
priority: P2
cost: M
design: true
refs: [circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound, conic-quadric-doors-choose-their-first-harmonic-arm-two-ways]
---


Found by the lane that gave `boolean::conic_quadric` one arm switch
(`conic-quadric-doors-choose-their-first-harmonic-arm-two-ways`).

## What

`conic_quadric::conic_quadric_roots` takes its first-harmonic arm when
the second harmonic's amplitude `A₂` is in the zero band, and charges
`A₂` to both extremes' noise. `circle_roots::first_harmonic_roots`
then meters each root's slack as `speed·(noise/|R′| + …)`, so a
definite crossing whose slope `|R′|` is under about
`speed·A₂/zero` refuses `Uncertain`, where the ladder, which drops
nothing, certifies it. The arm is required for a constant residual and
for a tangency (the module docs); for a definite crossing it is the
weaker of the two.

## Evidence

The arm-switch probe of the lane above (a circle tilted against a wall,
over wall radii 1 mm, 1 m and 100 m, tilts 0 to 1e6 zero bands, grazing
inside and outside, crossing and coaxial; each answer checked against a
sampled true-distance oracle), on the poses whose tilt was definite or
in its gap while `A₂` read zero — the poses the tilt switch sent to the
ladder:

| ε | poses | ladder certified, first-harmonic arm `Uncertain` | of those with `A₂ ≥ 0.01·zero` |
|---|---|---|---|
| 1e-9 | 380 | 59 | 21 |
| 1e-6 | 220 | 15 | 8 |
| 1e-12 | 500 | 102 | 17 |

Every refused answer the ladder gave was true of the geometry; neither
arm answered wrongly. The rows with `A₂` under a hundredth of the band
are the rounding charge
(`circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound`);
the rest are this one — e.g. at ε 1e-9 a unit circle tilted 6e-5 m
against a unit wall (`A₂ = 0.9·zero`), crossing at `θ ≈ ±1.98`: the
ladder certified both roots on the wall, the arm refused.

## The shape of a fix

A design question. Two shapes: charge the dropped harmonic at the
root (`A₂·|cos(2θ − φ₂)|` bounded on the root's bracket) rather than
everywhere; or take the first-harmonic arm only for what needs it
(the constant residual and the tangency its extremes read in the band)
and hand a definite crossing to the ladder.
