# Review probes for PR 5772 (frozen head ec3a48d4), not for merge

- `base-hooks.patch` / `head-hooks.patch`: the review-only capture hook
  (`UNION_CAPTURE_DIR`), the judged-pair and given-pair lines, and the
  judgement-naming fault mutant (`REVIEW_FAULT_JUDGEMENT=k`), applied to
  the merge base `bd36ab7b` and the head `ec3a48d4` respectively.
- `cmp.py`: compares two capture directories per test as multisets.
- `review_precedence_probes.rs`: the two precedence fixtures (editor-core
  tests; wire into `crates/editor-core/tests/all.rs`).
- `crates/sweep/tests/nary_door_review_probes.rs`: CONTACTHOLD m2 reached
  through `topo::nary_boolean`.
- `crates/editor-core/tests/review_forgery/forge.rs`: six forgery routes;
  each fails to compile on the head.

## Results (2026-10-26)

- editor-core release, default profile with the slow set (docm5_subject
  included): 464 tests, 11,529 union/intersect evaluations (11,450 bodies,
  79 refusals); identical multisets on base `bd36ab7b` and head
  `ec3a48d4` except `coincidence_door::an_unproven_fold_row_…`, whose rows
  differ only in `ZeroWord` indices (identical once masked). Judged-pair
  and given-pair lines: 184 tests, 38,880 lines, identical unmasked.
  Failing sets identical (29, all main-red or the probe file's
  aggregation row, plus two e6 interval timeouts).
- ci profile at eps 1e-9, 1e-6, 1e-12, topo+sweep+verbs+editor-core:
  795 evaluations per eps identical; the only red on either side is
  `every_suite_file_is_aggregated`, caused by the probe file.
- Head, 1 vs 4 rayon threads, union/intersect filter with slow set:
  224 tests, 6,242 evaluations identical.
- Precedence mutant (`REVIEW_FAULT_JUDGEMENT=0`): base returns the
  injected naming error for both fixtures; head returns the later
  judgement's sliver refusal (union) and `EMPTY` (intersect).
- m2 through the door, dy = 1e-11: judged tangencies margin 0.0 (P1,R)
  and 1.0e-11 (P2,R); the step's restated row carries 0.0, restates both.
- Batteries (release, ignored): 5/5 both sides, outcome lines identical.
- Die union (21 pips) peak RSS: base 142.2 MB, head 142.7 MB.
