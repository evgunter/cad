---
id: props-header-says-no-door-between-create-or-replace-and-value
kind: issue
title: props.rs's parameter-panel header still says no door sits between create-or-replace and value-only
status: open
opened: 2026-09-16
priority: P4
cost: E
---


## Finding

`crates/viewer/src/props.rs:44-56` (module header, "What a parameter has
no door for is CHANGING that notation") states a premise that two
shipped kernel doors have since falsified, and the panel's behaviour
still follows the stale sentence:

> A parameter's unit rides beside its `Distribution`, and the edit
> vocabulary offers create-or-replace (`DocEdit::SetDocParam`, which
> would drop the annotation) or value-only (`DocEdit::SetDocParamValue`,
> which leaves the unit alone, deliberately) — **and nothing in
> between**.

Two things sit in between today:

- `DocEdit::SetDocParamUnit` through `DocParam::with_display_unit`
  (PR #2732) — the notation door, which writes the unit and carries the
  value and the annotation forward.
- `DocEdit::SetDocParamDistribution` through
  `DocParam::with_distribution` (PR #2774) — the annotation door, which
  writes the distribution and carries the notation forward.

The consequence the header draws from the stale premise is the part
that matters:

> * no unit picker on a parameter row, and no way to add one **until
>   the kernel door exists**
>   (`work/issues/doc-param-unit-edit-has-no-door.md`);

The kernel door exists. The trigger this row is written to wait on has
fired, so the panel can offer a unit picker on a parameter row now, and
the sentence that says it cannot is the only thing recording that it
could not. The cited path is also dead — the item lives at
`work/edit/doc-param-unit-edit-has-no-door.md` — but that half is
already the class filed as
`work/issues/dead-work-citations-from-shipped-code-and-docs.md`; this
row is about the READING, not the path.

## What is needed

Re-read the header against `edit.rs`'s four `DocParam` doors
(`SetDocParam`, `SetDocParamValue`, `SetDocParamUnit`,
`SetDocParamDistribution`) and decide what the parameter panel offers
now that a notation edit and an annotation edit both exist. Either the
picker lands, or the header says why the panel still does not want one —
but not by asserting a door that shipped does not exist.

## Home

`work/chrome/` — `crates/viewer/src/props.rs` is CHROME's and VIEW's
(`work.py territory`); filed on CHROME's slate as the panel-shape owner.

## Found by

The style review of EDIT's `doc-param-distribution-edit-has-no-door`
(PR #2774, review lane `distrib-rv`), sweeping Q4 for prose that cited
the premise the new door removes.
