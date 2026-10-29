---
id: intent-stage3-is-built
kind: issue
title: INTENT stage 3 (spaces and placement) is built: poses defined, placements owning mates and values, the world node, constructions placed, the per-operation computing frame, overconstraint refusing, repetition by index
status: closed
opened: 2026-10-08
priority: P0
cost: H
blocked_on: []
closed: 2026-10-29
---

The release trigger for the rows that wait on INTENT stage 3 and on nothing later (`work/intent/plan.md`, stage 3; `docs/INTENT-STAGE3-SPEC.md`). Those rows park with `blocked_on: [intent-stage3-is-built]` instead of on the whole-program hold `d10-one-way-to-say-intent-is-unbuilt`, so they are released as soon as this stage lands (the 2026-10-08 re-homing, `work/intent/log.md`).

The stage is these units, after the rulings FORK-S3P, S3O, S3M, PAT and MOVE (fork log rows 95, 96, 97, 99 and 149) and PR 5494 (rows 189–191):

- A `poses-are-variables`;
- A′ `poses-move-by-lengths-and-angles`;
- B `a-placement-is-the-bundle-of-mates`;
- C `a-mate-relates-two-poses`;
- D `transform-retires-into-a-placement`;
- D′ `a-placement-pins-by-written-values-without-a-mate`, built as D′1 `a-placement-names-its-target-space`, D′2 `a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space` and D′3 `assemble-is-an-operator-and-the-product-is-a-slot`;
- E `an-operation-computes-in-a-frame-of-its-reads` (its rider merged as PR 5547);
- F `a-mate-on-a-pinned-copy-refuses`;
- G `patterns-are-index-variables`.

A, A′, B, C, D and F have merged. This row closes when D′, E and G have. When it closes, each row parked on it is re-read against the built code: closed if its code is gone, opened if its scene still stands.
