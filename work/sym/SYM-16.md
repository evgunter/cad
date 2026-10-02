---
id: SYM-16
kind: unit
title: "the ignored receipt rows drifted red: attribute the drift, re-take, and give them a schedule"
status: dispatched
opened: 2026-10-02
priority: P1
cost: M
branch: sym/16-receipt-drift
refs: [ignored-sym-receipt-rows-drifted-red-on-main-unattributed]
---

## What

`ignored-sym-receipt-rows-drifted-red-on-main-unattributed`: the pad's,
the link's and the bracket's exact receipts in two `#[ignore]`d rows
drifted (`numeric` +8 and `frozen` 2750 → 2722 on the pad), and no
commit is credited with it.

Phase 1 re-takes both rows at `main`, bisects the window from SYM-11
Phase 1 (`03ac24d8ba`) to `8ee3daf171`, and says whether each moved
decision is right. Phase 2 re-takes the tables and moves the rows onto
the gate's slow set, so a drift cannot land unseen again.

Spec: `docs/SYM-16-SPEC.md`. Opus implementer. Review tier: single FULL
review (`work/sym/log.md`, 2026-10-02).
