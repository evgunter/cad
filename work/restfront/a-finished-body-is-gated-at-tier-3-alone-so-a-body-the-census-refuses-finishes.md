---
id: a-finished-body-is-gated-at-tier-3-alone-so-a-body-the-census-refuses-finishes
kind: issue
title: A finished body is gated at tier 3 alone, so a body the census refuses finishes and reaches every door as an operand
status: open
opened: 2026-10-21
refs: [boolean-door-runs-the-census-over-its-result, a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass]
priority: P2
cost: H
---


Found by FLUSH's fin-crown unit (PR 5534) and its review. Filed here
because the gate is `validate.rs`'s and `props.rs`'s at-rest policy.

## The contract

`crates/topo/README.md` (tier 3, "A **finished body** (`AtRestBody`)
passes tier 3′ against its own contact records: none, for a body that
touches nothing, where the census must find no coincidence at all.
Every door that returns or consumes one pays that gate once, at the
door that built it."). REACHHOLD's
`boolean-door-runs-the-census-over-its-result` reads the same bar from
`docs/DESIGN.md` for the boolean's result.

## What the code does

- `AtRestBody::validate` (`validate.rs`, near line 9811) is "the one
  constructor of `AtRestOutcome::Validated`", and it runs
  `validate_geometric`: tier 3 alone, with no census.
- `AtRestPolicy::gate_at_rest_kept` at `f64` (`props.rs`, near line 4795)
  is `AtRestBody::validate`, and so is every finisher built on it
  (`test_support::finished`, the fixtures' `*_kept`).
- The boolean door reads an operand through `reduce::gate_operand`
  (scaffolding and edge carriers) and, only for an operand with no
  verdict, `reduce::gate_unverdicted_operand` (`ops.rs`, near line 761):
  tier 2, check 7 and check 10, and check 11. Its doc says a `Validated`
  operand's tier-3 verdict covers orientation. No census runs on an
  operand.
- Tier 3′ runs at the door only through
  `AtRestPolicy::gate_at_rest_declared`, which editor-core calls over a
  product's aggregate (`editor-core/src/product.rs`, near line 1274). No
  topo door calls it on its own operands or results.

So a body the census refuses can still finish, and reach every door as
an operand.

## Witnesses

- **The fin crown on its old bottom.**
  `meeting::posed_crown_kept(&fin(), [0.0, 0.2, -0.6], pose, tol)` finishes
  at every pose, yet `validate_pseudomanifold` with no contacts refuses
  it with two `EdgeFacePierce`. Its faces cross; the exact test is in
  `crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`. SECTOR's
  `a-thin-fin-voids-boolean-refuses-tier-3-prime-with-an-edge-face-pierce`
  was this body's pierces seen through a boolean result.

FLUX's seam-region witness B (the 1e-12 dimple's flip) was a second
witness: the census refused it `CensusEscalated` while the gate passed
it. Since PR 5516, check 6 refuses it at tier 3
(`SeamRegionOvercovered`), so the old crown is the witness left. It is a
fixture, and no verb is known to build a body the census refuses that
tier 3 passes.

## Proposed

- **A defect against the stated contract, kept at P2** because no verb
  is known to reach it. It rises to P1 on the first verb-built witness.
- **The cost is H.** Running the census in `AtRestBody::validate` will
  refuse fixtures and results that finish today, as REACHHOLD measured
  at the door (175 results), and it charges every finish a census.
- The question that may need a ruling is whether "finished" should mean
  tier 3′ at the constructor, or whether the README should say tier 3,
  with tier 3′ at the doors that declare contacts.
