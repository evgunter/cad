---
id: the-product-list-is-kept-by-a-default-at-the-edit
kind: issue
title: Stage 2 FORK-2: how the explicit product list is kept under edits (A10's maintenance clause)
status: open
opened: 2026-10-07
priority: P0
cost: E
needs_ev: true
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

Stage 2 replaces A10's product rule (the root set is the sink set of the consuming graph) with D10's explicit list. Under D10 nothing consumes anything, so A10's maintenance clause ("a new sink appends; an insert consuming roots takes the first one's place; a delete restores the orphaned inputs") has no graph meaning. Raised as FORK-2 by the stage-2 spec (`docs/INTENT-STAGE2-SPEC.md` §11, PR 4216). It blocks unit C's (`the-product-is-an-explicit-list`) ratified text.

A designer pair converged after four rounds: one default applied at the edit, and `SetProduct` as the one override. The question and both reports are in the `[ev]` PR. Ev's answer closes this row and writes A10.

