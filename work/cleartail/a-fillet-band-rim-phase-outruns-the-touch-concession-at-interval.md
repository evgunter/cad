---
id: a-fillet-band-rim-phase-outruns-the-touch-concession-at-interval
kind: issue
title: A fillet band's rim phase at Interval is wider than the touch concession, so the filleted drum's injectivity check refuses Budget at Interval and holds at f64
status: open
opened: 2026-10-30
priority: P2
cost: M
---


## Finding

Found by RIMSCALAR, which runs every row of
`crates/sweep/tests/self_intersection_proves_injectivity.rs` at
`Body<Interval>` beside `Body<f64>`. The row
`a_filleted_drum_certifies_across_its_fillet` (the extruded unit disc,
its top rim filleted at `0.3`, built at each scalar by `extrude` and
`sweep::blend::fillet_edges`) reads:

| body | verdict |
|---|---|
| `Body<f64>` | Holds: 11 candidates, 581 discharged, 150 certified, 36 outside |
| `Body<Interval>` | `Refused(Budget(Depth { max_cell_depth: 40 }))`: 1161 discharged, 202 certified, 240 outside, 36 refused |

The row pins the Interval verdict as this known divergence.

**Where it departs.** The torus band's chart image of each half-rim it
shares with the wall (`v` an angle channel of winding 0) carries its
phase at Interval as `[−1.7e-14, 1.7e-14]`; at `f64` it is a point at 0.
A band cell abutting the rim from outside the band, `v ∈ [2π − δ, 2π]`,
is read outside by `injective::present_chart`'s touch concession only
where `line.lo() >= hi − few_ulps(hi)`. `few_ulps(2π)` is about
`1.1e-14`, narrower than the phase's enclosure, so at Interval the cell
is read as holding the face. The (wall, band) touching pairs there
split instead of certifying (depth 14: 8 splits at Interval to 2 at
`f64`). Their children pair wall cells just under the rim, clear of it,
with band cells outside the band but within reach of the wall, which
the distance test never separates: each split leaves one half abutting
the rim. The walk reaches depth 40 on (wall, band) pairs, which no
cross-face pair at `f64` reaches past depth 15.

**Not RIMSCALAR's cause.** RIMSCALAR's divergence was an exact
comparison on an ulp-wide enclosure: a winding's rate, `w·τ/τ`, read
by `injective::Trig::of`'s `exactly(pl, ±1)`, which the `f64` reading
holds as the point `w`. This one is a few-ulps concession calibrated
on `f64` rounding, which a derived enclosure (the fillet's torus phase)
outruns. Widening the touch to the line's enclosure would loosen a
concession. Whether to do that, to tighten the fillet's phase
enclosure at its derivation, or to read the band's rim structurally
(the band meets the wall tangentially along a rim of the band's own
chart), is the question.

Evidence: the row's `[rimscalar] the filleted drum` print, and
`injective.rs` `present_chart` (the "A cell past the line on its face
side by no more than `few_ulps` lies outside it" clause) and `vacuous`.
