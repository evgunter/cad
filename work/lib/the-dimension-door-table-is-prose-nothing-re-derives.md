---
id: the-dimension-door-table-is-prose-nothing-re-derives
kind: issue
title: The six-door DimensionError table is a prose anchor nothing re-derives, and its count was wrong three ways before it existed
status: open
opened: 2026-09-23
priority: P3
cost: E
refs: [2702]
---


Filed by the PORT orchestrator at PORT-DIMS-1's merge (#2702), named by
that unit's own row in `ERRORS_MINTING_ITEMS`
(`crates/pncad-py/src/tests.rs`), which records the holder as *nothing*.

## The finding

`crates/pncad-py/src/errors.rs`'s `DIMENSION_DOORS` is a
`#[doc(hidden)] pub const … : () = ()` — a documentation anchor whose
entire content is a six-row table saying where the document layer's
`DimensionError` reaches Python, under which class, and in which
attribute. It is good documentation and it is the **one home** for a
fact that previously had none.

**Nothing re-derives it.** The table is prose; the census that covers
`errors.rs` counts literals in code and correctly reports this item as
holding none, so a door added or moved leaves the table silently wrong.

## Why that is worth a row rather than a shrug

This exact fact has already been wrong, three ways at once. Before
#2702 the door count was stated in four places with two different
numbers — `errors.rs` said four doors under three class names,
`py/mod.rs` said three, `test_binding_census.py` said three, and
`pncad.pyi` implied four — and the real number was six. A reviewer found
it by enumerating the routes by hand. The table is the repair; it is
also a fifth statement, and the only thing standing between it and the
same rot is that it is now the only one.

The table's own rows are mechanically checkable in principle: each names
a site that mints `crate::tags::expr_dimension_error_tag`'s word into a
named Python attribute, and that is a property of the tree. A census
that walked those sites and compared them against the table would close
this. Whether that is worth building is LIB's call.

## Related, and not the same

The table itself names two open rows it deliberately does not decide:
`work/lib/literalerror-publishes-its-tag-under-two-names.md` (the third
row says `variant` where its two siblings say `kind`) and
`work/lib/persist-inner-variant-stops-one-rung-above-the-check.md` (a
seventh route, a replayed edit at `load`, that stops one rung above the
check). Both are about what the table *says*; this row is about the fact
that nothing checks it.
