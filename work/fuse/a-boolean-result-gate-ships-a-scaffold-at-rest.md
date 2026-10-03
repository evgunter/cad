---
id: a-boolean-result-gate-ships-a-scaffold-at-rest
kind: issue
title: The boolean's result gate runs tiers 1 and 2 only, so a body carrying a scaffold at rest ships as tier-3 currency
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-03
pr: 3913
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
JOIN-1's fix pass first restated the scaffold, which only hid it: the
body then passed every tier and was still no legal operand (two
same-sense coplanar neighbours). Fix pass 2 removed the restatement.
That pose is an undeclared continuation, which the reduction now
refuses (`UndeclaredCoincidence`, REACH's scan), and declared, the pair
is merged. The gate would
still pass the next cause of a scaffold at rest the same way.

## The fix

Run the at-rest check the contract names before a `BooleanBody` is
returned: `validate_pseudomanifold(&body, &contacts)` (which is tier 3
on an empty-contacts body), or at least the cheap structural part of it
that `ScaffoldAtRest` belongs to, and refuse `ResultInvalid` with its
errors. Measure the cost on the corpus first: tier 3 re-certifies every
edge, which the result's own certificate pass may already pay for.

## Closed (FUSE, PR 3913, 2026-10-03)

The result gate now runs tier 3's transience fence after tiers 1 and 2,
and refuses a scaffold at rest as `ResultInvalid`. All four
`BooleanResult::Body` sites route through it, and there is no fifth. The
fence was chosen by measurement: it costs under 1 % of tiers 1–2. Full
tier 3 and 3′ at the door wait on REACH's
`boolean-door-tier-3-waits-on-the-description-gap` and #3870, and their
cost and refusal classes are recorded there. `is_scaffold` has one home,
`geom_brep::EdgeDescription::is_scaffold`. Residue:
`a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`
and `a-two-pinch-union-ships-a-pinch-its-records-do-not-declare` (both
FUSE), and CONTACT's seam-description row (P1, with re-pin wording).
Review tier: single FULL, with one fix pass.
