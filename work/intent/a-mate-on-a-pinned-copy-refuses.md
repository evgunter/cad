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
provisionally. After Ev's comments on #4325, a placement's mates fold in
order. Each mate is projected onto the residual subgroup the earlier
ones leave and fixes only that, exactly, by construction, never
checking what earlier mates fixed. The only refusal is `Overconstrained`:
a mate that fixes nothing (on a pinned copy no predicate runs). A
degenerate configuration (a secondary plane parallel to the primary)
refuses as its construction does, banded like `Meet` (FORK-1b).
`Contradictory`, its measured clash, `member_of`'s re-measure and
`trivial_member` retire. The two-peg plate with separately typed
spacings is admitted: peg 2 fixes the spin, and whether it meets hole 2
is an at-rest contact the census records and the lint proves or reports
(`s` against `t`), quieted by an `Assert` like any contact. Nothing
symbolic runs at the mate door. The migration drops only mates that fix
nothing, naming each. When it lands, A11 (1) reads: "A placement's mates
fold in order, each projected onto the residual subgroup the earlier
ones leave, to DETERMINED or UNDER, or refuse OVERCONSTRAINED (a mate
that fixes nothing; on a pinned copy no predicate runs); a degenerate
configuration refuses as its construction does." The GUI shows a
placement's mates in order (primary, secondary, tertiary).
