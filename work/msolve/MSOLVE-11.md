---
id: MSOLVE-11
kind: unit
title: The solve's decisions reach a mate's own log, the lever is a finite length by construction, and a Part's own index is refused at the Part
status: dispatched
opened: 2026-09-24
priority: P1
cost: H
branch: msolve/11-escalations
---


Spec: `docs/MSOLVE-11-SPEC.md`. Gathers `plan.md` item 18: PROPS's
`mate-lane-escalations-reach-no-nodes-log` and CHROME's
`placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate`.
The whole-document solve records each decision on the log of the one
mate whose answer it decided, and each mate node splices its own
recording into its frame; the lever is formed once per mate as a
finite length by construction, so `coset::parallel`'s hand-minted
escalation has no input left to reach it; a `Part`'s own index
refuses at the `Part`. Review tier: single, full (the frame
attribution is a correctness claim, not a reading). Dispatches from
main after MSOLVE-9 merges; both units rewrite `mate/solve.rs`.

Riders (plan item 20, 2026-10-01): AUTH's `materole-has-no-display`,
CHROME's `msolve-refusals-short-of-the-shape-guard` and
`mate-refusals-name-documents-by-hex-id`, and
`lever-refusal-respells-reach-refusal`, which the hex-id row asks to
be done with it. Spec §4–§7. Dispatched 2026-10-01 on Opus.
