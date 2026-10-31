---
id: the-judgement-naming-fault-seam-refuses-other-tests-unions
kind: issue
title: The judgement-naming fault seam is one process-global Mutex, so under threaded cargo test it refuses other tests' unions
status: open
opened: 2026-10-28
priority: P4
cost: E
---

Filed by the WIRE orchestrator from the PR 5902 review (MINOR, pre-existing, off that PR's change).

## What

`crates/editor-core/src/test_support.rs:1420`: `with_judgement_naming_fault` arms `static JUDGEMENT_FAULT: Mutex<Option<usize>>`, which is process-global.
- While one test holds it, every union evaluated in the same process refuses the naming of judgement `k` with `JUDGEMENT_NAMING_FAULT`, including unions in other tests.
- The PR 5902 reviewer's parallel run had `name_invariance::a_member_order_moves_no_name` fail with "a test refused the naming of a union's judgement". It passed alone.

CI is unaffected: nextest runs each test in its own process. A threaded `cargo test` run of `editor-core::all` is not.

## Done when

The seam reaches only the evaluation of the test that armed it: a thread-local, or a value carried in the evaluation options. A witness runs two threads, one armed and one not, and the unarmed thread's union names.
