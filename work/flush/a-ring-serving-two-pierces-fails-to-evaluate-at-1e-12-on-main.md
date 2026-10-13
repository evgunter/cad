---
id: a-ring-serving-two-pierces-fails-to-evaluate-at-1e-12-on-main
kind: issue
title: Main is red at ε = 1e-12 on editor-core coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven: a node fails to evaluate
status: closed
closed: 2026-10-13
opened: 2026-10-12
priority: P1
cost: M
---


Found by CLEAVE: PR 4782's CI ran its ε = 1e-12 leg, and CLEAVE then reproduced the failure on clean main.

```
CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core \
  -E 'test(=coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven)'
```

On main at `ba47521813`, freshly compiled, it panics at `crates/editor-core/tests/coincidence_door.rs` (the `rows` helper, ~L44: `"{node:?} evaluated: {failure}"`). So a recipe node in the fixture fails to evaluate at 1e-12. At the default ε it passes.

The test arrived with PR 4728 (FLUSH's pinch cluster, `e564d25b23`, "record a ring's value grouping as a VertexFusion"). Its fixture reads a 1e-13 vertex gap. At ε = 1e-12 that gap is a tenth of the band rather than far below it, so the fixture likely sits in band at that row.

Owed: decide whether the fixture should hold at every ε row (rescale its gap to the run's ε) or whether the node's failure at 1e-12 is a kernel answer to fix. Nightly runs the 1e-12 row, and so does every PR that touches editor-core's ε-sensitive crates, so it reds them.

## Closed (2026-10-13, by TANG's fix PR on branch `tang/flush-ring-row-1e-12`)

The fixture was at fault, not the kernel. At ε 1e-12 the blocks' union (`ring_pinch`'s Union node) refuses `Escalated { Coincidence(Sectors, Moot) }`: the 1e-13 gap reads as ≈1.67e-12, which is in the escalate band [1e-12, 1e-11]. `common` and `cut` are then `Poisoned` through it. The "evaluated: None" came from the test helper, because `blocks::failure` maps `Poisoned` to `None`.

The row never passed at 1e-12: it fails identically at its introducing commit, e564d25b2 (PR 4728).

The fix is `pierce_gap()` = min(1e-13, ε/100). It leaves the default and 1e-6 rows building the same fixture. The test's `rows` now names a poisoned node's root failure.

Possible follow-up for FLUSH: `blocks::failure` returns `None` for `Poisoned` in every caller.

