---
id: rounded-stack-subtract-and-intersect-refuse-fallback-extent
kind: issue
title: The rounded two-plate stack's subtract and intersect refuse FallbackExtentUnsupported with every finding declared
status: open
opened: 2026-10-01
priority: P3
cost: M
---

Found by the review of PR 3657, measured on `d2d5b09076`.

## Repro

The rounded two-plate stack (`crates/sweep/tests/reach_continuation.rs`:
6 × 4 plates, corner fillets r = 0.5, z 0 to 1 and 1 to 2), every
finding declared (the mating plane `Rest`, eight continuations):

- union: builds (14 faces, 4 recorded curved skips);
- subtract P − Q and intersect P ∩ Q: refuse
  `FallbackExtentUnsupported` (`crates/topo/src/boolean/ops.rs:1124`
  and its siblings at `:2323` and `:2382`).

The sharp stack's subtract and intersect build with the same
declarations. Whether this is the fallback-extent lane meeting the
coaxial fillet pair (two cylinders on one carrier, touching along the
mating arc) has not been checked.
