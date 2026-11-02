---
id: the-clearance-leaf-doors-ask-an-assertions-question-a-second-way
kind: issue
title: editor_core::clearance's leaf doors have no caller outside tests and ask an assertion's question by a second spelling
status: open
opened: 2026-11-01
priority: P3
cost: M
---


Found by the clearance-consumer designers (DOORS fork, `[ev]` PR 6261, 2026-11-01). All three reported it independently; the orchestrator has not probed it.

`editor_core::clearance` (`clearance`, `clearance_with`, `clearance_over` in `crates/editor-core/src/clearance.rs`) answers E7's toleranced question, "separation ≥ c over a drive's certified leaves", with `Holds` / `Violated { witness }` / `Refused`. It has no caller outside tests, and the `pncad` façade leaves it out on purpose (the header of `crates/pncad/src/analysis.rs`).

The question it asks is an assertion's, a `min_clearance` measure compared against a bound. Yet it is spelled a second way: its own `Selection` plus a constant `c`, rather than the assertion node. When a tolerance surface arrives, in the GUI or the CLI, the question should reach it through the assertion: the drive, then `assertion_at` per leaf, with the violation witness moved there. Otherwise retire the door. Precedent for the shape: `work/coincide/enforce-checks-has-no-caller-outside-tests.md`. (DOORS orchestrator)
