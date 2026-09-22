---
id: SYM-13
kind: unit
title: the leaf receipt's frozen column is the leaf's NEED, not its work — schedule-independent by construction
status: dispatched
opened: 2026-09-22
priority: P0
cost: D
branch: sym/13-leaf-need
refs: [leaf-frozen-column-is-schedule-dependent-under-the-drive-memo]
---

## What

Under the drive-scoped plain memo a leaf receipt's `frozen` column
records which leaf got to a node first — a property of the schedule,
on a receipt whose every other column is not. The unit measures the
race (builds a document where two leaves race for a node and shows the
cross-schedule row red today), the column on both documents under
three schedules, the consumers, and NEED's cost; then makes the column
the leaf's NEED — the distinct nodes of the drive's frozen set this
leaf's walks reached — schedule-independent by construction, with the
adversary as a gating row and the drive's column and the serialized
receipt unmoved. Block SYM-B3 slot 2 (D / STRUCTURAL, pre-draw; arm
OPUS); protocol v7 IN, the full v6 dual. Spec: `docs/SYM-13-SPEC.md`.
