---
id: tag-inventory-missing-two-measure-tags
kind: issue
title: main is red at any drawn test lane: src/tags.rs carries two node_error_tag values TAG_INVENTORY does not
status: open
opened: 2026-09-04
---


Found by CIW unit 5's gated run (PR 1722, run 33822129170), which is the
first run in some time to actually DRAW a test lane. Not caused by that
diff: it touches `scripts/criterion-emit.py`,
`scripts/opt-level-calibrate.py`,
`crates/editor-core/tests/m4_pr8_latency.rs` and three
`docs/perf-data/*/README.md` files, none of which this test reads.

## The failure

`crates/pncad-py/src/tests.rs:2629`, in
`the_whole_tag_table_matches_its_committed_inventory`:

    src/tags.rs has moved away from TAG_INVENTORY.
    `node_error_tag`: value(s) ADDED ["measure_clearance_refused",
    "measure_selection_kind"], value(s) GONE []

Both values are live on `main`:

* `crates/pncad-py/src/tags.rs:318` —
  `NodeErrorKind::MeasureSelectionKind { .. } => "measure_selection_kind"`
* `crates/pncad-py/src/tags.rs:322` —
  `NodeErrorKind::MeasureClearanceRefused(_) => "measure_clearance_refused"`

`TAG_INVENTORY` in `crates/pncad-py/src/tests.rs` names neither, so the
guard fires exactly as designed. Nothing is wrong with the guard.

## Why it went unseen

`main`'s three most recent CI runs (`59c461c`, `ae25e03`, `d8d0256`)
all SKIPPED both `test (...)` rows — docs-tier diffs — so the point that
executes this test has not been drawn on `main` since the tags landed.
It is a latent red, not a new one: any code-tier run on any drawn lane
reds on it.

## The fix, and what it obliges

The test's own message states the contract and it is the right one:

> THE TAG VALUES ARE A PUBLIC PYTHON CONTRACT, not an implementation
> detail: Python callers branch on these strings, so a renamed value is
> a breaking change to the bindings and a new one is new public
> surface.

So the fix is not just an inventory line each. Both words are NEW PUBLIC
SURFACE on the Python side and the same commit owes: `TAG_INVENTORY`
entries, a check of `crates/pncad-py/pncad.pyi` for whether the two
`reason` words are documented there, and a check of
`crates/pncad-py/tests/*.py` for callers.
