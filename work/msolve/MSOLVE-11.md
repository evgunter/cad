---
id: MSOLVE-11
kind: unit
title: The solve's decisions reach a mate's own log, the lever is a finite length by construction, and a Part's own index is refused at the Part
status: closed
opened: 2026-09-24
priority: P1
cost: H
branch: msolve/11-escalations
pr: 3680
closed: 2026-10-01
---


Spec: `docs/MSOLVE-11-SPEC.md` (deleted at merge; `docs/doc-ledger/msolve-11-spec.md`). Gathers `plan.md` item 18: PROPS's
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

## Closed

Merged on PR 3680 (2026-10-01) after a single full review on the
frozen head `fff779e03`: APPROVE-WITH-FIXES, no MAJOR. Claims C1, C2,
C3, C5 and C7 held. C4 and C6 failed on edges, and the fix pass closed
them. The fix pass's twelve rulings (R1–R12) are in `work/msolve/log.md`.
Each decision now lands on one mate's log (A1–A3). The lever is
`coset::Arm`, minted only by `Arm::of` (A4). A `Part`'s index refuses
at the `Part` (A5). All four riders closed (A7).

