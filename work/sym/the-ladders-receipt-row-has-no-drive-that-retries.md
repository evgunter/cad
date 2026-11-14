---
id: the-ladders-receipt-row-has-no-drive-that-retries
kind: issue
title: sym_9_the_drive_writes_the_ladders_receipt pins no nonzero retried= column: its pad no longer retries in a one-leaf drive
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by BOXFINISH's `boxfinish/sweep-frames`, which re-baselined the row.

`crates/editor-core/tests/sym_9_retry_interval.rs` `sym_9_the_drive_writes_the_ladders_receipt` drives the pad with its first leg written as an angle (`m10_8_r2_probes_interval::pad_first_leg_by_angle`) in one leaf, and pinned the ladder's receipt: `retried=32` on the goldening line (`drive.rs`, `ParamBoxVerdict::serialize`, the `" retried={}"` write) and "32 of those discharges reached only by a second attempt" in the human form. Since the sweeps keep their sketch frames, the decisions the ladder retried on that leaf discharge on the first attempt: shipped and bare both read `[1400, 1762, 0]`, and the row now pins that nothing retried writes no column.

So no row pins the column when it is written: a regression in how a nonzero `retried` is serialized or rendered would pass. The kept-atom row (`sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`) still measures a recovery of 6 on the filleted bracket, but at a nominal replay; the bracket's one-leaf drive at scale 1 does not certify (measured: `certified 0`), so it is not a drop-in.

**Repair shape.** Find or build a one-leaf drive the ladder still retries on, and pin its receipt line and human form there.
