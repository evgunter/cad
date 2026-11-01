---
id: SYM-16
kind: unit
title: the ignored receipt rows drifted red: attribute the drift, re-take, and give them a schedule
status: closed
opened: 2026-10-02
priority: P1
cost: M
branch: sym/16-receipt-drift
pr: 4155
refs: [ignored-sym-receipt-rows-drifted-red-on-main-unattributed]
closed: 2026-10-06
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

Spec: `docs/SYM-16-SPEC.md`, deleted at close (`docs/doc-ledger/sym-16-spec.md`). Opus implementer. Review tier: single FULL
review (`work/sym/log.md`, 2026-10-02).

## Closed (2026-10-06, PR #4155)

Merged after a single FULL review (NOT-MERGEABLE-AS-IS: #3774 had
re-taken both rows on `main`), a fix pass by a second implementer (the
first was lost to a container restart after opening the PR) and a delta
review (MERGEABLE). Every move since SYM-11 is attributed and judged in
the item; #3257, #3270 and #3774 among them are right. Both rows are off
`#[ignore]`: `m10_9_the_pad_at_both_rule_f_dials` on the slow set
(64 s hosted), `sym11_the_exact_channel_never_contradicts_past_the_ceiling`
on the per-PR fast set (0.86 s hosted since #3774). Filed:
`work/rules/the-pads-frozen-set-moves-with-the-documents-id-mint`,
`work/sym/the-past-the-ceiling-row-replays-only-validation-on-the-bracket-and-pad`.
