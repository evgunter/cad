---
id: a-mate-on-a-pinned-copy-refuses
kind: issue
title: D10 stage 3 PR F: a mate that lowers no dimension of its bundle's fold refuses as an overconstraint, decided by subgroup algebra; the interim verify path retires
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [a-placement-is-the-bundle-of-mates, mates-declare-no-contact]
refs: [intent-stage3-is-built, mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion]
needs_ev: true
---

INTENT stage 3, PR F. Spec: `docs/INTENT-STAGE3-SPEC.md` §7.

A mate that lowers no dimension of its bundle's fold refuses `Overconstrained` at the insert door and in the solve (FORK-S3-4). A mate on a pinned copy is the measure-free case, `(Trivial, _)`. `Contradictory` stays for a lowering mate whose cosets do not meet. C's interim verify-and-mint path and `coset::intersect`'s redundant re-measure retire, and a one-time migration drops the corpus mates that overconstrain, naming each.

The unit waits on stage 4's retirement of A5's hard error on an unattributed contact. Until then, refusing a would-be declaring mate would leave its contact undeclared. That unit is stage 4's I, `mates-declare-no-contact`, which itself waits on this stage's C through stage 4's H: the order is C → H → I → F (orchestrator's ruling on PR 4316).

Closes `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion` (with A11 (4)), `a-box-over-a-solved-clocking-widens-thirty-thousandfold` and `an-identically-zero-margin-escalates-at-a-fine-eps`. Re-reads `a-far-meeting-point-fails-membership-by-its-own-rounding` and `a-box-independent-mate-fault-bisects-the-whole-leaf-budget`.

FORK-S3-4 was weighed with FORK-S4-5 as FORK-S3O (fork log row 96) and
went to Ev in an `[ev]` PR; this unit builds on the answer
provisionally, and it widens the unit. A mate refuses `Overconstrained`
when its subgroup contains the fold of the bundle's other mates
(leave-one-out, so order-independent), or when an equation the other
mates already fix (codim(held) + codim(added) − codim(result) of them)
is not proven structural by the coincidence door. The two-peg plate with
separately typed spacings refuses; its recourse is one `Frame` mate
between `Through { peg-1 axis, peg-2 centre }` and `Through { hole-1
axis, hole-2 centre }` plus an `Assert` on the clearance. The table's
case splits (parallel, perpendicular) are chosen structurally; at a
degenerate value a structurally generic branch refuses `Degenerate`.
`Contradictory`, its measured clash, `member_of`'s re-measure and
`trivial_member` retire; nothing in the fold is decided from a value, so
nothing is recorded. This unit now needs stage 4's merged C+D unit
(the door's rungs) before it. The migration drops every value-only
excess mate, naming each: measure the corpus count first. When it lands,
A11 (1)'s outcomes read: "Mates fold by coset intersection whose
branches are chosen structurally and whose representative is
constructed, never checked, to PINNED (`Trivial`), UNDER (the residual
subgroup named), OVERCONSTRAINED (a mate that fixes nothing new, or
whose already-fixed equation the door does not prove, named with the
door's residual as recourse) or DEGENERATE (a structurally generic
branch at a value where its construction fails). A case split in the
sliver band escalates `Indeterminate`." Spec test 20 becomes
`Overconstrained`.
