---
id: the-indeterminate-seal
kind: unit
title: only the funnel builds an Indeterminate: non_exhaustive, MarginDiag::INVALID crate-private, a test-support constructor, invalid_margin.rs deleted (mints step 5)
status: parked
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
blocked_on: [split-sector-rules-mint-no-invalid, split-straddles-and-disagreements-carry-their-reading, point-in-solid-mints-outside-its-period-and-nappe-rows, boolean-gate-mints-route-through-the-funnel, glue-disagreements-and-held-contradictions-after-stage-4, period-headroom-margin-has-no-shared-home, cone-nappe-is-decided-in-five-places, chart-region-mints-indeterminates-after-a-definite-sign, chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value, boolean-unreadable-norm-ends-as-a-kernel-defect, ring-contact-and-sliver-split-endings-want-their-decisions, contact-gate-readers-drop-the-arm-verdict-or-mint-invalid]
---

Unit 6 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`). This is design step 5 and design
item 1.

## What

- `geom_core::Indeterminate` becomes `#[non_exhaustive]`. Its fields
  stay readable.
- `MarginDiag::INVALID` becomes crate-private.
- Fixtures move to a `test-support` constructor. On 2026-10-10 that
  covers 136 test literals and 85 `INVALID` spellings outside
  `geom-core`.
- `crates/topo/src/invalid_margin.rs` is deleted.
- `scripts/gates/reporting-margin-door.sh`'s `terminal_sliver: true`
  literal pin becomes a compile error. Its prose says so.

## Why it waits on every other row

Both attributes are crate-wide in `geom-core`, so the seal breaks every
remaining production hand mint in every crate at once. It cannot land
in parts. The remaining hand mints are:
- CLEAVE's units 1–5;
- PRED's four (`period-headroom-margin-has-no-shared-home`,
  `cone-nappe-is-decided-in-five-places`);
- CHART's four (`chart-region-mints-indeterminates-after-a-definite-sign`,
  `chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value`);
- TOPO's `boolean-unreadable-norm-ends-as-a-kernel-defect`;
- RESTFRONT's `ring-contact-and-sliver-split-endings-want-their-decisions`;
- SECTOR's `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`;
- the 35 D10-held arms, through unit 5.

A blocker row may close without retiring its mint. Re-run the
re-scope's two passes when the last blocker closes. The ratchet gate
(`hand-minted-indeterminate-ratchet-gate`) is what that re-run should
find at zero.
