---
id: ignored-sym-receipt-rows-drifted-red-on-main-unattributed
kind: issue
title: The ignored SYM receipt rows (pad's rule-F differential, SYM-11 past the ceiling) were red on main with the drift unattributed
status: open
opened: 2026-09-26
---

**Found by ENCL's must-carry-gate re-baseline, and pre-existing.** Two
`#[ignore]`d evidence rows pin the pad's receipts exactly, and both were
red at `8ee3daf171` (main, before the must-carry first-order gate):

- `editor-core/tests/sym11_exact_channel_rows.rs`, `PAST_THE_CEILING`
  (read by `sym11_the_exact_channel_never_contradicts_past_the_ceiling`),
  measured at `8ee3daf171`, ε = 1e-6, `[symbolic_zero, registered,
  numeric, frozen]`: link `[214, 76, 179, 556]` against the stored
  `175`; bracket `[428, 141, 343, 1096]` against `341`; pad
  `[854, 128, 979, 2722]` against `971` / `2750`. Plate and annulus
  match.
- `editor-core/tests/m10_9_pins_interval.rs`,
  `m10_9_the_pads_four_at_both_dials`: the shipped side read
  `854 / 128 / 979 / 2722` against the asserted `(854, 128, 971, 2750)`
  (the gating row, `m10_9_no_registrant_lies_on_any_measured_document`,
  pins only `registered` and `symbolic_zero` and so stayed green).

`registered`, `registrations_refused`, `registrations_contradicted` and
`theorems_disputed` did not move, so no registrant is implicated. What
is **not** measured: which change moved `numeric` (+4 link, +2 bracket,
+8 pad) and shrank the pad's `frozen` set by 28. The sym11 row's own
message calls a moved receipt "a finding, not a table to refresh".

Both tables were re-taken on `encl/batch-split-loop-mustcarry` to the
values that are true there, because the must-carry gate moves the pad
again (by +28 `symbolic_zero` and +84 `numeric`, attributed by A/B in
that batch). This row owns the part the gate did not cause. Its
remedies are to bisect main for the commit that moved these numbers
and to say whether each moved decision is right, and to give the two
rows a schedule: `#[ignore]` plus "re-taken at each SYM unit's close"
let the drift land silently.
