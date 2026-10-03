---
id: tessellator-panics-on-a-self-slit-face-every-validator-passes
kind: issue
title: the tessellator panics (chord-census debug_assert, live in release) on a plane face carrying a two-edge self-slit ring that tiers 1-3 and 3' all accept
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Measured 2026-10-03 (CLEAVE's zero-area-ring measurement for the edgeless-contact fork, origin/main `a6a636fdd`, f64). A plate [0,3]²×[0,1] whose top face carries a ring of two coincident antiparallel line edges P=(1.5,1,1)–Q=(1.5,2,1), each edge with the top face on BOTH sides (built: strut, `kemr`, `mev(Lone)`, `mef_chord`, `kfmrh(top, membrane)`; v10 e14 f6 r2 s1). Tiers 1, 2, 3, 3′ and `gate_at_rest_kept`/`_declared` all pass it. `tessellate` panics at `crates/mesh/src/tessellate.rs:597`: "chord segment Some((8, 9)) is an edge of 2 face triangles, not two per chord carrying it" — a `debug_assert` that is live in release (the workspace's release profile keeps debug-assertions on). A consumer panicking on a body every validator accepts is a kernel defect regardless of whether the body should be legal: either the mesh handles it or a validator refuses it (`validators-accept-a-lamina-slit-or-zero-area-membrane-face`). The doubled-slit form (each edge shared with a distinct wall) meshes correctly. The throwaway harness is not committed; rebuild it from the recipe above.
