---
id: tier-3-prime-reads-no-side-at-the-end-of-a-line-touch-inside-one-solid
kind: issue
title: "Tier 3′ reads no side at a point where a line touch inside one solid ends: neither the line rule nor the point rule reaches it"
status: open
priority: P3
cost: M
design: true
opened: 2026-10-21
---

Found by `tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`'s
lane (branch `restread/point-touch-winding`).

## What

D1 tier 3 (`crates/topo/README.md`) reads a solid touching itself along
a line by its line rule, and at an isolated point by its whole link's
winding. The self-touch reading (`sweep_self_touch_points`,
`crates/topo/src/census.rs`) skips a point that a line touch of the
solid reaches (`line_touch_reaches`): two of its edges on one line
there, or one lying in a face of it. Such a point is no isolated touch.
The line rule reads the line's sides at its cells' middles
(`edge_in_own_face`) and, once built, along coincident edges. Neither
reads the point where the line ends.

The skip is what lets the slit forms pass: the split's one-end slit,
the keel in the floor, the keel wrapped at both ends, the straddle
notch (`review_m3_pr6`, `review_s6_probe`), and the split at a
crossing whose edge stays whole. At each of these the link does not
gather: a face is held both at a corner and along an edge, or the
section face carries the slit as a zero-area ring.

## The question

Is there a crossing a line's end can carry that neither the line's
cells nor any isolated point nearby shows? Two planes crossing through
the end cross along a line from it, and that line ends at another
event. The case the reading cannot see is a sheet crossing there whose
other end is again a line touch's end. If the answer is yes, the end
needs a reading of its own: the link with the line's coincident sheets
paired off, the zero-area ring exempt as D1 exempts its winding.
