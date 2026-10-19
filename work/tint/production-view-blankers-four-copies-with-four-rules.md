---
id: production-view-blankers-four-copies-with-four-rules
kind: issue
title: Four hand-rolled 'production view' blankers each decide what a test cfg gates by their own rules
status: open
opened: 2026-10-19
---


Found by the review of PR 5401 (`encl/kernels-own-type`).

Four guards each build a "production view" of a Rust file, blanking what
a test-only `cfg` gates, and each has its own rules for which `cfg`
counts and how far its item runs:

- `tier3_tests::check_4_and_the_rim_route_their_second_order_reading_through_the_one_walk`,
  an inline closure (`crates/topo/src/tier3_tests.rs` ~2112): only the
  literal `#[cfg(test)]` on a `mod`, blanked from the next `{`. A gated
  `mod name;` has no `{` of its own, so it would blank the next item's
  body.
- `without_test_modules` (`crates/topo/src/boolean/offer_rows.rs` ~2393):
  only `#[cfg(test)] mod name { .. }` blocks.
- `gates_on_test` plus `test_module_starts_at` and `gated_module_files`
  (`crates/profile/tests/raw_door_census.rs` ~152): any attribute run
  that mentions `cfg(` and `test`, with the test region running from the
  first gated `mod` block to the end of the file.
- `policy_lane::origin_census::production`
  (`crates/topo/src/policy_lane.rs`): `cfg(test)` and `cfg(any(..))` over
  `test`, `feature = "test-support"` and `-testing` features, each gated
  item, field, variant or arm blanked to its own end.

The last of these already uses the shared extent,
`test_utils::source::attributed_extent` (`crates/test-utils/src/source.rs`,
added by PR 5401). Its first version scanned a gated field to the next
`;`/`{` and blanked the next production item silently. The
`tier3_tests` copy can still do the same through a gated `mod name;`.

The fix: one `test_utils::source` function that answers "this file's
production view" (which `cfg` is test-only, plus `attributed_extent`),
and the four guards calling it. `test_only` in `origin_census` is the most
complete `cfg` reader of the four.

Checked and not a blanker: `euler.rs` ~7724 (`reference_variants` steps
over a variant's attributes, which is correct) and
`crates/test-utils/tests/deny_unknown_fields_census.rs` (it reads gated
declarations like any other, on purpose).
