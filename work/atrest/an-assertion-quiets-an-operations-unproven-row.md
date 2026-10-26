---
id: an-assertion-quiets-an-operations-unproven-row
kind: unit
title: An assertion at its site quiets an operation's unproven row (a boolean's glue, a split's ON verdict, a blend's), by the contact half's cell rule
status: open
opened: 2026-10-26
priority: P0
cost: M
refs: [the-at-rest-census-is-a-check]
---

DESIGN `[intent.checked]` says of every unproven coincidence that "an assertion at its site quiets it". Stage 5 C quiets only the at-rest rows of each `⊔`. Its contact half is written over a row's two cells (`docs/INTENT-STAGE5-SPEC.md` §4, "The contact half of the quieting rule", and §11 Q1), so this unit applies the same rule to the rows an operation decides. It adds no second rule.

**Where an operation row's cells are named.** A `NamedCoincidence` (`crates/editor-core/src/coincide.rs`) names each cell as a `NamedCell`:
- `Entity { input, name }` is an entity of an input body of the deciding node, keyed by the read it entered through and its name in that read's table;
- `Tool { input, .. }` is a cutting plane's tool read, which is a pose and not a cell of any body.

The rows sit on each deciding node's value (`NodeValue::coincidences`): a boolean's glue, a split's ON verdict, a blend's. The lint reads them in evaluation order (`checks.rs`, `door_rows`).

**What selecting them takes.**
- An `Entity` cell is selectable when a `Face` selection of the input read's `Body` names it (`[operations]`, *Selections*). A `Gap = 0` over two such selections, in the deciding node's space, matches the row as the at-rest rule matches its cells.
- Often a cell is merged away in the result (two glued caps), but it is still named on the operand, which is a first-class value (`[operations]`, "Reading is the only dependency"). Confirm that the selection resolves against the input read and not the result.
- A `Tool` cell is not a body's cell, so no `Gap` reads it. Decide whether such a row is quietable at all, and if it is, by what measure. If no measure can quiet it, the row is reported loud and never refuses (CHECKS DS6).
- Cells below a face wait on `gap-reads-vertex-and-edge-cells`.

Done when a boolean's unproven glue, a split's unproven ON row and a blend's unproven row are each quiet under a holding `Gap = 0` over their two cells, and loud under one over any other pair. After this lands, the lint's knob is re-read against CHECKS DS3 (§11 Q4).

## 2026-10-26 — opened: its trigger fired

Stage 5 C landed the contact half over a row's two cells
(`checks::at_rest::quiet_contact`), read by the lint's `door_rows` for the
at-rest rows only; an operation's rows carry `None` there. This row
applies the same rule to them once their cells can be selected.
