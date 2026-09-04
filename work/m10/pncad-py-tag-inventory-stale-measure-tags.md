---
id: pncad-py-tag-inventory-stale-measure-tags
kind: issue
title: pncad-py TAG_INVENTORY is stale: node_error_tag's two Measure arms are uninventoried (main is red)
status: open
opened: 2026-09-04
---


## The finding

`main` is red. `pncad_py::tests::the_whole_tag_table_matches_its_committed_inventory`
fails on main's own tree, not on any branch's:

```
src/tags.rs has moved away from TAG_INVENTORY.
  `node_error_tag`: value(s) ADDED ["measure_clearance_refused", "measure_selection_kind"],
  value(s) GONE [] (a RENAME shows as one of each)
```

`crates/pncad-py/src/tags.rs:318` maps
`NodeErrorKind::MeasureSelectionKind { .. }` to `"measure_selection_kind"`
and `crates/pncad-py/src/tags.rs:322` maps
`NodeErrorKind::MeasureClearanceRefused(_)` to
`"measure_clearance_refused"`. Neither string appears anywhere in
`TAG_INVENTORY` (`crates/pncad-py/src/tests.rs:1427`), which the test at
`crates/pncad-py/src/tests.rs:2629` compares it against.

The test's own message states the stake: these values are a **public
Python contract**, so an uninventoried addition is unreviewed public
surface. The repair is the one the message names — add the two rows to
`TAG_INVENTORY`, and check `pncad.pyi` and `tests/*.py` for callers.

## Provenance

Introduced by `5a3fc8389` *"M10-6 part 1: the min_clearance primitive,
the min_separation door, the reporting layer's first doors"* — the
commit that added both `node_error_tag` arms without the matching
inventory rows. Homed here because M10-6 is M10's.

## Reproduction (executed, on main's own tree)

A detached worktree at `origin/main` (`d3a1303f6`), its own
`CARGO_TARGET_DIR`, nothing of any branch in it:

```
cargo test -p pncad-py --lib the_whole_tag_table
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 33 filtered out
```

Identical message to the one hosted CI reports. Found from PR 1749,
whose diff touches no file under `crates/pncad-py/` — that directory is
byte-identical between 1749's head and `origin/main` — so the red is
inherited there and annotated on that PR rather than absorbed.
