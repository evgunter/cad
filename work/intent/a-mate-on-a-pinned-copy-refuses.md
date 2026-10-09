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
---

INTENT stage 3, PR F. Spec: `docs/INTENT-STAGE3-SPEC.md` §7.

A mate that lowers no dimension of its bundle's fold refuses `Overconstrained` at the insert door and in the solve (FORK-S3-4). A mate on a pinned copy is the measure-free case, `(Trivial, _)`. `Contradictory` stays for a lowering mate whose cosets do not meet. C's interim verify-and-mint path and `coset::intersect`'s redundant re-measure retire, and a one-time migration drops the corpus mates that overconstrain, naming each.

The unit waits on stage 4's retirement of A5's hard error on an unattributed contact. Until then, refusing a would-be declaring mate would leave its contact undeclared. That unit is stage 4's I, `mates-declare-no-contact`, which itself waits on this stage's C through stage 4's H: the order is C → H → I → F (orchestrator's ruling on PR 4316).

Closes `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion` (with A11 (4)), `a-box-over-a-solved-clocking-widens-thirty-thousandfold` and `an-identically-zero-margin-escalates-at-a-fine-eps`. Re-reads `a-far-meeting-point-fails-membership-by-its-own-rounding` and `a-box-independent-mate-fault-bisects-the-whole-leaf-budget`.

FORK-S3-4 was weighed with FORK-S4-5 as FORK-S3O (fork log row 96) and is
with Ev in an `[ev]` PR. Corrected against Ev's 2026-10-03 words
(`5f7a1c71e3`): no cleverness, uniformly. A mate is admitted only when
every one of its equations fixes something the bundle's other mates
left free (codim(held) + codim(added) = codim(result)). Otherwise it
refuses `Overconstrained`, pinned or not, by subgroup algebra alone, with
nothing measured and nothing symbolic. In practice a pin is one `Frame`
mate between two constructed frames (`FaceFrame` plus `Offset`,
`Through { axis, point }`, `Meet`). A `Plane` or `Axis` mate alone leaves
the copy `Under`. The two-peg plate is
`Through { peg-1 axis, peg-2 centre } = Through { hole-1 axis, hole-2 centre }`.
Peg 2 in hole 2 is then a contact the census records and the lint proves
or reports, and the recourse is an assertion. `Contradictory`, the
table's measured case splits, `member_of` and `trivial_member` retire. A
configuration degenerate at its values refuses as its construction does
(`Through`/`Meet`, FORK-1b). The migration rewrites each multi-mate
bundle (the MSOLVE fixtures) as one constructed `Frame` mate, naming each.
When this lands, A11 (1) reads: "a placement's mates fold by subgroup
algebra, measuring nothing, to DETERMINED or UNDER, or refuse
OVERCONSTRAINED (a mate any of whose equations the bundle already fixes,
named)".
