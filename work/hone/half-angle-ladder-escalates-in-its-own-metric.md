---
id: half-angle-ladder-escalates-in-its-own-metric
kind: issue
title: The half-angle ladder no longer answers but still escalates, in its tau metric, configurations the subdivision would certify
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [the-half-angle-ladder-certifies-in-band-configurations, non-circle-conic-edge-refuses-against-every-curved-face]
---


Found by REACH's ellipse lane, PR 3805's fix pass (2026-10-02).

## What

`circle_roots::half_angle_roots` answers by certified subdivision in
the residual's metres (`certified_subdivision`); the quartic ladder
before it no longer decides anything, but it still runs, and an in-band
sign it meets escalates as the door's decision (`ArcSphereRoots`,
`ArcCylinderRoots`, `ArcTorusRoots`). Its margins are read in its root
variable `τ`, which is arc length only to first order about the anchor
(along an ellipse, to within the semi-axes' ratio), so "in band" there
is not "in band" on the surface: it escalates configurations the
subdivision certifies.

## Measured

`ellipse_roots::fuzz_rows`, the pinned seed (600 millimetre ellipses,
eccentricity to 25), at ε = 1e-6: 422 certified, 65 `Uncertain`, 113
escalated. With the ladder's `τ` scaled at the semi-major axis instead
of the semi-minor (a change that moves only the ladder): 500 certified,
92 `Uncertain`, 8 escalated — every one of the extra answers checked
against the true distance. At ε = 1e-9: 7 escalations against 0.

## The shape of a fix

Retire the ladder from the three doors and give the subdivision the
escalation posture the first-harmonic door has: a piece called down to
the band (a double root) decides the residual at its midpoint, `Zero`
answering `Uncertain` and the band's gap escalating as the door's
decision. Rows that pin a ladder escalation by predicate name
(`circle_cylinder`'s in-band rim row among them) move to the
subdivision's rows.
