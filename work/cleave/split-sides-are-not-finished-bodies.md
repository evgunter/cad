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

A flared frustum (radii ½ → 1 over height 1, revolved about y, apex at y = −1) cut through its
wall ruling at azimuth a, the normal turned t off the outward normal, at ε 1e-12. These poses
refused `Join(DegenerateSection)` on main; with the ruling pairing they answer, and one side
fails tier 3 or 3′ by escalation:

- a ∈ {0.3, 2, 3}, t = 1e-4, both normals: tier 3 `VolumeUncomputable`, an `Escalated` on
  `props_du_consistent`, margin 1.986e-12 (band 1e-12 / 1e-11), on the sliver side;
- a = 4, t = 1e-5, s = −1: tier 3′ `CensusEscalated` on `pm_census_ee_gap`, margin 1.836e-12.

The frustum with radii 1 → ½, at a ∈ {0.3, 3, 4} and t = 1e-4, refuses `Pcurves(Escalated)`
inside the split on the same margin, 1.98596694644948e-12. The escalating side passes tiers 1 and 2.
