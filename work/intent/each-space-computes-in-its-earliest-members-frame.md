---
id: each-space-computes-in-its-earliest-members-frame
kind: issue
title: D10 stage 3 PR E: each space computes in the frame of its earliest member (D9); export composes the world's map and nothing else reads it
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [a-placement-is-the-bundle-of-mates]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR E. Spec: `docs/INTENT-STAGE3-SPEC.md` §6.

Each space computes in the frame of its earliest member, a seed or a copy in document order, never the world (FORK-S3-6, D9). `SolvedPoses::world_of(space)` is the one door that composes the world's map: export and the viewer's display read it, and a grep gate holds that nothing else does.

Product digests move into computing-frame coordinates, and STEP bytes move by rounding. Test 15 checks that each moved digest moved by its world map alone. An edit to the world mate moves no body bit.
