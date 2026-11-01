---
id: mate-solve-rebuilds-the-nominal-environment-per-check
kind: issue
title: check_reference and derived_offset build doc.param_env::<f64>() afresh on every call, twice per mate and twice per cluster edge inside one solve
status: closed
opened: 2026-09-08
closed: 2026-09-19
pr: 2885
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

## Closed (2026-09-19, PR 2885)

Fixed by MSOLVE-7: `solve_document` builds `doc.param_env::<f64>()`
once and hands it down as a parameter — `check_reference(doc, env,
…)`, `derived_offset(doc, env, …)`, through `solve_cluster` and
`pair_left_factor` — and the sentence about the nominal environment
(the document's own, under no box and no seed) lives at that one
build site, the two readers pointing there. Pinned by
`msolve7_member_residue::a1_the_solve_builds_its_nominal_environment_
exactly_once` (the one `param_env` build under `mate/` stands in
`solve_document`'s body; `member.rs` holds none) and by the
signatures. No verdict moved: every mate row passes unchanged at the
default ε, 1e-6 and 1e-12.
