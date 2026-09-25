---
id: annulus-rim-host-outer-boundary-is-metered-only-by-the-sampled-screen
kind: issue
title: blend: an annulus rim's host outer boundary is metered only by predicate 2's sampled screen, which overestimates an off-sample gap
status: open
opened: 2026-09-25
priority: P1
cost: D
---


## Finding (the code's own statement; not re-measured here)

`ring_clearance_pass` (`crates/sweep/src/blend/surgery.rs`) meters a
LADDER rim's host outer boundary in closed form, but leaves an ANNULUS
rim's host outer boundary to predicate 2's sampled boundary-pair
screen. Its comment argues that is exact for coaxial latitude circles
and says it stops holding for a boundary whose closest approach falls
between samples — "a non-coaxial ring, or a trimmed wall. Such bodies
exist and are rowed (an off-axis bore's cap,
`review_ring_clearance_r1_probes`), so this is a live gap on the
boundary question". A sampled gap is never smaller than the true one,
so the screen can PASS a trim circle that reaches the boundary between
samples, and nothing exact backs it.

Found while sweeping for carves that move a face boundary without
metering the rest of that face (the ruled cut-off's cap gap,
`ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver`).

## What the taker owes

Measure it: an annulus rim whose host's outer boundary carries a line
or circle edge whose closest approach to the trim circle falls between
the screen's `CHAIN_SAMPLES` stations, at a radius where the true gap is
negative and the sampled one positive. If the carve returns a body,
add the closed-form walk the ladder already has (restricted to the
boundary edges the trim does NOT replace).
