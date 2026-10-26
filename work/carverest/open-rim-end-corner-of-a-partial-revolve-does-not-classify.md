---
id: open-rim-end-corner-of-a-partial-revolve-does-not-classify
kind: issue
title: blend: a partial revolve's open neck-flare rim ends in a corner battery::corner_at reads as Indeterminate
status: open
opened: 2026-10-06
priority: P3
cost: M
---



## Finding

Witness: the tour's Klein bottle (`demos/tour/src/klein.rs`,
`wall_probes`, wall 2). The sharp-cornered meridian band revolved
PARTIALLY (5 rad) has an open neck→flare rim, a cone×cylinder arc whose
two ends meet the revolve's planar cut faces. `fillet_edges` on the inner
wall's corner at `RF + WALL/2 = 0.325` refuses
`UnsupportedCorner { corner: Indeterminate, policy: Some(RunOutStopAtVertex) }`
at one of those end vertices. The same corner on the FULL revolve (a
closed rim, no end) rolls, and the rolled body has the authored bulb's
volume to nine digits, so the arm and the headroom are not what stops the
partial one: the end corner is.

`battery::corner_at` folds any neighbour edge whose own supports refuse
or escalate into `CornerConfig::Indeterminate` (the comment there,
"the fold is deliberate and it is lossy"), so the refusal does not say
which neighbour failed or why. Whether this corner (two meridian-plane
cut faces against a cone and a cylinder, the rim arc running into them
at right angles) should classify as a transverse cap is the question.

Surfaced when sided radius headroom
(`radius-headroom-reads-a-ball-outside-a-concave-support-as-inside`)
let the bottle's corners past predicate 1. Before that, both revolves
refused `RadiusHeadroom` first.

## What the taker owes

Measure which neighbour edge's resolve fails at that vertex and why. Then
either classify the corner, so the partial revolve rolls as the full one
does and wall 2 retires, or name the refusal the corner actually owes.
