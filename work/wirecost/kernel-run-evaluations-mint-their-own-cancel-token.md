---
id: kernel-run-evaluations-mint-their-own-cancel-token
kind: issue
title: Evaluations the kernel runs on a caller's behalf mint a fresh CancelToken, so the caller's cancel never reaches them
status: dispatched
priority: P2
cost: M
opened: 2026-11-02
branch: wirecost/kernel-evaluations-take-the-callers-stop
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

## Build (2026-11-02, lane wire-budget, branch `wirecost/kernel-evaluations-take-the-callers-stop`)

The stop condition is one type, `editor_core::Stop { cancel, member_budget }` (`eval/mod.rs`), with `Stopped` (canceled, or exhausted with its budget) as what a door refuses with. A door that builds the options of the evaluations it runs takes a `&Stop`; a door handed whole `EvalOptions` takes the `&CancelToken` alone, its options carrying the budget. Sites, re-grepped at the merge base:
- `drive.rs` (`drive`'s witness, `classify`'s two lanes, `probe_midpoint`, `assertion_at`; `witness_options`/`lane_opts` take the stop): the caller's `&Stop` at `drive`; `DriveRefusal::Stopped`. `range::certified_range` passes its own.
- `clearance.rs` (`verify_witness`, the leaf replay through `read_leaf_with`): `ClearanceQuery::stop`, set by `at_least`/`strictly_positive`; `ClearanceRefusal::Stopped`.
- `stackup.rs` (record, base, passes, `bind_verdict`, `worst_case`'s prior and leaves): `sensitivities`/`stackup` take `&Stop`; a stopped run is `BindingViolation::{Incomplete, Exhausted}`.
- `mc.rs` (nominal, each sample): `monte_carlo` takes `&Stop`; `McRefusal::Stopped`, the first in sample order.
- `report.rs` (`leaf_histogram`): takes `&Stop`, answers `Result<_, Stopped>`.
- `checks.rs` (`run_checks_on`'s inline door) and the coincidence door (`coincide::prove`, `DoorJob::run`, `replay.rs`'s replay options): take `&Stop`; `ChecksError::Canceled`.
- `refactor.rs` (`split`): takes `&Stop`; `SplitError::Stopped`.
- `mate/corner.rs` (`corner_frame`), `eval::solve_document_at` / `mate::solve_document`, `edit.rs` (`sketch_on_face`, `constrain`, `remove_mate`, `nearest_to_face`), `take_over.rs` (both runs): take `&CancelToken` beside their `opts`.
- `eval/mod.rs` leaf doors (`LeafPrior::of`, `replay_leaf`, `read_leaf_with`): take the token; a replay that stopped says so (`LeafReadback::stopped`, `Err(Stopped)`).
- pncad-py: `Evaluation` keeps the stop it ran under (`cancel=`, `member_budget=`), and `Evaluation.coincidences` and `run_checks` replay under it. The other Python doors (`monte_carlo`, `split`, `corner_frame`, the mate and sketch gestures) take no `cancel=` today: top-level doors, a default stop at the door.
- viewer: the threaded door seam passes its own token (`Stop::canceled_by`). The UI thread's synchronous gestures and reads (`session.rs`, `tree.rs`, `scene.rs`, the panes, `evalseam::evaluate_beside` and the inline seams) have no token a cancel reaches: top-level, unchanged.
- Test-only (`test_support.rs`, `emit_topo.rs` and `stackup.rs` tests, `evaluate_running_pass_two`): not findings.

Not done here: an edit whose inner run stopped still commits (`nearest_to_face` falls back to the identity, `rebundle_by_geometry` takes the structural bundle), which is the existing posture for a face that does not read.
