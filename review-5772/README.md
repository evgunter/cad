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
