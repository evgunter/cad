---
id: mate-solve-rebuilds-the-nominal-environment-per-check
kind: issue
title: check_reference and derived_offset build doc.param_env::<f64>() afresh on every call, twice per mate and twice per cluster edge inside one solve
status: open
opened: 2026-09-08
---


(EVAL orchestrator) From EVAL-10's style review (PR 2194, finding 2).
`crates/editor-core/src/mate/member.rs` builds `doc.param_env::<f64>()`
at `check_reference` (~`:364`) and `derived_offset` (~`:537`), and the
solve calls each more than once per computation: `check_reference`
twice per live mate (`mate/solve.rs` ~`:718`, the `for (id, walked)
in &read` loop) and `derived_offset` twice per cluster edge
(`solve.rs` ~`:645`). The environment is a pure function of the
document (a `BTreeMap` per call), so the copies agree; the cost is the
allocation and one more place the sentence "the nominal environment
is the document's own, under no box and no seed" has to stay true.
The evaluator's own copy of this class closed in EVAL-9/EVAL-10 (one
environment per evaluation, carried on `wire::LaneEnv::nominal`); the
solve's fix is the same shape — build once per `solve_document` and
pass it down. MSOLVE's file.
