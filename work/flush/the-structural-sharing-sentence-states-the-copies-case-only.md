---
id: the-structural-sharing-sentence-states-the-copies-case-only
kind: issue
title: Tier 3′'s structural-sharing sentence states the copies case only; value-decided Zero pairs share the kept key too (ruling B, ruling A)
status: closed
opened: 2026-10-12
design: true
priority: P3
cost: E
closed: 2026-10-14
pr: 4793
---


`docs/DESIGN.md` tier 3′ says: "Structural sharing (same surface or point key) needs no record: an op that cuts one vertex into copies hands each copy the original's point, as it hands a cut face's fragments one surface." Ev ratified it with fork-log row 45 (`[ev]` PR, split pinch halves, commit `73cda58102`).

Since then, two rulings by the FLUSH orchestrator have put vertices on one point key from a value decision:
- **Ruling B** (PR 4728): one key per pinch, with each grouping recorded as a `VertexFusion` row.
- **Ruling A** (fork-log row 133; `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys`): the same across an assembly's solids.

Neither changes a decision. The sentence now describes one source of a shared key out of two. The census already reads both alike. The `[ev]` PR that carries this row restates the sentence.

## Ruling

Ev approved the restated sentence on PR 4793 (2026-10-14: "this looks good!"). It lands in `docs/DESIGN.md` with that PR, and fork-log row 133 records the answer.
