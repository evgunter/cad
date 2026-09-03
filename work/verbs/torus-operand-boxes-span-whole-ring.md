---
id: torus-operand-boxes-span-whole-ring
kind: issue
title: Torus operand boxes span the whole ring: tighten to the trimmed arc (a boundary-tight box retires lily wall 1 with no declaration at all)
status: open
opened: 2026-09-01
github: 1488
refs: [1477, 968]
---

## From GitHub issue 1488

opened 2026-09-01, 0 comments.

Filed at MATE-7a's adjudication (PR #1477) as the scheduled home for a residue both review arms flagged as disclosed-but-unscheduled (deviation 2's "not attempted here").

**The artifact.** A trimmed torus face's operand box is the WHOLE ring's box (a 22° arc of the lily stem's 5 m ring boxed as the full 10 m ring) — the same per-kind box artifact VERBS-GATE fixed for the cone and deferred for the torus. MATE-7a measured the consequence precisely: lily wall 1's gate refusal names the stem's tube wall against the arch's FAR cap, whose exact loci are **2.08 m apart**; the far cap's centre sits inside the whole-torus box but more than 1.5 m clear of a boundary-tight one.

**The strong motivation (R2's measurement, going beyond the PR):** the minimum true-locus separation over ALL of lily wall 1's offending cross pairs is 0.008 m (the 0.060 − 0.052 annular gap at the weld plane) — so a boundary-tight torus box retires wall 1 **with no declaration machinery involved at all**. The declared-cover admission MATE-7a built is near-inert on today's whole-ring boxes (its own disclosed deviation 2); the box fix is what makes the lily stem glue.

**Soundness burden** (stated in PR #1477's body): the cylinder's `clip_to_boundary` argument does not port to the torus — the arc's box in the ring plane needs its own enclosure argument. That argument is the unit.

Context: issue 968 (the durable record of lily wall 1) carries the corrected diagnosis.

## Home

`work/verbs/` — the per-kind operand box is VERBS-GATE's ground (VERBS unit 6, the operand gate per face kind), which fixed the cone case and deferred the torus one.
