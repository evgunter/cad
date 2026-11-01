---
id: tessellation-is-serial-per-face
kind: issue
title: the tessellator visits faces one at a time although every lane is read-only per face
status: closed
opened: 2026-09-12
parent: PERF-7
closed: 2026-09-13
---


## The finding

`mesh::tessellate` (`crates/mesh/src/tessellate.rs`, `tessellate_impl`)
runs each face's lane in a serial `for (fk, face) in body.faces()`;
since PERF-3 each lane returns a `Patch` of local ids and the
sequential `place` fold rebases it in arena order, so the loop's body
is already the map half of D9's idiom 1 with the fold half in place.
The lanes are the cost on every large document (PERF-5's tables:
`tube_ring` 1.7 s of a 2.4 s index build, `gallery_ring` 0.3 of 0.44 s,
and every first open of a ring document). `plan.md` §2.2 names it the
cheapest of the four unbuilt parallel targets.

## What a fix is

Idiom 1 over faces into a pre-sized buffer, the existing arena-order
fold, bit-identical at any thread count; the one shared mutable
(`FaceBounds`) split so each face owns its entry; the patch memo's
inserts moved into the fold. `docs/PERF-7-SPEC.md`.
