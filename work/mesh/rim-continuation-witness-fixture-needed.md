---
id: rim-continuation-witness-fixture-needed
kind: issue
title: "coherence: the rim-continuation condition has no natively constructible witness — commit a fixture that reaches it through tessellate"
status: open
opened: 2026-09-02
github: 1588
refs: [1585, 868, MESH-12, coherence-findings-have-no-consumer]
---

## From GitHub issue 1588

Opened 2026-09-02; 0 comments.

**Filed from MESH-8 (PR [#1585](https://github.com/evgunter/cad/pull/1585)) as the schedule for a disclosed forward observation.**

MESH-8 measured that the old rim-continuation `debug_assert` in `mesh::walk` was **unreachable through `tessellate` on any natively constructible body**: a v-gap between two edges of one rim row forces one carrier `sqrt(εR)` off the surface, and MESH-7's shape door (`props_rim_*`) refuses the face before the walk runs. The relocated condition (`CoherenceCondition::RimContinuation`) fires on a synthetic (two circles at 1024 ε) and is quiet at c = 0, but no in-tree body reaches it through the public door, so the corpus row that would pin "quiet on everything that meshes" cannot include a rim-continuation positive.

**Owed:** a committed fixture that reaches the condition end to end — most likely a STEP file whose rim is stated as two circle arcs at slightly different levels within the props band but over the coherence band (the import route, where such data actually arrives), or an Euler-door body if one can pass the shape door — plus the corpus row it enables. Both reviews confirmed the unreachability reading; neither found the fixture.

Refs #868, MESH-8, issue 1587-class consumers.

## Home

`work/mesh/` — the condition lives in `crates/topo/src/coherence.rs`, an S-MESH territory glob, and MESH-12's live slate explicitly carries the rim-continuation witness.
