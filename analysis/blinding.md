# Design fork: arc-closer-constructed-from-arrival-tangent (with via-and-center-arc-closers-declared-arrival)

Dispatched 2026-10-19 by the PATHS orchestrator. Blinding byte 129 (parity 1): Fable is A, Opus is B.
Protocol at dispatch: 26db1af89e
Deviations: none at dispatch.

## Round 1 (first reports, 2026-10-19)
- A: new endpoint-full mode `Arrive` carrying endpoint + authored end direction (director inside the mode); admitted on the sharp leg and the fused incomings; `Start.arrives_tangent()` constructs where the end is free, checks where fixed; Via/Center take the token (a leftover). Likely. Rejects no framing; widens it (the B's seam is sharp). Proposes changing ratified text (§2, §2c, §3, §6 PQ4).
- B: new mode `Arriving { p }` with the end direction bound afterwards by the ordinary director (binder-after-spec, as `fillet_arc(r, Via)`); fused rows refused; same token rule (check or supply); Via/Center take it. Likely. Same widening; same ratified-text change (§2 Closure, Legs, §2a).
- Split: where the end direction is spelled (inside the mode vs a pending director), and with it whether the fused rows admit the mode. Round 2: each shown the other's report.

## Round 2 (each shown the other's round-1 report)
- A moved to B's round-1 position: a pending director after the spec; fused rows refused (no concrete need identified). Cites `ViaArrival`'s director as exact precedent; a field would be the director vocabulary twice.
- B moved to A's round-1 position: the direction inside the mode (a two-variant end); fused rows required by §2c "converting sharp→filleted is an edit at the same call site" and round 9 "every admissible pair ships".
- A crossover. Round 3: each shown the other's round-2 report, asked what moved it, whether its own earlier argument is answered, and specifically whether §2c/round 9 make the fused rows owed.

## Round 3 (each shown the other's round-2 report; asked about §2c round 7 / round 9 and a third shape)
- A: direction in the spec as a two-variant end over ONE shared `Director` type (the steps project it); fused rows owed under round 9 (sure); "no concrete need" was use-case gating.
- B: same design, leaning (b) likely; holds that round 9 reaching a new mode's fused sites is its extension (likely, not sure), and names that reading as Ev's.
- Converged on the design; residual: how far round 9 binds, stated for Ev. To the [ev] PR.

## Round 4 (Ev's framing on #5374: "make it a way to write a 'fillet', just not necessarily arriving tangent")
- Both endorse a no-argument verb on a point tip opening the fillet family's arrival builder; arc tip→anchor, tangent at arrival. Reading agreed (tangent at arrival, free at departure).
- A: arc-arrival landing (`.at(p)`+director → directed point, departure free); sharp seam `.at(Start)`+director; name `arrive()`.
- B: line side (`.at` continues, `.to(p)` ends); sharp seam `.toward(d).to(Start)` (far-end Start for every fillet verb); name `arc_onto()`.
## Round 5 (each shown the other's round 4)
- Crossed again: A took B's round-4 cells and name; B took A's. Round 6 asks the question beneath: is the arc the arrival side itself (arc arrival) or a fillet-position arc onto a line side (line arrival).
## Round 6 (asked the question beneath: arc arrival or line side)
- Both: the arc IS the arrival (arc-arrival builder), `arrive()`; `.at(p)`+director lands a directed point, departure free; `.to(Start)` tangent closer; `.at(Start)`+director sharp seam. Deciding arguments: with no `r` the anchor is forced to be the tangent point, so line grammar would need a new rule (A) and re-spell one loop twice (B). Converged; no split. #5374 rewritten to it.
