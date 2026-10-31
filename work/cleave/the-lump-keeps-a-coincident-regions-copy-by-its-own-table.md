---
id: the-lump-keeps-a-coincident-regions-copy-by-its-own-table
kind: issue
title: The Eq. 15.3 lump keeps operand A's copy of a coincident region by its own table, not by the glue's kept-side decision
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Found by the sweep of `the-kept-side-of-a-glued-pair-is-chosen-in-three-places`
for other places that choose which description a glued pair keeps.

## What

Where a coincident sector pair's region is kept once, the Eq. 15.3
lump keeps operand A's copy (`tables::eq15_3_lump`, `tables::kept_copy`
in crates/topo/src/boolean/tables.rs; `shell_witness::on_kept` and
`ops::fallback_fates` read the same rule, and the latter refuses a cut
in A's clone on the strength of it). That is the copy whose
description the overlap region carries into the merge, so it is the
same choice as the merged face's kept description, made by its own
table rather than read from `boolean::glue::kept_first` (DESIGN.md
`[one-way.places]`: "Which description a glued pair keeps | decided
once, at the glue").

Today the two agree (both A's), so nothing builds wrong; the defect is
the second place, which a change to the decision would leave behind.

## Owed

The lump table states its kept copy as the glue's kept side
(`kept_first`), with the ∪ / ∩ / ∖ rows derived for that side, and the
readers that assume A's (`fallback_fates`, the
`the_lump_keeps_operand_as_copy_where_it_keeps_one` row) read it too.
