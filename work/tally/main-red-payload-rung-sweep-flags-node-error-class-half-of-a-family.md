---
id: main-red-payload-rung-sweep-flags-node-error-class-half-of-a-family
kind: issue
title: main red: payload-rung-sweep --check flags NodeErrorClass (HalfOfAFamily) under Unsettled, uncurated
status: open
opened: 2026-11-23
priority: P1
cost: E
---


## The finding

(Filed by the WALKS orchestrator, 2026-11-23.) `python3 scripts/payload-rung-sweep.py --check` exits 1 on origin/main d7963dd38f — so every PR's `lint (fmt, clippy, gates)` gates step is red and skips fmt, clippy and rustdoc behind it:

> NEW payload rung, curated on no list that carries its carrier: NodeErrorClass at crates/editor-core/src/eval/class.rs:41 under Unsettled (crates/editor-core/src/drive.rs:635). Decide it — carry it where its carrier is carried, or argue the non-carriage beside that carrier — and add it to CROSS_LIST_DISPOSITIONS with the home of the argument.

It arrived with `NodeErrorClass::HalfOfAFamily` (commit a9c18332b0, "eval: narrow to the typed refusal; revert the per-member projection", merged via a `c-sf` branch). `crates/editor-core/src/eval/class.rs` is TALLY's ground, so the decision is TALLY's (or the program whose branch landed it).

## What a fix is

What the gate's message says: carry `NodeErrorClass` where `Unsettled`'s carrier is carried, or argue its non-carriage beside that carrier, and add the disposition to `CROSS_LIST_DISPOSITIONS`.

**Seen again** on PR 7408 (`sectcone/certified-foot`, run 45147103119, 2026-11-23). The same merges left a second red, in `test (app)`: `.config/nextest.toml`'s slow-set entry `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere` matches no test. `85a85f0f54` renamed that test to `an_arc_loft_refuses_the_root_under_a_box_elsewhere` and left the entry behind.
