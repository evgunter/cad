---
id: split-sides-are-not-finished-bodies
kind: issue
title: The split's sides are plain bodies: its door takes a finished operand but hands back sides no at-rest gate read
status: open
opened: 2026-10-06
priority: P3
cost: M
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
