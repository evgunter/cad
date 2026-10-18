---
id: free-move-post-merge-review-minors
kind: issue
title: viewer: PR 5296's post-merge review — an overtaken pin landing, unpinned NotEvaluated and pane window, stale docs
status: closed
opened: 2026-10-18
closed: 2026-10-18
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
   - Pre-existing: `crates/viewer/src/session/op.rs`
     (`permitted_during_value_gesture`) says the pane resolves against the
     previewed document, and `crates/viewer/tests/gesture_table.rs`
     (`a_value_gesture_and_a_free_move_probe_do_not_disturb_each_other`'s doc)
     calls it "the shown one the panel draws the control from". The pane
     reads `committed_doc()`.

## Taken in the PR that closed `an-overtaken-pin-landing-is-never-pruned`

- Finding 1 got an item of its own, and is fixed.
- Finding 2's admission half: `the_probe_refuses_while_the_document_has_not_landed`
  pins `NotEvaluated` and the rule against reading an older document's spaces.
  The `space_evaluation`-unconditional mutant now goes red.
- Finding 3's sentence half: the split row asserts the "a loose space of its own
  now" sentence.
- Finding 4: all four stale passages are corrected.
- Finding 5's first bullet: `space_check` and `gesture_check` share `in_space`.

Still open here:
- Finding 2's pane half: the properties pane reading `evaluation()` instead of
  `space_evaluation()` survives. It wants a driven-app row, unpinned and then
  undone before the landing.
- Finding 3's rank half: the `NotEvaluated` rank mutant.
- Finding 5's census sample sharing an ordinal, and the "instance N" rule for
  a `Place` root.
- The pre-existing pane-document prose.

## Closed (2026-10-18)

- **Finding 2's pane half:**
  `app::properties_pane_tests::the_pane_does_not_offer_the_probe_off_an_older_landing`.
  It holds an undo's run behind the `Gated` seam and checks that the pane says
  "not known yet" rather than offering the probe off the older landing. It goes
  red with the pane reading `evaluation()`.
- **Finding 3's rank half:** `a_space_not_known_yet_ranks_below_a_space_that_refuses`
  (the earlier PR).
- **Finding 5's census:** `cause_ordinal` now gives each sentence its own
  ordinal, mirroring `MovedSpace`'s guard, so dropping a sample breaks the
  contiguity check.
- **The "instance N" rule:** the census asserts that no sentence but
  `NotAnInstance`'s writes "instance" itself, since the spoken node says its
  subject's kind (a frame's root may be a `Place`).
- **The pane-document prose:** corrected in the earlier PR.
