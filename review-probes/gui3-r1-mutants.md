# Review probes — GUI-3 R1 (PR #1101, frozen head 956ef3cf)

Four mutants applied, run, reverted (recorded so the runs are
reproducible). Scoped runs: `cargo test -p viewer --test all -- <filter>`.

## M1 — `History::commit` truncates like a stack
`crates/viewer/src/history.rs`, commit: clear the parent's children and
`entries.truncate(parent.0 + 1)` before appending.
Result: **RED** — `undo_tree::an_edit_after_an_undo_mints_a_sibling_and_destroys_nothing`
and `undo_tree::a_save_writes_the_current_paths_linear_log` fail (2/5).
Note: `redo_follows_the_current_branch_not_the_abandoned_one` PASSES
under truncation (with the old branch destroyed, redo trivially reaches
the new one) — the tree-shape assertions are what carry the claim,
exactly as the suite's header says.

## M2 — `undo` forgets which child it left
`history.rs`, undo: drop the `active_child = Some(leaving)` write.
Result: **GREEN across the shipped suite AND R1's rows.** Equivalent
mutant under the exposed op vocabulary: `commit` already stamps
`active_child` along every path the cursor can walk, and with only
undo/redo (no cursor jump) the cursor can never sit on a branch the
active chain does not already name. The write becomes load-bearing only
when GUI-6's branch picker jumps the cursor. Not a suite defect; noted
so the PR-body sentence "undo records which child it left" is read as
mechanism, not as an independently observable behavior.

## M3 — `DocSession::land` ignores the generation
`session.rs`, land: remove the `done.generation != self.generation`
early return.
Result: **RED** — `eval_seam::a_stale_result_is_discarded_by_generation`
fails.

## M4 — the poisoned badge invents a string
`tree.rs`, status_of: `message: Some("upstream failed".to_owned())`
instead of `ev.node_error(id)`.
Result: **RED** — `tree_badges::a_failing_document_renders_failed_and_poisoned_from_the_typed_payloads`
and R1's `r1_a_two_hop_poison_chain_reports_the_root_cause` both fail:
the byte-for-byte payload pin does its job.

## Non-mutant measurements

- `demo-tour gallery` regenerated at the witness ε: `ring.pncad` is
  **byte-identical** to the committed
  `crates/viewer/tests/gallery_ring.v14.pncad`.
- Regenerated at `CAD_TOLERANCE_EPS=1e-12`: differs from the fixture in
  **exactly the `"epsilon"` line** (`1e-9` → `1e-12`) — the doc_io
  re-stamp comment's measured claim, re-taken and confirmed.
- `cargo test -p viewer --test all -- doc_io` at
  `CAD_TOLERANCE_EPS=1e-12`: green (the re-stamp mechanism works at the
  ε that killed run 33107827538).
- ThreadEvaluator, two submits back-to-back (r1_e2e §6): answers are
  `[(Generation(0), Canceled), (Generation(1), Completed)]` — TWO
  results, the first canceled and generation-discarded. The
  InlineEvaluator's "two submits → one result" coalescing is
  inline-only; the trait doc's "replaces any queued one" describes the
  inline shape. Session-level behavior agrees between the two lanes
  (the canceled gen-0 result lands Stale); the seam-level poll counts
  differ.
