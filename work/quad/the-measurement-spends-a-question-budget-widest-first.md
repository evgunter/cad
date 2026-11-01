---
id: the-measurement-spends-a-question-budget-widest-first
kind: unit
title: build [tolerance.measure] as ruled on PR 6102: a caller-stated budget per question, widest-first refinement across the question's cells, resolution over the whole surface, a fixed constant for the kernel's own questions
status: open
opened: 2026-10-31
priority: P1
cost: H
refs: [the-measurement-work-bound-attaches-to-the-question, quad-a-faces-round-zero-is-outside-the-wall, 6102, 6092]
---


## What

DESIGN `[tolerance.measure]` as ruled on `[ev]` PR 6102. Today `Run` and the walks (`topo::props` `walk_set`/`deciding_walk`, the at-rest volume-sign walk, the boolean's refine gate) give each face a uniform round per step under one per-face `FACE_WORK_WALL`. Owed:
- **One budget per question.** The caller states it with its predicate, as a multiple of reading the measured surface once (round zero, counted in cell-work units only, so it does not depend on how the surface is cut). No default at the public doors, Rust and Python alike. The kernel's own questions (at-rest gate, boolean decisions, document assertions) state one fixed constant per question; recalibrate it from the corpus, since the 162 torus tiles spend about 39e6 units to meet 1 µm, against today's per-face 14.25e6.
- **Widest-first.** Each step refines the span cells whose next round removes the most width for its cost, across every face in the question, in a deterministic batch, breaking ties by a cell's place on its carrier, never by face. A whole patch and its tiles then spend the same work and stop at the same enclosure, met or not.
- **Resolution over the whole surface.** Today `Settle::Resolution`/`measure_to`'s `Met` is read per face, which is cut-dependent: a tile must meet it alone, while the whole face meets it on average.
- **Round zero inside the budget.** Covered by `quad-a-faces-round-zero-is-outside-the-wall`, which carries the measurements.

## Acceptance

- **The 9×9 conic torus:** whole and as 162 tiles, it stops at the same enclosure for the same work, both at the default target and at a target it cannot meet.
- **The 12+12-knot `cert5_r2` rows at 1e-12:** they finish within the gate's constant.
- **Docs updated:** `run.rs` "fewer rounds the more each round costs", and the PR 5589 witnesses.

## Review

Dual (H, architectural).

Signed: (QUAD orchestrator)
