---
id: a-boolean-coincidence-names-a-minted-vertex-by-its-input-edge
kind: issue
title: The boolean's decision on a minted vertex names the input edge it was cut from, so the lint's report points at the operand rather than at the tool faces that made the coincidence
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

`input_cell` (`crates/topo/src/boolean/ops.rs`) maps a vertex the
reduction minted to the input edge it was minted on. So the boolean's
decision that two minted vertices coincide (two tool faces cutting two
kissing edges at heights `a = b`) names the two kissing edges, the same
cells as the carried edge–edge record, and the `unproven-coincidence`
lint's report points at the operand, not at the two tool faces whose
values made the coincidence. If the lint replays the margin symbolically
it still sees `a` and `b` and only the report is poor; if it reads the
named cells, it misses the coincidence. Name a minted vertex as its
(edge, tool face) pair. From the record-carry fork (PR 4749), read from
the code, not measured.
