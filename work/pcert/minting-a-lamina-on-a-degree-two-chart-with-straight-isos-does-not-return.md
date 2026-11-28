---
id: minting-a-lamina-on-a-degree-two-chart-with-straight-isos-does-not-return
kind: issue
title: Minting a lamina's pcurves on a degree-2 spline chart whose isos are straight and collinear does not return in 270 s
status: open
opened: 2026-11-25
priority: P2
cost: M
---



## Finding

Reported by the review of PR 7787 (lane `embedwalk-trimmed`), not
reproduced in the lane. Building a seamed or folded trimmed spline
fixture, the reviewer found that `topo::mint_pcurves_of` on a lamina
over a degree-2 spline chart whose isos are straight and collinear did
not return within 270 s in a debug build.

A mint that does not return is no refusal, and so no caller can type
it. It also leaves the seamed trimmed spline face, whose own pairs the
injectivity engine reads across its seam, without a test fixture.

## What is needed

Reproduce it, bound the mint's work, and answer: a pcurve, or a typed
refusal.
