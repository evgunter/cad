---
id: split-sides-are-not-finished-bodies
kind: issue
title: The split's sides are plain bodies: its door takes a finished operand but hands back sides no at-rest gate read
status: closed
opened: 2026-10-06
priority: P2
cost: M
closed: 2026-10-26
---



Left by `split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody` on the way in. The way out did not move: `SplitResult`'s
sides are `SplitPart<T>` over plain `Body<T>`, gated at tier 2 only
(`split_direct`'s `validate_closed`, `SplitFinishError::ResultInvalid`).
The Boolean's result is an `AtRestBody` gated by
`T::gate_at_rest_kept` (`boolean-door-adopts-the-finished-body-type`).

So a split side that a later door takes finished pays tier 3 there (the
editor's seat finishes every split side a Boolean or a second split
consumes), and the type does not say the side is finished.

Why the sides are not gated at tier 3 today, per `split`'s own doc: a
pinch side's touching pieces carry contacts the split declares nowhere.
Unmeasured: which sides `T::gate_at_rest_kept` refuses today. What closes
it: gate each side with `T::gate_at_rest_kept` and hand back
`AtRestBody`, with the contacts the split mints declared where the gate
needs them, or a measurement that says which sides fail and why the type
stays plain.

## Evidence: near-tangent sliver sides (branch `cleave/frustum-apex`)

Measured: two frusta revolved about y, radii 1 → ½ and the flared ½ → 1 (apex at y = 2 and
y = −1), each cut through its wall ruling at azimuth a ∈ {0, 0.3, 1, 2, 3, 4, 5.5}, the normal
turned t ∈ {1e-3, 1e-4, 1e-5} off the outward normal, both normals: 84 poses per ε. With the
ruling pairing, 0 answer at ε 1e-6, 28 at 1e-9 (all clean at tiers 1–3′) and 56 at 1e-12. At
1e-12 every side passes tiers 1 and 2 and every volume `mass_properties` certifies is the closed
form. Ten poses have a side that tier 3 or 3′ escalates on. On main all ten refused
`Join(DegenerateSection)`.

- Tier 3 `VolumeUncomputable` (an `Escalated` on `props_du_consistent`, margin 1.986e-12, band
  1e-12 / 1e-11) on the sliver side, where `mass_properties` refuses typed:
  - frustum 1 → ½: a = 5.5, t = 1e-4, s = ±1;
  - flared ½ → 1: a ∈ {0.3, 2, 3}, t = 1e-4, s = ±1.
- Tier 3′ `CensusEscalated` on `pm_census_ee_gap`, on both sides, whose volumes certify right:
  - flared: a = 1, t = 1e-5, s = +1, margin 1.145e-12;
  - flared: a = 4, t = 1e-5, s = −1, margin 1.836e-12.

Inside the split, the 1 → ½ frustum at a ∈ {0.3, 1, 3, 4}, t = 1e-4 refuses
`Pcurves(Escalated)` instead, at the tier 3 rows' margin (1.986e-12 where read). Which of these
poses answers moves with rounding in the pose's construction. The gate
`a_near_tangent_cut_through_a_frustum_ruling_never_answers_wrongly`
(`crates/sweep/tests/split_through_a_ruling.rs`) takes every one of these poses and asserts
what the split promises: tiers 1 and 2, and no wrong certified volume. Gating the sides at
tier 3 would turn these ten into refusals.

## Evidence: a cut at the zero-band edge of a tangency (PR 4394's review)

The rounded shoulder (`crates/sweep/tests/wedge_end_doors.rs`, `shoulder(h)`: unit quarter arc
below a 2 × 2 block, extruded `h`), cut by `y = 1 − δ`, normal `−y`. The cut is tangent to the
wall to within the zero band, so the section table answers `TangentLine` and the split mints the
seam on the cut plane, `|δ|` off the cylinder. At `δ = −ε`, or a hair inside it, the split returns
`Ok` with a side that tier 3 refuses (the `y < 1` side):

| ε | δ | h = c·√(2ε) | seam stored | tier 3 on that side |
|---|---|---|---|---|
| 1e-6 | −1.0·ε | c ∈ {0.5, 0.9} | `Chart` on the cut plane | `VolumeUncomputable` on the wall: `props_meridian_on_surface` enclosure [9.99999999947e-7, 1.0000000001e-6] |
| 1e-12 | −0.9999·ε | c ∈ {0.5, 0.9, 8} | `Chart` (c < 1), `TangentIntersection` (c = 8) | `VolumeUncomputable` on the wall: `props_loop_closed` enclosure [9.9964e-13, 1.0002e-12] |

The 1e-12, c = 8 row (the intrinsic route) does the same on main before PR 4394. The `Chart` rows
are PR 4394's: that PR routes the tangent chord through the must-carry rule, and a chart image is
metered against the plane it is charted on only, so the carrier's ε offset from the wall shows up
first at the props walk. At 1e-6 the intrinsic route (c = 8) refuses typed instead
(`Join(Euler(Certification { Escalated { TangentGap } }))`, which meters both surfaces); at
`|δ| ≥ 1.0001·ε` the reduction refuses `SliverVertex`, and every other `|δ| ≤ 0.9999·ε` row is
tier-3 valid. Gating the sides at tier 3 would turn these rows into typed refusals. The reviewer's
probe is `probe_band_edge_offsets` on branch `tang/split-chord-review-probes`
(`crates/sweep/tests/tang_split_chord_review_probes.rs`).

## Note from `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (2026-10-14)

Branch `cleave/seam-split-invalid-ok` (PR 5100) gates each side at the
scalar's at-rest gate (`AtRestPolicy::gate_at_rest`), typed as the Boolean
types it (`SplitFinishError::ResultNotAtRest`, `ResultEscalated`). So at a
certifying scalar a split no longer returns a side tier 3 refuses.

What was measured there: the seam-vertex witness family turns from invalid
`Ok`s into `ResultNotAtRest`, with 30 such poses at 1e-9 and 21 at 1e-12. The
frustum poses above were not re-measured as refusals: PR 5100's review ran
the gate over `split_through_a_ruling` (90 gate calls) and found no invalid
`Ok` among them; why they no longer reach one was not measured.
`a_near_tangent_cut_through_a_frustum_ruling_never_answers_wrongly` now
asserts tier 3 on every side it gets back. The shoulder rows were not
re-measured.

**What this row keeps, and why it is now P2: the double pay.** The sides are
still plain `Body` values, so the verdict the split just derived is dropped.
Every consumer pays tier 3 again: `editor-core`'s `finished_operand`
(`eval/wire.rs`) re-gates every side a later node consumes, measured by PR
5100's review at 0.15–0.39 s per side on `revalue_cut_cylinder`. Handing the
sides back as `AtRestBody`, so the verdict rides with them, is the only
cheaper gate that stays honest.

## Closed (2026-10-26)

Fixed by PR 5590 (FLUX F1, merge `84ce01a580`): `SplitPart::Body` holds an `AtRestBody` the split's own gate reached (`T::gate_at_rest_kept` in `splitting::split`), as a Boolean's result does. The tier-3′ half of the bar is `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`. (CLEAVE slate triage)
