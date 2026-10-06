---
id: JOIN-2
kind: unit
title: The declared-REST zip reads the join's matched segments, by locus, instead of enumerating its own
status: closed
opened: 2026-10-02
priority: P1
cost: H
branch: join/2-zip-reads-segments
refs: [JOIN-1, rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity, dumbbell-joint-union-leaves-four-loose-ends]
pr: 3880
closed: 2026-10-03
---

Spec: `docs/JOIN-2-SPEC.md`.

## Closed 2026-10-03 — PR 3880

Merged after a dual review, two fix passes and a delta review (`work/join/log.md`). The declared-REST zip reads `join::section_segments`; its own enumeration, the straight-chord facing test and `ParallelSeamEdges` are gone.
