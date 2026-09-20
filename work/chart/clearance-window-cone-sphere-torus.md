---
id: clearance-window-cone-sphere-torus
kind: issue
title: Cone, sphere and torus clearance windows keep M10-5's whole-carrier rectangles
status: open
opened: 2026-09-13
refs: [clearance-window-tightening-needs-chart-boundary]
---



## What

TRIM-3 PR-2 tightens a clearance carrier window to the face's chart
boundary for **planes and cylinders only** (`window_of` in
`crates/editor-core/src/clearance.rs`, the `chart_arms` arm). A cone,
a sphere and a torus keep the whole-carrier rectangles M10-5 shipped:
the full turn plus a symmetric slant range around the apex, the full
turn plus the full latitude, and the two full turns. So every D3
looseness the item
`clearance-window-tightening-needs-chart-boundary` measures still
stands on those three carriers — a `Violated` there may still be a
window pair neither face occupies, and `ClearanceReport::windows`
counts the window `loose` so a reader can see it.

## Why it was held back, per carrier

Two separate obstacles, and neither is the description's minting.

- **The region side.** `MetredBound::certifies_outside` reads the walk
  as a chord polygon and answers a parity question. A polar cap's
  outer loop is ONE circle, which the amended `certifies_outside`
  skips outright (fewer than three vertices certifies nothing), and a
  cone's lateral face and a torus's full band are periodic in a
  coordinate whose chord polygon does not bound the region it looks
  like it bounds. Each wants its own argument about which side of the
  loop the material is on; PR-2 makes none of them.
- **The arms.** The outside test decides METRED margins, and PR-2
  passes arms it can call exact — plane `(1, 1)`, cylinder `(r, 1)`.
  A cone's azimuth arm varies with the slant coordinate and a sphere's
  with latitude, so a single pair of arms is a bound, not an identity,
  and the bound has to be derived and argued before any margin on
  those charts is a length.

A third limit is the description itself and is already recorded:
`chart_boundary` refuses typed on a sphere or cone loop with a joint
at the chart's singularity (`chart-boundary-refuses-pole-crossing-sphere-loops`).

## Fix shape

One unit per region argument, NUMERIC, each with the arms question
answered first. The consumer edit is then one more arm in
`chart_arms` and one more match arm in `window_of`'s root rule — the
seam PR-2 already opened, not a new one.

## Home

TRIM — the description is `chart_bound.rs`, this program's; the
consumer edit is the announced seam in `clearance.rs`.
