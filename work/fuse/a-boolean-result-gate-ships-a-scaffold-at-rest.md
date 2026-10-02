---
id: a-boolean-result-gate-ships-a-scaffold-at-rest
kind: issue
title: The boolean's result gate runs tiers 1 and 2 only, so a body carrying a scaffold at rest ships as tier-3 currency
status: open
opened: 2026-10-02
priority: P1
cost: M
---


(JOIN-1 dual review, PR 3790: both reviewers.)

## What

`crates/topo/src/boolean/ops.rs` `gate` (~2210) is the boolean's
result gate: it runs `validate` and `validate_closed` (tiers 1 and 2)
and nothing else. `BooleanBody`'s own doc (`ops.rs` ~146-159) says an
empty-contacts result "remains ordinary tier-3 currency
(`validate_geometric`)" and a non-empty one is tier-3′ currency. Nothing
on the way out checks either claim.

The review measured what that lets through. On JOIN-1's reviewed head
(4ef105c30) an undeclared ∪ of a hexagonal prism and a box sharing part
of a vertical corner edge, with wedges summing to 180°, returned a body
of the right volume carrying a minted chord whose description was
still a scaffold (`ScaffoldAtRest`): `validate_pseudomanifold` and
`validate_geometric_certificate` refuse it, the gate passed it. R1
counted 288 such poses among the hex-prism ∪ box battery, R2 found the
same in random z-prism pairs (`join1_r2_rand::r2_rand_seed2_case390`).
JOIN-1's fix pass closes that cause (the boolean's description pass
now restates a scaffold at rest, as the split finish already did), but
the gate would pass the next one the same way.

## The fix

Run the at-rest check the contract names before a `BooleanBody` is
returned: `validate_pseudomanifold(&body, &contacts)` (which is tier 3
on an empty-contacts body), or at least the cheap structural part of it
that `ScaffoldAtRest` belongs to, and refuse `ResultInvalid` with its
errors. Measure the cost on the corpus first: tier 3 re-certifies every
edge, which the result's own certificate pass may already pay for.
