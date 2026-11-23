---
id: a-near-full-arcs-span-reads-a-chord-scale-apex-at-interval
kind: issue
title: paths: an arc near a full turn reads its span off the chord-scale apex, whose Interval enclosure widens as tan²(θ/4); a clear near-full tangent arc that validates at f64 escalates at Interval
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Found by `tangent-arc-near-a-full-turn-loses-its-end-vertex` (its
1.36e4 row).

**Measured.** `At(0,0) Toward(1,0) TangentArcTo((-2.7, 2.68e-4)) LineTo(Start)`,
R = 1.36e4, default ε. With the tangent arc fixed, the stored sweep's
Interval enclosure is 7e-15 wide and its landing holds the end vertex.
The arc is clear of the full turn: its diameter clearance is 3.3e-5 m,
past Kε. `f64` validates. `Interval` refuses "whether a point falls
inside an arc's span is undecided", with enclosure ±9.7e-7 at the shared
vertex, whose true margin is zero.

**Why.** `seg::arc_span` reads `|a − apex| − |q − apex|`, and
`seg::classify` builds `apex` by `Arc2::apex`, the chord-scale
`mid − n̂·L·tan(θ/4)/2`. Near a full turn `tan(θ/4) ≈ 4/(2π − θ)`, so the
sweep's width is multiplied by about `L·tan²(θ/4)/4`: ±1e-6 here from a
7e-15 sweep. `Arc2::apex`'s doc chose the chord scale for FLAT arcs,
where the centre's enclosure is the wide one. Near a full turn it is
the other way round: the carrier-scale apex `centre − n̂·sign(Δθ)·r` is
as narrow as the centre, which `tangent_arc` builds from `across`
directly. The f64 doc on `arc_span` already states the matching
conditioning, 1/cos(θ/4).

**Question.** Should the span's apex be chosen by scale, or the reading
moved to an angular test past some turn? Either way, the conditioning
and its doc both change.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
