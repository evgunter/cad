---
id: chart-boundary-refuses-pole-crossing-sphere-loops
kind: issue
title: chart_boundary refuses a pole-crossing sphere loop, whose chart polygon it cannot state
status: open
opened: 2026-09-04
---


## What

`topo::chart_boundary` (`crates/topo/src/pcurves.rs`) fences its loops
on EXACT chart closure: after `walk_loop` returns, the first entry and
the last exit must meet with no period allowed, or the loop is a lift
and the description refuses `PcurveMintError::LoopWraps`.

That fence is a superset of TRIM-3's spec, which named only
`loop_closes`' `± τ` arm. It also catches the SPHERE INVOLUTION arm
(`crates/topo/src/pcurves.rs`, `loop_closes`' final limb: a
pole-crossing loop legitimately ends on the twin representation of its
start, azimuth off by π with the polar channel mirrored). A face whose
outer loop crosses a pole therefore gets no description at all, even
though it is a perfectly ordinary at-rest face that `mint_pcurves`
accepts.

## Why it is fenced rather than handled

The refusal is the SOUND direction — the chord from the last exit back
to the first entry is spurious on such a walk, so a polygon built on it
would bound a region the face does not have, and
`certifies_outside` would then certify cells the face occupies. What is
missing is not a fix to the fence but the missing half: what a sphere
chart's boundary polygon IS across a pole, where the azimuth chart
degenerates and the involution is the only continuous representation.
That is the same argument TRIM-3 §3 declines to make for cone, sphere
and torus window tightening.

## Who needs it

Nobody today. TRIM-3 PR-2's consumer (`clearance.rs` `window_of`)
tightens PLANE and CYLINDER windows only; cone, sphere and torus keep
their existing carrier windows, so no shipped caller asks a sphere
chart for a boundary. This is a capability gap disclosed at the moment
it was created, not a defect with a consumer waiting on it.

## Shape of a fix

Either (a) a polar-cap description whose polygon is stated in a chart
that does not degenerate at the pole, or (b) a typed refusal narrower
than `LoopWraps` so a future consumer can tell "this loop lifts the
azimuth" from "this loop crosses a pole", which are different facts
with different recourses. (b) is cheap and is what the cone/sphere/
torus tightening unit would want first.
