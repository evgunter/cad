---
id: a-band-decided-order-of-off-curve-feet-needs-k-above-two
kind: issue
title: Ordering two points' feet by a band-decided reading is sound only for K > 2, and the K configuration admits any K > 1
status: open
opened: 2026-10-07
priority: P3
cost: E
---


## What

`emit_topo::param_along` and `emit_topo::chord_along` read where a
crossing point lies along an edge, and `discriminate::extent_before`
orders two readings by a difference decided through the linear band.
Each point lies within ε of the carrier. A difference decided
`Positive` is at least Kε, so the two feet are ordered only when
Kε > 2ε, that is K > 2. Both functions state the premise.

`geom_core::tolerance::Tolerance` (`k`, checked in `Tolerance::validate`)
accepts any finite K > 1, with no floor, and `CAD_AMBIGUITY_K` sets it.
The default is 10. So with 1 < K ≤ 2, the order of two crossings can
be decided wrongly with no sign of it.

Options: give K a floor of 2 (or above) at validation, or state the
premise where K is configured and let the decisions that rely on it
say so. The tolerance surface belongs to this program. Nothing has
been changed there.

## Re-homed from FLUX to SCALAR (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. SCALAR collects the scalar doors and the readers over them: the certified-door family, the margin and recourse readers, the unit-direction doors, and the box driver's readings. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
