---
id: a-mate-on-a-pinned-copy-refuses
kind: issue
title: D10 stage 3 PR F: a constraint any of whose equations the bundle already fixes refuses as an overconstraint, pinned or not, by subgroup algebra; Contradictory and the membership re-measures retire
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [a-mate-relates-two-poses]
refs: [intent-stage3-is-built, mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion]
---

INTENT stage 3, PR F. Spec: `docs/INTENT-STAGE3-SPEC.md` §7. Built on FORK-S3O (fork log row 96, PR 4325), with FORK-S3M's values on equal footing (row 97, PR 4326).

A constraint (a mate or a value) any of whose equations the bundle's others already fix refuses `Overconstrained { placement, constraint, held }`, pinned or not. The test is subgroup algebra alone: admitted only when codim(held) + codim(added) = codim(result), read off the table's families, nothing measured and nothing symbolic. A pinned copy is the case `(Trivial, _)`. The door folds the held subgroup from the bundle's definitions, and the solve refuses the same for a state the door did not see.

Retired: `Contradictory`, `member_of`, `trivial_member`, `coset::intersect`'s membership re-measure and the table's measured case splits. A configuration degenerate at its values refuses as its construction does (`Through`, `Meet`).

Where two copies meet beyond what their mates fix is a contact. The census records it at stage 4's door, the `unproven-coincidence` lint proves or reports it, and its recourse is an assertion.

The migration restates each corpus bundle that overconstrains as one mate plus values, and drops each mate that fixes nothing new. The report names each. When this lands, A11 (1) reads: "a placement's mates fold by subgroup algebra, measuring nothing, to DETERMINED or UNDER, or refuse OVERCONSTRAINED (a constraint any of whose equations the bundle already fixes, named)".

Closes `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion` (with A11 (4)), `a-box-over-a-solved-clocking-widens-thirty-thousandfold` and `an-identically-zero-margin-escalates-at-a-fine-eps`. Re-reads `a-far-meeting-point-fails-membership-by-its-own-rounding` and `a-box-independent-mate-fault-bisects-the-whole-leaf-budget`.

Waits on C (`a-mate-relates-two-poses`), whose table F reads by pose kind. It also waits on stage 4 C (`carriers-compare-in-canonical-form`). That rung, with stage 4 D's (`the-door-s-third-rung-is-the-symbolic-tier`, merged), proves the contacts F's refusals leave behind.

## C's interim F lifts (INTENT stage 3 C, ruling C-3)

Until F, the refusals that need the bundle's residual are evaluation
faults: C's door (`crates/editor-core/src/mate/solve.rs`'s
`admit_mate` and `admit_value`) decides only what the recipe shows,
and the placement refuses `Overconstrained`, `Contradictory` and a
value whose freedom the bundle's other constraints fix
(`NotAFreedom` with no kind, `mate/value.rs`'s `settle`) where it
evaluates. F's measure-free door lifts it: each moves to the door.

## Two B interims F closes (INTENT stage 3 B's review, PR 5232, 2026-10-17)

Both reviewers of B raised these, and F's rule ends both. Each is
recorded here so that F's rows pin the close.

- **The door's verdict depends on the order constraints arrive in.**
  B's door checks each constraint against the bundle already held
  (`bundle[..k]`, `crates/editor-core/src/edit.rs`'s
  `admit_written_constraint`; `mate/solve.rs`'s `admit_constraint`).
  - A frame coincidence after a coaxial is admitted, and the solve then
    refuses `Overconstrained`.
  - The same two in the other order refuse at the door.
  - This is the spec's B letter (the door checks "already `Trivial`"). F's
    "any of whose equations the bundle already fixes" decides the
    constraint whatever order it arrives in.
- **A copy's space turns on a value.**
  - B's `solve_placement` (`mate/solve.rs`) makes a copy loose when the
    fold's subgroup is not determined. The fold decides its case splits
    over measured values, so two coaxial world mates are world-pinned
    at a tilt θ = 0.5 and loose at θ = 0.
  - At θ = 0 the copy leaves the product with no refusal; on main,
    `MateFault::Under` refused it.
  - Spec §1 decides kinds from the recipe. F's codimension rule decides
    determination by codimension alone, which ends this.
