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
needs_ev: true
---

INTENT stage 3, PR E. Spec: `docs/INTENT-STAGE3-SPEC.md` §6.

Each space computes in the frame of its earliest member, a seed or a copy in document order, never the world (FORK-S3-6, D9). `SolvedPoses::world_of(space)` is the one door that composes the world's map: export and the viewer's display read it, and a grep gate holds that nothing else does.

Product digests move into computing-frame coordinates, and STEP bytes move by rounding. Test 15 checks that each moved digest moved by its world map alone. An edit to the world mate moves no body bit.

FORK-S3-6 was weighed with FORK-S3-1 as FORK-S3P (fork log row 95) and
went to Ev in an `[ev]` PR; this unit builds on the answer
provisionally, and it changes the unit: nothing computes in the frame of
a space. After Ev's principle on #4323 (do not hide
arbitrariness), each operation computes in its first operand's
construction frame as the author lists it, never by mint order. Operands
on one free frame compute in it with nothing chosen. The walk stops at a
copy. The at-rest census is defined order-free: each pair's verdict is
the same in either member's frame, or the band refuses, and a test pins
that both agree. The frame is a function of the reads and is keyed in the
memo with them. A new mate relating two spaces moves no computed bit,
only the world map export composes. Spec tests 15–17 need
restating (a copy's body digest equals its source's; a re-mint, not a
reorder).
