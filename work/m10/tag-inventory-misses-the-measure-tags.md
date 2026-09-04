---
id: tag-inventory-misses-the-measure-tags
kind: issue
title: main is red: node_error_tag gained two measure/clearance values that TAG_INVENTORY does not pin
status: open
opened: 2026-09-04
refs: [1732]
---


## The finding

`pncad-py tests::the_whole_tag_table_matches_its_committed_inventory`
fails on main. Found from #1732, whose diff does not touch `pncad-py`
at all — the two files the guard reads are byte-identical to main on
that branch, so the red is inherited and not that PR's.

`crates/pncad-py/src/tags.rs` — `node_error_tag` emits two values that
`TAG_INVENTORY` in `crates/pncad-py/src/tests.rs` does not pin:

- `"measure_clearance_refused"`
- `"measure_selection_kind"`

Both arrived in `5a3fc8389` ("M10-6 part 1: the min_clearance
primitive, the min_separation door, the reporting layer's first
doors"), which added the tag words without the inventory row. Neither
string appears anywhere in `crates/pncad-py/src/tests.rs`.

The guard's own message states the stake, and it is the reason this is
worth a row rather than a baseline bump:

> THE TAG VALUES ARE A PUBLIC PYTHON CONTRACT, not an implementation
> detail: Python callers branch on these strings, so a renamed value is
> a breaking change to the bindings and a new one is new public surface.

So the fix is not to make the assertion pass. It is to decide that these
two words are public Python surface, pin them in `TAG_INVENTORY` in the
same shape as their neighbours, and check `pncad.pyi` and `tests/*.py`
for callers — the three things the guard's message asks for.

## Reproduction

```
cargo test -p pncad-py --lib the_whole_tag_table_matches_its_committed_inventory
```

Fails at `crates/pncad-py/src/tests.rs:2629` with
`value(s) ADDED ["measure_clearance_refused", "measure_selection_kind"]`.
It reads source text and compares strings, so it is independent of ε and
of the compile mode — it reds every lane and every tolerance row, which
is what it did on both shard-2 legs of #1732's run.
