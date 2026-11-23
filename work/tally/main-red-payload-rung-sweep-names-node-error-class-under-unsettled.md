---
id: main-red-payload-rung-sweep-names-node-error-class-under-unsettled
kind: issue
title: main red: the payload-rung sweep names NodeErrorClass (eval/class.rs) under Unsettled at drive.rs, on no curated list
status: open
opened: 2026-11-23
---


The `lint` job's payload-rung sweep (`scripts/payload-rung-sweep.py`) fails on main's merge base as of 2026-11-23. PR 7408's head `aa60e4c706`, run 45147103119, quotes it:

> NEW payload rung, curated on no list that carries its carrier: NodeErrorClass at crates/editor-core/src/eval/class.rs:41 under Unsettled (crates/editor-core/src/drive.rs:635). Decide it — carry it where its carrier is c…

PR 7408 does not touch `editor-core`. The site arrived with main's recent merges into `drive.rs` (`581b4e1090`, the split-reads fix pass; PR 7369 and `sf2` merged the same day). Every PR gated on today's main inherits the red until the rung is decided: carried where its carrier is curated, or listed.

The same merges bring a second red, to the `test (app)` job: `.config/nextest.toml`'s slow-set entry `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere` matches no test. `85a85f0f54` ("split rows: the arc loft refuses the root…") renamed the test to `an_arc_loft_refuses_the_root_under_a_box_elsewhere` and left the entry behind.
