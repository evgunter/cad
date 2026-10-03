---
id: a-partly-applied-regauge-then-mate-list-declares-silently
kind: issue
title: regauge_then_mate's edit list is computed against the pre-regauge document; applying the insert alone, or first, inserts a DECLARING mate without a word
status: review
opened: 2026-10-02
priority: P3
cost: M
branch: recipe/regauge-then-mate-door
pr: 3925
---


Found by SHOW's `bench-on-a-gauge` (PR 3840) and its review.

## What

`regauge_then_mate` (`crates/editor-core/src/edit.rs`) returns a
`Vec<DocEdit>`: one `SetGauge` per member of `a`'s group, then the
mate's `InsertNode`. The list is computed against the document BEFORE
the re-gauge, and it is the caller's job to apply it whole and in
order. Nothing enforces that. Applied alone, or first, the insert
lands with the two instances on different gauges, and the edit door
accepts it as a DECLARING mate, with no refusal and no maintenance
row. The author meant a placing mate. Only the solve's
`role` or a later at-rest gate shows the difference, and only when
the declared contact happens not to hold.

The Python door (`crates/pncad-py/src/py/doc.rs`,
`Doc.regauge_then_mate`) applies the list itself, so Python callers
never see the hazard. Rust callers apply it by hand
(`demos/tour/src/assembly.rs`, `mate_onto`, gap-commented there).

## Shape of a fix (the owner's call)

A door that applies the compound action and returns the mate's id,
as Python's does. Or an edit the insert door can check (a "this
mate must place" flag that refuses a declaring result, typed, with
the regauge recourse).

## Built (2026-10-03, PR 3925)

Option (a): `regauge_then_mate` applies the whole action and answers one
`Applied` (the mate's id in `record.minted`); the edit list is gone, and a
`compile_fail` doctest pins that it cannot come back. Python's door and the
tour's `mate_onto` call it. PR 3925 has the choice, the sweep and the
filed VSEAM row.
