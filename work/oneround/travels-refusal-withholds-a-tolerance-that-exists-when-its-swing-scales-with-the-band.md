---
id: travels-refusal-withholds-a-tolerance-that-exists-when-its-swing-scales-with-the-band
kind: issue
title: travel's refusal withholds a tolerance that exists when its swing scales with the band
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [7442]
---



Found by the review of PR 7442 (`oneround/readings-not-levers`, finding d1) and left as a stated limit of that unit.

## What

`geom_core::k_stats::decide_across(names, datum, swing, band)` decides the bracket `[|m| − s, |m| + s]`. Where the bracket holds zero at the band it is asked in, its refusal offers no tolerance (the far row keeps its own reading; the floor's gate refuses a floor decided Zero). For a swing that is a fixed length that is the whole truth: the floor stays zero at every smaller tolerance.

`topo::boolean::join::travel` hands it a swing made from the band, `s = c·ε` with `c = 2·cot ψ`. That swing shrinks with the tolerance, so a datum that is not zero is decided once `ε′ ≤ |m|/(K + c)`, and the refusal says nothing of it. In a band of `(1e-9, 1e-8)`:

| datum | swing | refused on | offers | decides Positive at |
|---|---|---|---|---|
| `1e-9` | `1.5ε` | the far row | none | `ε′ ≈ 8.7e-11` |
| `3e-10` | `9.9ε` | the floor | none | `ε′ ≈ 1.5e-11` |
| `5e-9` | `9.9ε` | the floor | none | `ε′ ≈ 2.5e-10` |

The row `geom-core` `k_stats_doors::a_band_scaled_swing_is_decided_below_a_band_that_offers_none` holds these three as they stand. The door cannot name the tolerance: it is handed the swing as a length, not how the length was made, and no door takes a pure number from its caller to scale by the band.

## What it would take

Either of:

- the swing handed to the door as a function of the band (so the door can solve `|m| − s(ε′) ≥ K·ε′` for the tolerance it offers), which is new surface on `k_stats`; or
- `travel` offering from its own reading: where `decide_across` refuses with no offer and `|m|/(K + c)` is above the least admissible ε, the join's refusal quotes that tolerance itself.

`work/oneround/the-joins-sites-are-read-on-the-conic-so-the-travel-carries-no-swing.md` removes the swing altogether, and this row with it, if it lands first.

## Done when

A pair `travel` cannot order at this tolerance, and can at a smaller one a run admits, is told that tolerance; or `travel` carries no swing.
