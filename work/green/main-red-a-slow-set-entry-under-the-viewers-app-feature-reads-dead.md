---
id: main-red-a-slow-set-entry-under-the-viewers-app-feature-reads-dead
kind: issue
title: Main red: the slow-set name check lists viewer without its app feature, so PR 7548's feature-gated slow entry reads dead
status: closed
opened: 2026-11-23
priority: P0
cost: E
closed: 2026-11-23
pr: 7599
---


`test (app)`'s slow-set name check (`.github/workflows/ci.yml`, the
step that pipes `cargo nextest list --ignore-default-filter $SCOPE` into
`scripts/check-slow-set-names.py`) lists `viewer` without
`--features app`. `crates/viewer/src/lib.rs` compiles `pub mod app` only
under `#[cfg(feature = "app")]`, so no test under `app::` is in that
listing.

PR 7548 (`offer/range-is-the-certified-search`, merged at `674fb6d49b`)
added `app::properties_pane_tests::refused_slot_entry::the_range_buttons_grey_from_the_admission_door`
(`crates/viewer/src/app.rs`) and its slow-set entry
(`.config/nextest.toml`, `binary_id(viewer) & test(=app::…)`). It is the
first slow-set entry under `binary_id(viewer)`'s feature-gated module, so
the check reads it dead:

```
check-slow-set-names: 2 dead, 0 duplicated.
##[error]slow-set entry matches no test: viewer: app::properties_pane_tests::refused_slot_entry::the_range_buttons_grey_from_the_admission_door
```

Seen on PR 7400's run 45168956268 (test merge with main `de59b39c79`).
The test and the entry both stand on main `75c8ac8d4b`, unchanged, so
`gate ok` is red on every PR whose run selects `test (app)`. (The other
dead entry in that run, `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere`,
is fixed on main by `188fc58d25`.)

A second consequence: the `viewer app feature` job runs
`cargo nextest run --profile ci -p viewer --features app`, and the `ci`
profile's filter leaves the slow set out, so this test runs in no PR
gate. Whether the nightly runs it with `--features app` is unchecked.

The fix is the check's, not the entry's: list `viewer` with its `app`
feature (or a second listing for the feature-gated tests). Dropping the
entry instead would put a test its author measured as slow back in the
fast set.

## Closed

PR 7599. The default-feature listings (ci.yml's `test` job, nightly.yml's
`full-suite-build`) pass `--elsewhere viewer` to
`scripts/check-slow-set-names.py`, and both viewer jobs check viewer's
entries over `cargo nextest list -p viewer --features app`. The PR viewer
job also runs the slow-set tests its diff adds or edits
(`slow_set.py --new-expr`) under `--features app`; the nightly's viewer job
already ran them (`--ignore-default-filter`). Both app-feature entries
main had dropped to get past the old check are restored: PR 7548's (dropped
by PR 7574) and PR 7562's
`an_offer_an_earlier_op_of_its_frame_outran_is_dropped_and_said` (dropped
by `0f2bca5254`). The probe timed them at 1.871 s and 0.489 s. Of
the slow set's other 2395 entries on `bba39ef945`, none sits under a
positive `cfg(feature …)` gate.
