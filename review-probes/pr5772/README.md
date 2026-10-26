# PR 5772 dual review probes (FLUSH, nary door), frozen head ec3a48d4

- `hook.py` patches `crates/editor-core/src/eval/wire.rs` (base bd36ab7b and
  head alike) with a capture hook around `wire_combine`: per evaluation, the
  hash of the payload (body), the output name tables, contacts, rows,
  cited inputs and carried findings, or of the error. `NARY_CAPTURE=<dir>`
  turns it on. Not for merging.
- `reaper.sh` killed non-union debug interval tests past 400 s (same 4-5 on
  both sides; listed in the report).
- `forge_compile_fail.rs`: seven ways a sibling crate might hand a step a
  glue. Added to `crates/verbs/tests/all.rs` by a `#[path]` line,
  `cargo check -p verbs --tests` gives 7 errors (E0425 x2, E0603 x2,
  E0624 x2, E0560).

Results (editor-core default profile with the slow set, debug):
- base and head: 3013 run, 2985 pass, the same 28 fail.
- capture: 462 tests, 11,527 union/intersect evaluations on each side
  (78 errors), including docm5_subject's four tests (69 evaluations).
  Identical per test, in order, except the rows of
  `coincidence_door::an_unproven_fold_row_is_reported_under_the_judged_row_decided_at_its_point`,
  where 8 `ZeroWord` indices move (17216->17210, 17753->17747, 39186->39180,
  39723->39717, 151847->151843, 152376->152372, 173716->173712,
  174245->174241); full Debug of the rows otherwise identical.
