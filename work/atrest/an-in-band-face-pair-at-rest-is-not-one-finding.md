---
id: an-in-band-face-pair-at-rest-is-not-one-finding
kind: issue
title: an in-band face pair at rest is not one finding: its boundary cells escalate, and a pair-naming verdict blocks the arrivals' records
status: closed
opened: 2026-10-22
priority: P1
cost: M
branch: atrest/in-band-face-pair-one-finding
closed: 2026-10-26
refs: [the-at-rest-census-is-a-check, two-copies-filling-one-volume-certify-at-rest]
pr: 5782
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

## Decided (2026-10-26, ATREST orchestrator)

The recording side follows from ratified text, so it needs no designers. `[assertions]` decides each pair between arrivals "once, at the highest-dimensional cells that touch". A pair the census has no lane for "is a finding too, saying it could not look". D1 says carried records are never re-derived.

- **An in-band verdict on one face pair** (Door 1 `Undecided`/`Escalated` on a planar pair, or the flush door's face-pair `CensusUndecidable`) is that pair's one could-not-look finding. It names the two faces, and nothing of its boundary cells is asked. It does **not** block the records of the other pairs between the same two arrivals. Blocking them would leave a confirmed seat unrecorded, and a later census over the result would decide that seat again.
  - An **opposed** in-band face pair within ε is not evidence of an overlap past the band. Its two materials lie on either side, so any overlap it hides is thinner than the band, which the representation cannot tell from contact.
- **Corrected (2026-10-26, ATREST orchestrator, on PR 5782's review):** the premise above holds for opposed pairs only. An **aligned** pair has both materials behind both faces, so the area the two share decides.
  - Door 1 `Undecided` on an opposed pair: `ThinInBand`.
  - Door 1 `Undecided` on an aligned pair, area decided positive: `SameSideInterference`.
  - Door 1 `Undecided` on an aligned pair, area decided empty or thinner than the band: `ThinInBand`.
  - Door 1 `Undecided` on an aligned pair, area undecided: `MayOverlap`, which blocks the records.
  - Door 1 `Escalated`, which reads no relation: `MayOverlap`.
  - Only a pair's escalations become its finding; a decided finding of its cells stands.
- **A solid-level "cannot clear" verdict still blocks the pair of arrivals' records**: arm 2's mixed-touch undecidable ("one passes into the other where they touch"), or an undecided crossing side. That verdict is evidence the solids may interpenetrate past the band.
- **`census_uncleared_solids`** therefore reads the solid-level verdicts and not the pair-naming in-band one. If `CensusUndecidable` does not distinguish the two today, it gains a typed reason that does. No bool, and no matching on rendered text.

**Rows:**
- `census_records_what_it_decides::a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair` reports one could-not-look finding for the near-flush wall pair, with no pair-less escalations. The seat's face pair is recorded.
- The near-flush mated seat no longer refuses the product on its walls' escalations once stage 5 C retires the gate. Until then it refuses on one finding, not thirty.

## Closed

2026-10-26 (in-band lane): built as decided.

- **The census.** `planar_face_pairs` reads Door 1's `Undecided` or
  `Escalated` on two planar faces of different solids as an in-band pair.
  `Declared::absorb` turns what the sweeps find of a touch between the
  pair's cells into the pair's one `CensusUndecidable`, naming the two
  faces, with `reason: Undecidable::ThinInBand`. An escalation of two
  other cells whose closures hold such a touch is held, and is settled
  after every pass (`Declared::settle`). Arm 2's material test does not
  refuse on a vertex of such a touch.
- **The recording side.** `census_uncleared_solids` and arm 2's `blocks`
  read only `Undecidable::MayOverlap`, so the seat's face pair is
  recorded beside the in-band finding.
- **The flush door** leaves Door 1's in-band pairs to the census.
- **Corrected on review.** An aligned in-band pair reads its area. The region door's carrier gate refuses a pair standing off in band, so `thin_overlap` reads the case where the two share no area thicker than the band (`ThinInBand`); the rest are `MayOverlap`. `Escalated` is `MayOverlap`. Only escalations are absorbed. Shadows are dropped by where they lie (`drop_shadows`). `an_overlap_whose_aligned_faces_are_in_band_is_never_cleared` pins the reviewer's three overlap poses.
- **Rows.** `census_records_what_it_decides::a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair`
  reports one finding, records the seat, and confirms.
  `asm_r2b_assembly::a_near_flush_seat_refuses_on_its_one_in_band_pair`
  pins the gate refusing on that one finding until stage 5 C.

The sweep turned up one hit outside the fence:
`an-in-band-pair-rides-an-interference-as-its-overlap`
(`checks/at_rest.rs`, INTENT D′3's).

