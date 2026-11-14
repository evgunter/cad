---
id: a-plane-crossing-past-the-band-of-a-thin-ellipses-fast-end-read-at-the-end
kind: issue
title: A plane crossing past the band of a thin ellipse's fast end was read at the end, and the split dropped it: span_place metered the end gap at the inf speed
status: closed
opened: 2026-11-09
priority: P0
cost: E
closed: 2026-11-09
pr: 6687
branch: onecopy/needle-basin
---


Found by `onecopy/needle-basin` while routing span membership to one home.

## What

`geom_brep::curve_face::plane_crossing::span_place` read a root's place
on a conic span as `(t − t₀)·meter` and `(t₁ − t)·meter`, with `meter`
the smaller semi-axis: an INF bound on the speed. That is sound for
"inside" (a gap it proves clear is clear in metres) but not for "at the
end": a Zero read there covers an arc up to `(speed_max/speed_min)·z`.
On an ellipse 0.01 × 10 over `[0, 0.3]`, a plane at
`y = 10·sin δ` crosses at `t ≈ δ`: for `δ = 5e-8` the crossing is 5e-7 m
from the end (past the zero at default ε) and was read AtEnd, so the
split lost a genuine crossing. A wrong answer, not an escalation.

## Fixed: the wrong answer is a typed refusal

The lane reads `curve_pair::Span::place_param` (an end by the raw chord
to its point, else the side by the parameter metered at the SUP speed),
the reading `Span::place` already gave a foot. Pinned by
`a_crossing_near_a_fast_end_is_placed_by_its_chord`
(`crates/geom-brep/src/curve_face/plane_crossing.rs`), red on main.

End to end the crossing is not yet inserted. `split_edge`'s own
interiority gate still meters the gap at the speed floor, so across
`Kε … (sup/inf)·Kε` of a fast end it refuses (`SplitParamNotInterior`,
or `SplitParamEscalated` in band), and `split_reduce` raises
`CrossingInsertion`: a wrong answer (a crossing silently dropped) became
a typed refusal. The split lands with
`the-euler-splits-interiority-gate-meters-an-end-gap-at-the-speed-floor`.
