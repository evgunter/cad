---
id: seven-vector-families-is-a-prose-count-at-two-tag-sites
kind: issue
title: the seven vector families is an unforced prose count at two pncad-py tag sites
status: open
opened: 2026-09-12
---



Filed by DOOR's `vectorslot-all-has-no-reader` unit, which deleted the
kernel-side enumeration of the vector families and swept for anything
that would go stale when the enum grows. Filed here because
`crates/pncad-py/src/` is LIB's ground (`work.py territory`).

## The finding

Two doc comments in the tag surface state the number of vector slot
families as prose:

- `slot_id_tag` (`crates/pncad-py/src/tags.rs:223`) — *"The seven
  vector families spell their component into the word rather than
  beside it"*
- `slot_from_word` (`crates/pncad-py/src/slot_word.rs:41`) — *"The
  seven vector families spell their component INTO the word, exactly
  as the forward map does"*

Both are correct today (`VectorSlot` declares seven variants,
`crates/editor-core/src/node.rs`). Neither is forced. An eighth family
reds `VectorSlot::slot` and `VectorSlot::label` in `editor-core` and,
downstream, the exhaustive `match slot` in `slot_id_tag` — so the
author IS standing in `tags.rs` when they add one. That is the forced
VISIT, not a forced number: the sentence three lines above the match
keeps saying seven and nothing reds. `slot_word.rs`'s copy does not
even get the visit — `slot_from_word` matches on `&str` with a
fallthrough, and its guard is the by-test agreement with `slot_id_tag`,
which says nothing about a count in prose.

This is the class `tag-inventory-prose-counts-are-stale` closed on
2026-09-08 by deleting the aggregates and keeping the qualitative
claim. The same repair fits here and is smaller: the load-bearing half
of both sentences is *"a family spells its component into the word,
because the family name alone names three slots"*, which is true at any
count. Dropping the numeral costs the sentences nothing.

## Not fixed here

DOOR's unit is one PR, one row, on `editor-core` ground; these are two
files on LIB's.
