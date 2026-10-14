---
id: an-operation-computes-in-a-frame-of-its-reads
kind: issue
title: D10 stage 3 PR E: an operation computes in a frame that is a function of its reads, never the world's (D9); minted reference directions follow the inputs; export alone composes the world's map
status: parked
opened: 2026-10-09
priority: P0
cost: M
blocked_on: [transform-retires-into-a-placement]
refs: [intent-stage3-is-built, each-space-computes-in-its-earliest-members-frame]
---

INTENT stage 3, PR E. Spec: `docs/INTENT-STAGE3-SPEC.md` §6. Built on FORK-S3P (fork log row 95, PR 4324), which with FORK-S3O (row 96) supersedes "the kernel computes each space in the frame of its earliest member"; this row replaces `each-space-computes-in-its-earliest-members-frame`, closed for it.

An operation computes in a frame that is a function of what it reads and of nothing else, chosen for conditioning near the geometry it builds, never the world's, and keyed with its inputs (D9). Where conditioning does not decide, a construction computes in its own coordinates and an operation over copies in its first listed operand's. A value-informed re-centring (the operands' bounds snapped to a power-of-two grid) is allowed and measured on the far-from-origin rows.

The frame is no part of meaning: the body up to the rigid map, its names and every verdict outside the sliver band agree in any two frames, which test 13 pins by computing in two frames and comparing. So a minted reference direction is a function of the inputs (D10), and E re-derives the three sites `a-minted-reference-direction-follows-the-computing-axes` names. The at-rest census is order-free.

`SolvedPoses::world_of` is read by export alone, and a grep gate holds it; the viewer draws a space from display state no logic reads. Product digests move into computing-frame coordinates and STEP bytes by rounding, each checked against the world's map.

Waits on D (`transform-retires-into-a-placement`): before it, a construction computes in the world coordinates its absolute datum names.
