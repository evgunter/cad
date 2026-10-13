---
id: near-concentric-arcs-run-the-census-meeting-search-past-its-budget
kind: issue
title: Two arcs within about 1e-5 of each other along a long run (concentric circles) run the census's meeting search past its budget and refuse, where the first-order bend charges each carrier's whole curvature
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [two-curve-curve-meeting-finders, 4980]
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

## 2026-10-13: why a relative-bend bound does not reach it, and a candidate

From PR 4980. The orchestrator ruled that the closed form below belongs to
`work/fuse/two-curve-curve-meeting-finders.md`, where kind-specific
closed forms will live, rather than being patched into the census first.
The window fails loud, so there is no pressure toward a silent answer.

**Today's behaviour, pinned.** Row
`census::curved::tests::near_concentric_arcs_meet_part_or_refuse_loudly`
covers two concentric unit-circle arcs, `(0.3, 2.9)` and `(0.5, 3.1)`:

| Δr | Reading |
|---|---|
| half the coincidence threshold | `Some(true)` |
| `1e-3` | `Some(false)` |
| `1e-5`, at every ε row | refuses `CensusUnsupported`, the budget, on both edges |

The row goes red if the window ever answers silently. Measured window:

| ε | Reads true | Refuses on the budget | Reads false |
|---|---|---|---|
| 1e-6 | Δr ≤ 1e-6 | `1e-5` | Δr ≥ 1e-4 |
| 1e-9 | Δr ≤ 1e-10 | `1e-8` to `1e-5` | Δr ≥ 1e-4 |
| 1e-12 | Δr ≤ 1e-13 | `1e-10` to `1e-5` | Δr ≥ 1e-4 |

**Why a relative-bend bound is O(h), not O(Δr).** The pair walk bounds
`D(σ, τ) = A(m_a + σ) − B(m_b + τ)` with mean-value remainders,
`½A″(ξ)σ² − ½B″(η)τ²`, where `ξ` and `η` are independent points of the
two pieces.

- A bound on the difference of the second derivatives charges
  `|A″(ξ) − B″(η)|`. On two concentric unit circles that is about
  `|ξ − η| + Δr`, and `|ξ − η|` runs up to the piece width `h`. So the
  "relative bend" is O(h), and the remainder is O(h³).
- The bound therefore clears only once `h ≲ Δr^(1/3)`: about 4.6e-3 rad
  at `Δr = 1e-7`, roughly 560 pieces a side at depth about 9. With 2 to
  4 neighbours per piece over those levels, that is about 9k pair reads
  against the 4096 budget.
- This is better than today's `√Δr`, but not "the first levels", and
  `1e-7` still refuses.
- Pairing the pieces by matched parameters, `A(s)` against
  `B(λs + ω)`, hits the same wall. The `ω²` and `σω` terms carry the
  whole `|B″|`, and need the same `h³` split between the tangential and
  radial directions.
- The cancellation is structural: `|D|` depends only on `σ − τ` here.
  Absolute-value bounds destroy it.

**Candidate closed form: a coaxial proxy for circle × circle.** For two
circle carriers `A` (centre `c_A`, unit axis `a_A`, radius `r_A`) and `B`:

1. Let `A*` be the circle with `B`'s centre and axis, radius `r_A`, at
   `A`'s height `z = a_B·(c_A − c_B)` along `B`'s axis.
2. Every point of `A` lies within
   `δ ≤ |c_A − c_B − z·a_B| + r_A·|a_A × a_B|·√2` of its twin on `A*`.
   This is the centre's offset across `B`'s axis plus the frame's tilt;
   the `√2` covers the rotation of the in-plane frame. The constant
   should be re-derived tightly when this is built.
3. The distance from every point of `A*` to `B` is the constant
   `√((r_A − r_B)² + z²)`.
4. So `dist(A, B) ≥ √((r_A − r_B)² + z²) − δ`, decided once for the
   pair: positive clears the root; Zero within the band is a coincident
   rim.

This is exact on coaxial rims (lathe bodies, fillets on cylinders) and
does nothing for near-parallel ellipses or splines. It is a candidate for
`two-curve-curve-meeting-finders`' design: the census would read it as
a root-level clear before the generic search, beside `carrier_cross`'s
closed-form line, circle and ellipse crossings.
