---
id: intent-stage3-is-built
kind: issue
title: INTENT stage 3 (spaces and placement) is built: poses defined, placements owning mates and values, the world node, constructions placed, the per-operation computing frame, overconstraint refusing, repetition by index
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [poses-are-variables, a-placement-is-the-bundle-of-mates, a-mate-relates-two-poses, transform-retires-into-a-placement, an-operation-computes-in-a-frame-of-its-reads, a-mate-on-a-pinned-copy-refuses, patterns-are-index-variables]
---

The release trigger for the rows that wait on INTENT stage 3 and on nothing later (`work/intent/plan.md`, stage 3; `docs/INTENT-STAGE3-SPEC.md`). Those rows park with `blocked_on: [intent-stage3-is-built]` instead of on the whole-program hold `d10-one-way-to-say-intent-is-unbuilt`, so they are released as soon as this stage lands (the 2026-10-08 re-homing, `work/intent/log.md`).

The stage is seven units, after the rulings FORK-S3P, S3O, S3M and PAT (fork log rows 95, 96, 97 and 99):

- A `poses-are-variables`;
- B `a-placement-is-the-bundle-of-mates`;
- C `a-mate-relates-two-poses`;
- D `transform-retires-into-a-placement`;
- E `an-operation-computes-in-a-frame-of-its-reads`;
- F `a-mate-on-a-pinned-copy-refuses`;
- G `patterns-are-index-variables`.

This row closes when all seven have merged. When it closes, each row parked on it is re-read against the built code: closed if its code is gone, opened if its scene still stands.
