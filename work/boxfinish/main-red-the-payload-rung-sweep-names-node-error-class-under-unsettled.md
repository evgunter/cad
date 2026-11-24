---
id: main-red-the-payload-rung-sweep-names-node-error-class-under-unsettled
kind: issue
title: main red (lint): payload-rung-sweep --check names NodeErrorClass under drive's Unsettled, on no disposition list, since PR 7458
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by `oneround/box-region` (PR 7225), whose lint job reds on it (run 45132012432).

**What is red.** The lint job's gates step runs `python3 scripts/payload-rung-sweep.py --check`. On main `43295779` that check prints:

> NEW payload rung, curated on no list that carries its carrier: NodeErrorClass at crates/editor-core/src/eval/class.rs:41 under Unsettled (crates/editor-core/src/drive.rs:635). Decide it — carry it where its carrier is carried, or argue the non-carriage beside that carrier — and add it to CROSS_LIST_DISPOSITIONS with the home of the argument.

`drive.rs`'s `Unsettled` started carrying `NodeErrorClass` in PR 7458's merge `50c82758` (`boxfinish/split-reads`): the first-parent `-S` search for `NodeErrorClass` in `drive.rs` finds that merge.

**What is owed.** Carry `NodeErrorClass` where `Unsettled` is carried, or argue the non-carriage beside `Unsettled` and add the `CROSS_LIST_DISPOSITIONS` row the check names.
