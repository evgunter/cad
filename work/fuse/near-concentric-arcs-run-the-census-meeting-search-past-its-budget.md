---
id: near-concentric-arcs-run-the-census-meeting-search-past-its-budget
kind: issue
title: Two arcs within about 1e-5 of each other along a long run (concentric circles) run the census's meeting search past its budget and refuse, where the first-order bend charges each carrier's whole curvature
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by FUSE's review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

Measured on PR 4946's search: two concentric coplanar unit circles'
arcs read `Some(false)` at Δr = 1e-3 (515 pieces), and run past
`bisect::PIECE_BUDGET` and refuse `CensusUnsupported` at Δr = 1e-5 and
1e-7, at ε 1e-9 and 1e-12. On main they read stale (the circle closed
form found no crossing). Both are errors at the census; the refusal is
the loud one.

The second-order clear charges each piece's bend `|C″|·h²/2` against
the gap, so two near-parallel pieces clear only once `h² < Δr`: about
`2.6/√Δr` pieces a carrier. A bound on the DIFFERENCE of the two
carriers' second derivatives (the relative bend), or reading two
circles' radial gap in closed form, would clear them at the first
levels.
