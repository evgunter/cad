---
id: MSOLVE-11
kind: unit
title: The solve's decisions reach a mate's own log, the lever is a finite length by construction, and a Part's own index is refused at the Part
status: open
opened: 2026-09-24
priority: P1
cost: D
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
