---
id: a-half-turn-kill-under-a-band-tighter-than-pi-over-two-misses-as-decided
kind: issue
title: A half-turn kill under a band tighter than K = π/2 meets its ends yet misses as decided
status: open
priority: P3
cost: E
refs: [an-in-band-near-pole-read-is-dropped-so-a-half-turn-joint-misses-as-decided]
opened: 2026-10-14
---

Found writing PR 5071's fix pass (the half-turn argument both its
reviewers asked for).

`topo::pcurves::turn_element` reads a killed half's turn where its two
ends meet: `pcurve_turn_closes` decides their 3-D distance Zero, i.e.
within the band's zero ε. A stored gap of half a turn is admissible
(the ends meet within ε, not exactly). On a band-sized lever `ℓ`
(a narrow cylinder, or a vertex near a narrow cone's apex) the chord
between the two half-turn ends is `2ℓ`, so the ends meet for
`ℓ ≤ ε/2`.

The near-pole read then meters the two branches astride the gap's mark
at `π·ℓ ≤ (π/2)·ε`. That is under the band's escalate edge, so the read
closes or escalates, only where `escalate ≥ (π/2)·zero`, i.e. `K ≥ π/2
≈ 1.571`. `geom_core::Band::new` admits any `K > 1`, and
`pcurves::room_fence_tests` names `K = 1.5` the tightest band in
practice. For `π/2 > K > 1` a window `ℓ ∈ [escalate/π, ε/2]` remains:
the ends meet, every branch tried is decided off at `π·ℓ ≥ escalate`,
and the turn answers a decided miss (`turn_element`'s `Err(None)`) on
ends the kernel itself decided coincident.

Not measured: no row builds a `K < π/2` band through a kill turn. The
fix is either to read the read's band against the closure's (a joint
whose ends were decided to meet cannot be decided apart at the same
lever), or to require `K ≥ π/2` where `Band` is minted for the loop
walk.
