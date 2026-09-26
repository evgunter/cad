---
id: DECIDE-7
kind: unit
title: rule G's leaf cost: where inside the canonical root the time goes, and the part that can go with every decision unchanged
status: closed
opened: 2026-09-25
priority: P2
cost: D
branch: decide/7-rule-g-cost
pr: 3246
refs: [rule-g-is-the-link-and-pads-leaf-cost]
closed: 2026-09-26
---

## What

`rule-g-is-the-link-and-pads-leaf-cost`. With rule G shut, the pad's
release leaf goes 73.8 → 17.2 s and the link's 8.5 → 3.8 s. DECIDE-6's
review ruled out the exact quotient and A0's folds as the cause.

This unit clocks inside rule G: the mint site's branches, `poly_sqrt`,
the side condition, the magnitude door, and the form sizes it leaves
downstream. Then it takes the cheapest answer that leaves every decision
unchanged. If every answer changes a decision, it stops for Ev's call,
since rule G's design is Ev's ruling on #2970.

Spec: `docs/DECIDE-7-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).

## Closed (2026-09-26)

Merged as #3246 into `props/sign-hull` (landing head `0c20aef42`, run
36196467565).

**What Phase 1 found.**
- Rule G's mint site is 0.35% of its cost on the pad's release leaf
  (0.20 of 57.18 s).
- The cost is the per-node rule A/B reduction over the larger forms
  rule G leaves, and 45.25 s of it re-reduces an input the session had
  already reduced.
- None of the spec's mint-site answers would save more than 0.13 s.

**What ships.** `reduce_per_node`, a per-session memo of the per-node
reduction:
- keyed on the input form, the rules and the ring bound;
- exact by construction, since an entry answers only while every atom
  id its reduction missed is still absent;
- holding `Arc`s under the `REDUCTION_FORMS` cap.

No form, receipt, ledger or pin moved, with and without the ladder.
The pad's release leaf goes 74.18 → 28.30 s, the link's 8.56 → 4.78 s,
and `m10_10_pins` in dev 339 → 174 s. The pad's peak memory goes
877 → 885 MB. The feature-gated Phase 1 instrument (`RootProfile`,
the reduction's outcome tables) ships with it.

**Filed.**
- `work/sym/the-substituted-numerator-is-built-before-the-quotients-pre-bound-refuses-it`
  (P3): 10.6 s of what rule G still costs on the pad.
- `work/decide/rule-gs-magnitude-door-never-asks-rule-c` (P3).

**Review.** A single FULL review on `5ad40726b`: APPROVE-WITH-FIXES,
0 MAJOR / 2 MINOR / 6 NOTE. The fix pass A–K at `0c20aef42`:
- the memo capped and shared;
- the whole-form, ladder and gate key rows adopted;
- the mint-before-reference dependence replaced by the absent-id check.
