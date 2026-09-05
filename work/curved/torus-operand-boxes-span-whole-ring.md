---
id: torus-operand-boxes-span-whole-ring
kind: unit
title: Torus operand boxes span the whole ring: tighten to the trimmed arc (a boundary-tight box retires lily wall 1 with no declaration at all)
status: review
opened: 2026-09-01
github: 1488
refs: [1477, 968]
branch: curved/torus-box
pr: 1907
---

## From GitHub issue 1488

Opened 2026-09-01; 0 comments.

Filed at MATE-7a's adjudication (PR #1477) as the scheduled home for a residue both review arms flagged as disclosed-but-unscheduled (deviation 2's "not attempted here").

**The artifact.** A trimmed torus face's operand box is the WHOLE ring's box (a 22° arc of the lily stem's 5 m ring boxed as the full 10 m ring) — the same per-kind box artifact VERBS-GATE fixed for the cone and deferred for the torus. MATE-7a measured the consequence precisely: lily wall 1's gate refusal names the stem's tube wall against the arch's FAR cap, whose exact loci are **2.08 m apart**; the far cap's centre sits inside the whole-torus box but more than 1.5 m clear of a boundary-tight one.

**The strong motivation (R2's measurement, going beyond the PR):** the minimum true-locus separation over ALL of lily wall 1's offending cross pairs is 0.008 m (the 0.060 − 0.052 annular gap at the weld plane) — so a boundary-tight torus box retires wall 1 **with no declaration machinery involved at all**. The declared-cover admission MATE-7a built is near-inert on today's whole-ring boxes (its own disclosed deviation 2); the box fix is what makes the lily stem glue.

**Soundness burden** (stated in PR #1477's body): the cylinder's `clip_to_boundary` argument does not port to the torus — the arc's box in the ring plane needs its own enclosure argument. That argument is the unit.

Context: issue 968 (the durable record of lily wall 1) carries the corrected diagnosis.

## Home

`work/verbs/` — the per-kind operand box is VERBS-GATE's ground (VERBS unit 6, the operand gate per face kind), which fixed the cone case and deferred the torus one.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

## Closed — what the box buys, and what it does not

**The motivation above is refuted.** MATE-7a's R2 inference — "the
minimum true-locus separation over all of lily wall 1's offending
cross pairs is 0.008 m, so a boundary-tight torus box retires wall 1
with no declaration machinery at all" — conflates LOCUS separation
with BOX separation. `docs/CURVED-TORUS-SPEC.md` §R3 carries the
geometry: the arch's start cap is a disc of radius 0.052 concentric
and coplanar with the stem tube's end circle of radius 0.060, so every
AABB containing that circle contains the disc. The stem's torus wall
box therefore overlaps the arch's start cap under ANY sound box, and
the same holds for the arch's wall box against the stem's end cap and
for the two walls against each other. `Torus` is not on
`boolean_arm_exists`, so `first_unsupported_pair` refuses on the first
overlapping pair in arena order whatever the boxes are.

**What this unit delivered is a RE-AIM, per the CURVED orchestrator's
ruling 1 at the spec's ratification.** Lily wall 1 no longer names the
arch's FAR cap 2.08 m away — a pure box artifact — and names a weld
pair at 0.008 m true separation instead; the refusal is now about
geometry that is genuinely close. What the tightening buys beyond that
is the sweep tree, `separation`, `ops` and the census, none of which
gates on the kind.

**Wall 1's retirement needs `Torus` on `boolean_arm_exists`**, which is
a gate-policy change and is only honest once the crossing layer has
torus arms. That is `work/curved/torus-operand-gate-admission.md`,
after `work/curved/circle-residual-harmonics-needs-torus-arm.md`.
