---
id: a-split-plane-built-through-a-vertex-is-unproven-at-rung-one
kind: issue
title: A split whose plane is constructed through an operand vertex leaves that vertex's ON row unproven: rung one reads the tool only for the split's own section cells
status: open
opened: 2026-10-30
priority: P2
cost: M
---


Found by the designer pair on `atrest/a-gap-over-adjacent-cells-states-nothing-of-the-row` (2026-10-30, `[ev]` #6086), and checked by both in reconciliation.

**The gap.** The answer on #6086 makes a split's ON row (a vertex on the cutting plane) a construction fact: read the split's plane through the vertex. The door doesn't prove it yet.
- `coincide` `plane()` maps a plane that is a pose not read off a face to `Origin::Pose(var, role)`.
- `lies_on` relates a cell to its tool only through `on_its_tool`, which covers the split's own section and crossing cells.

So a tool plane constructed `Through` an operand vertex leaves that vertex's `SplitOn` row unproven at rung one. Rung two might prove it at `Sym`; that is unchecked.

**Done when** a split whose plane is built through an operand vertex reports that vertex's ON row `Structural`, and a plane typed at the vertex's current value stays unproven.
