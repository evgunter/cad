---
id: the-profile-editors-fields-carry-no-per-argument-affordances
kind: issue
title: The profile editor's number fields carry no per-argument affordances (expression, unit, range); a driven profile cannot be held
status: open
opened: 2026-09-19
priority: P1
cost: D
---


## What is there now

PR #2862 opens a committed profile in the add-profile form's own editor
(`pane::profile`, `ViewerBehavior::edit_profile_ui`). The editor's
number fields hold plain numbers (`profile::Step<f64>`). Three things
a profile argument can have are therefore reached through the node's
generic slot rows, which `pane::properties`' `feature_rows_ui` keeps
folded under the editor ("arguments"):

- driving the argument by an expression (`SessionOp::SetSlotExpression`),
- the unit one argument is written in (`SessionOp::SetSlotUnit`),
- the locally-valid range probe (`SessionOp::ProbeBounds`).

And a profile with any argument already driven by an expression does not
open in the editor at all: `sketch::held_loops` refuses it
(`HeldRefusal::Driven`), and the pane shows that refusal above the slot
rows.

## What the preferred shape is

(The review of #2862, M1.) The step list's own fields carry those three
affordances in the edit door: each field knows its `SlotId`
(`SlotId::Profile { loop_, step, arg }`, the address `Node::slots`
walks in the same order `path_step_fields` draws the arguments), offers
the expression/unit/range doors the slot row offers, and a driven
argument shows its expression with the number locked, so the editor
holds every stored profile and the folded slot rows go away. The held
state then needs an argument's expression beside its number, and
`sketch::program_edits` must leave a driven argument whose value has
not moved unwritten, which it already does by comparing values.

## 2026-09-29 — `program_edits` is gone (AUTH-6)

AUTH-6 (`author/profile-reshape`) retired `sketch::program_edits`. The
edit door now commits one `DocEdit::SetProgram`. The duty named above
belongs to `session.rs`'s `carry_unmoved` now: every argument of a
kept step that has not moved is written as the document holds it, so it
keeps its expression and its notation. A kept step's driven argument
that the program does not hold unmoved refuses `DrivenByExpression`
(`profile_edit::a_kept_driven_argument_is_not_written_over`). The held
state still has no expression beside a number, so this row's work is
unchanged.
