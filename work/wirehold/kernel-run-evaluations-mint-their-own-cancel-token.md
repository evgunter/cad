---
id: kernel-run-evaluations-mint-their-own-cancel-token
kind: issue
title: Evaluations the kernel runs on a caller's behalf mint a fresh CancelToken, so the caller's cancel never reaches them
status: open
priority: P2
cost: M
opened: 2026-11-02
---


Found by designer B on the `a-family-has-no-member-ceiling` fork (2026-11-02); sites re-listed by the WIRE orchestrator against main.

Non-test evaluations started inside the kernel with `CancelToken::new()`, a token nobody holds:
- `edit.rs:6979` (`nearest_to_face` evaluates the whole document inside an edit)
- `drive.rs:1299, 1594, 1611, 1983, 1989` (inside the subdivision driver)
- `clearance.rs:1784`
- `stackup.rs:587, 617, 632, 2245, 2256`
- `checks.rs:1585`
- `refactor.rs:2711`
- `mate/corner.rs:275`
- `eval/mod.rs:2802, 2874, 2880, 2920`
- `eval/parts.rs:1184`

(Test-only sites — `test_support.rs`, `emit_topo.rs` tests, and `members.rs` tests — are not findings.)

A caller cancelling an edit, a drive, a clearance or a stackup run cannot stop the evaluations that run inside it, and a family inside them yields to a token no one can set. The fix is to thread the caller's stop condition through. Its shape depends on the ceiling fork: if the fork adopts a caller-stated evaluation budget, the stop condition carries both the token and the budget.
