---
id: DECIDE-8
kind: unit
title: "the apothem's sign: stated where it is decided, or read where it is not"
status: closed
opened: 2026-09-26
priority: P2
cost: H
branch: decide/8-apothem-sign
pr: 3282
refs: [the-apothems-sign-is-a-value-read, 3283]
closed: 2026-10-01
---

## What

`the-apothems-sign-is-a-value-read`: six arc-family decisions on the
parameter bulge documents stand on the apothem's sign,
`sign(1 − b²)·σ`. Route B (DECIDE-5) hands the tier `σ`, and nothing
hands it `sign(1 − b²)`.

Phase 1 looks first for a decision upstream that already separates a
minor arc from a major one. It also measures the item's narrowed read
behind a dial shipped off. If the sign is decided upstream, the sweep
states it the way route B states the turn. If only the read reaches the
six, the unit stops: two designers weigh the fork, and it goes to Ev.

Spec: `docs/DECIDE-8-SPEC.md`. Opus implementer. Review tier: set by
Phase 1 (`work/decide/log.md`, 2026-09-26).

## Closed at Phase 1 (2026-10-01)

**What Phase 1 found.** Nothing upstream decides the apothem's sign. The
six decisions are not the sweep's: they are asked in the profile's pair
pass, which recomputes an adjacent pair's shared vertex as a root of the
carriers' intersection, and then asks which root it got.

**The tier-side answer was measured and not shipped.** It is
`SymRules::signed_root_last`, behind a dial shipped off (draft #3282).
- It takes the six, and 28 on the bracket.
- It re-labels nothing.
- It misses the `0.4` pair.
- It costs +39% on the bracket's leaf.

**The fork.** Two designers weighed it and agreed: fix the six in the
profile, and don't ship the read. Ev ruled on `[ev]` #3283, yes to all
three decisions. #3282 is closed unmerged, and its branch keeps the code.

**What follows:**
- the PATHS row `an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root`;
- `the-brackets-fillet-decisions-owe-a-structural-look` (P3).
