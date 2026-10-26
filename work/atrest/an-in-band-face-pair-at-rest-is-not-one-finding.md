---
id: an-in-band-face-pair-at-rest-is-not-one-finding
kind: issue
title: an in-band face pair at rest is not one finding: its boundary cells escalate, and a pair-naming verdict blocks the arrivals' records
status: open
opened: 2026-10-22
priority: P1
cost: M
design: true
refs: [the-at-rest-census-is-a-check, two-copies-filling-one-volume-certify-at-rest]
---


`[assertions]` decides each pair between arrivals once, at the
highest-dimensional cells that touch, and
`the-census-records-a-planar-area-contact-as-one-face-pair` owes that
"elsewhere, an in-band pair is that pair's could-not-decide finding, not
a product refusal". That unit built the confirmed half (a confirmed face
pair's boundary cells are asked nothing, so the seat's own in-band edges
no longer escalate) and the flush door's in-band finding
(`census.rs` `flush_at_rest`, a `CensusUndecidable` naming the two
faces). It did not build the rest, which needs three things decided:

- **The census side.** A planar pair whose Door 1 reads in band (two
  side walls standing off by less than the band, aligned or opposed)
  is not confirmed, so the sweeps read every meeting of its boundary
  cells and each escalates on its own (`pm_census_ef_residual`,
  `pm_census_vf_residual`, `curve_carrier_gap`, the touch analysis's
  `bool_contact_*`): about thirty pair-less `CensusEscalated` on the row
  `census_records_what_it_decides::a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair`.
  One finding for the pair, asking nothing of its cells, is the
  ratified shape.
- **The recording side.** Any pair-naming undecided verdict
  (`CensusUndecidable`) between two groups reads as "not cleared of
  interpenetrating" (`census.rs` `census_uncleared_solids`), so
  `record_at_rest` records nothing between the two arrivals, the seat's
  confirmed face pair included. An in-band pair within ε is not
  evidence of an overlap past the band; whether it blocks the arrivals'
  other records is a decision.
- **The vessel.** A could-not-decide finding that refuses nothing is
  stage 5's `AtRest` could-not-look finding
  (`work/intent/the-at-rest-census-is-a-check.md`); until it lands, the
  gate (`checks::at_rest::partition`, then `assembly::verdict`) refuses
  every census verdict that is not an interference.

So a near-flush seat, mated or not, still refuses the product today,
on the in-band walls' escalations rather than on its seat's edges.

## Triage (2026-10-26, ATREST orchestrator)

P1: one pair reported as about thirty pair-less escalations is a special case handled non-uniformly against `[assertions]`'s "once, at the highest-dimensional cells that touch". `design: true` covers the recording side: whether an undecided pair between two arrivals blocks the arrivals' other records (`census_uncleared_solids`). That question shares ground with `two-copies-filling-one-volume-certify-at-rest` (PR 5745), which re-reads which undecided verdicts are really overlaps. Weigh it after 5745 lands. The vessel is stage 5 C's could-not-look finding (`docs/INTENT-STAGE5-SPEC.md` §4, `AtRestEvidence::NoLane`).
