---
id: tag-inventory-misses-the-m10-6-measure-tags
kind: issue
title: TAG_INVENTORY misses the two M10-6 measure tags: main is red on every PR's merge ref
status: open
opened: 2026-09-03
---


`pncad-py::tests::the_whole_tag_table_matches_its_committed_inventory`
(`crates/pncad-py/src/tests.rs:2516`, panicking at `:2629`) is RED on
main and therefore on every open PR's merge ref. Read from a hosted run
(33788875237, `test (interval, eps = 1e-12, 2/2)`), the payload:

```
src/tags.rs has moved away from TAG_INVENTORY.
  `node_error_tag`: value(s) ADDED ["measure_clearance_refused",
  "measure_selection_kind"], value(s) GONE [] (a RENAME shows as one of each)
```

Both values are emitted by `crates/pncad-py/src/tags.rs`'s
`node_error_tag` (`measure_clearance_refused` at `:322`) and neither
appears in `TAG_INVENTORY`'s `node_error_tag` row
(`crates/pncad-py/src/tests.rs:1637`–`:1667`, which still ends at
`measure_unsupported`).

**How it landed, and why no run caught it.** The inventory row is
#1696's (`lib: pin the refusal-tag VALUES, the whole table, by name`,
434964dfa); the `NodeErrorKind` arms it does not list come from the
M10-6 reporting work (`MeasureClearanceRefused` at 58b6cf9af, part of
#1685). Neither PR's merge ref contained both halves, and main's own
push runs carry only what is unique to them
(`.github/workflows/ci.yml`, the 2026-08-20 reduction), so the full
test matrix never ran over the two together. That is the
cross-merge shape, not a defect in either PR.

**Not a mechanical fix, which is why this is filed rather than taken.**
The inventory's header says the tag VALUES are a public Python
contract, and its own instruction on firing is to update the inventory
"in this same commit, and check `pncad.pyi` and `tests/*.py` for
callers of every word that moved". Adding two strings to the list
silences the row; deciding that `measure_clearance_refused` and
`measure_selection_kind` are the names Python callers should branch on,
and that `pncad.pyi` documents them, belongs to M10-6's owner.

Found by the TCOST-K3 lane, whose gate is red on this row and nothing
else; that branch touches no `pncad-py` or `editor-core` file.
