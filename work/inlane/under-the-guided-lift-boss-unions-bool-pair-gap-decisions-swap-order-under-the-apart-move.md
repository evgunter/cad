---
id: under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move
kind: issue
title: Under the guided lift, boss_union's apart move decides two bool_pair_gap margins in the other order (read as a laundering), then parts at pcurve_chart_azimuth_frame, decision 3996 of 11384; NOTHING PINS THIS once the row runs in step
status: open
opened: 2026-10-17
priority: P2
cost: M
---


Found by the shared replay session unit (`inlane/sym-replay-session`), guided lift, apart move.

**NOTHING PINS THIS.** The gating row `revalue_boss_union` now runs under the in-step move, which does not see it. The census below is the only reading until this is fixed and the row returns to the apart move.

- **Move:** Apart.
- **What the instrument reports:** `laundered {"bool_pair_gap"}` at decisions #3993 and #3996, then the builds part at #3996 of 11,384 (`pcurve_chart_azimuth_frame`).
- **What the numbers say:** an ORDER swap, not a constant.
  - #3993: p0 −1.3007496684315063, re-valued at p1 −1.3007497491109032, p1 build −0.5000004552002293.
  - #3996: p0 −1.5500000239999998, re-valued −1.5500017795501375, p1 build −1.3007497491109032.
  - The p1 build's #3996 is bit-for-bit #3993's re-valued margin. So the two builds decided the same `bool_pair_gap` margins in a different order, and position #3993 at p1 is another pair's gap.
  - So either the order the pairs are visited in depends on their values (a sort by gap), or a pair enumeration moved. That is a branch the instrument cannot tell from a laundering. It then parts at `pcurve_chart_azimuth_frame`.
- **Repro:** `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=boss_union cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

**Owed:**
- Find what orders `bool_pair_gap`'s decisions. If it is value-sorted, decide whether the instrument should key decisions by pair rather than position.
- Explain the `pcurve_chart_azimuth_frame` parting.
- Return `revalue_boss_union` to the apart move.
