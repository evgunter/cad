---
id: an-operation-computes-in-a-frame-of-its-reads
kind: issue
title: D10 stage 3 PR E: an operation computes in a frame that is a function of its reads, never the world's (D9); a body value is Framed (its body shared by Arc, one map); an operation over operands in one space computes in its first operand's frame and reads the rest through relative; export and the viewer's drawing compose through one door
status: parked
opened: 2026-10-09
priority: P0
cost: H
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
refs: [intent-stage3-is-built, each-space-computes-in-its-earliest-members-frame]
---

**Spec:** `docs/INTENT-STAGE3-SPEC.md` §6, restated against main at `4a4bb23608`; where this body and §6 differ, §6 governs. No design question is open: the copies ruling (orchestrator, 2026-10-20, `work/intent/log.md`) answered §6 Q1, PR 5547 answered Q2 and built the rider, and PR 5494 (`[placement.world]`) answered Q3. E's core waits on D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`): its operation over copies needs each operand to be one body in one space, and it rewrites the value D′3 leaves `wire_place` defining.

INTENT stage 3, PR E. Built on FORK-S3P (fork log row 95, PR 4324), which with FORK-S3O (row 96) supersedes "the kernel computes each space in the frame of its earliest member"; this row replaces `each-space-computes-in-its-earliest-members-frame`, closed for it.

A body value is `Framed { body: Arc<Body>, frame: Own | Map(RigidMotion) }`: the body in its own coordinates, shared, and one map into its space's coordinates. A copy keeps its source's `Arc` under the composed map and is never rebuilt, and content keys are unchanged.

An operation computes in a frame that is a function of what it reads and of nothing else, chosen for conditioning near the geometry it builds, never the world's, and keyed with its inputs (D9). Every operator's operands share one space (D′1), so where conditioning does not decide an operation computes in its first listed operand's frame and reads every other operand through `relative`, the one door between two operands of one space; a construction computes in its own coordinates. A value-informed re-centring (the operands' bounds snapped to a power-of-two grid) is allowed and measured on the far-from-origin rows.

The frame is no part of meaning: the body up to the rigid map, its names and every verdict outside the sliver band agree in any two frames, which test 13 pins by computing in two frames and comparing. A minted reference direction is a function of the inputs: PR 5547 built that as E's rider (`a-minted-reference-direction-follows-the-computing-axes`, closed). The census at a `⊔` is order-free.

Export and the viewer's drawing compose a body into a space's coordinates through one door, `in_space`, that nothing else calls, and a gate script holds that. Product digests move into computing-frame coordinates and STEP bytes by rounding, each checked against `in_space`.

The carry (`a-placement-carries-its-sources-at-rest-result`) follows this unit, because both rewrite `wire_place`'s value; G follows it, because a pattern's members are `Framed` over one body.
