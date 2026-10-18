---
id: free-move-post-merge-review-minors
kind: issue
title: viewer: PR 5296's post-merge review — an overtaken pin landing, unpinned NotEvaluated and pane window, stale docs
status: open
opened: 2026-10-18
priority: P2
cost: E
---


The post-merge whole-unit Opus review of PR 5296 (final head 6bc4618965) gave
APPROVE-WITH-FIXES and raised no MAJOR. Its findings follow, all shown by
execution unless marked.

1. **A pin landing that a value gesture overtakes (code).** The second prune in
   `DocSession::land` (`crates/viewer/src/session.rs`) runs only on a landing of
   the committed document. That assumes a scratch landing could newly answer
   nothing but a value-refused placement. It also carries any committed edit
   whose own landing it made stale. The sequence: commit a frame on a loose
   copy, undo (which re-pins it) without pumping, preview a value gesture, then
   pump. Mid-drag the probe treatment silently leaves the picture
   (`moved_placements` is empty while `free_move_of` is still `Some`), and the
   `InTheWorld` notice arrives only after the cancel, worded as if the cancel
   caused it. A fix: prune at a scratch landing for structural causes, or
   remember an unpruned committed landing and prune it when it is overtaken.
   The premise text at `session/op.rs` (`permitted_during_value_gesture`) says
   the same, and needs the same correction.
2. **`NotEvaluated` and the Ruled "never an older document's spaces" have no
   row (test-gap).** Two mutants survive the suite: `space_evaluation` returning
   `Some(run.evaluation)` unconditionally, and the properties pane reading
   `evaluation()` instead of `space_evaluation()`. The row to add: unpin, undo
   without pumping, then `BeginFreeMove` refuses `NotEvaluated`, and the pane
   shows the same refusal.
3. **Two more surviving mutants (test-gap):** `MovedSpace`'s "left" guard set to
   false (the split row asserts the payload but not the sentence), and
   `NotEvaluated`'s rank in `session/refuse.rs` changed from 2 to 1.
4. **Stale prose (doc, by inspection):**
   - `commit`'s doc in `session.rs` says a probe "is superseded by the mate";
   - `frame.rs`'s `WithdrawalKind::Superseded` says "the mate that landed",
     "names the instances" and "`free_move_check` is the condition";
   - `display.rs`'s `DisplayFault` and `Withdrawn::cause` docs name two prune
     tests, where `gesture_check` makes three;
   - the module doc's lists of how a frame goes omit `NoSpace`.
5. **Style:**
   - `gesture_check`'s match repeats `space_check`'s (`display.rs`): fold the
     two into one helper over the asked node.
   - The census in `frame_policy` samples `MovedSpace`'s second sentence under
     the ordinal its first already holds, so nothing would notice that sample
     going missing.
   - `AdmissionFault`'s "instance N" vocabulary rule fails for a `Place` root.
   - Pre-existing: `session/op.rs` and `gesture_table.rs` say the pane reads the
     previewed document, but it reads `committed_doc()`.
