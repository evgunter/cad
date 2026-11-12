---
id: a-mixed-rims-mate-with-a-ring-refuses-at-the-mate-gate
kind: issue
title: A mixed rim whose mate carries a ring refuses at the mate gate
status: open
opened: 2026-11-04
priority: P1
cost: M
refs: [a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses]
---

Filed by the band-mixchain lane (2026-11-04), from review lane band-chain-r2's Q6 on PR 6520.

`surgery::resolve_mixed_rim` (`crates/sweep/src/blend/surgery.rs`) refuses `UnsupportedChain` "a mixed rim's mate support carries rings of its own" whenever a link's mate face has a ring, even one well clear of the strip.

Witness: a cross-drilled plate.
- The rounded rectangle `[−1, 1] × [−0.6, 0.6]`, corner radius 0.3, 0.5 thick.
- A radius-0.08 bore runs through both long side walls at height 0.2.
- Its top rim, filleted at 0.05, refuses at that gate. The bore's rings on the walls lie 0.25 below the trim.
- The review's probe is `r2_cross_drilled_plate`.

Sequential calls do not help, because the ring is on the wall either way. Filleting the rim before drilling builds.

`support_boundary_clearance` already meters every ring of every support against its link's strip. What is unmeasured is the carve's move: whether the mate trim's `mef` leaves a ring on the shrunk mate face, or the ring needs moving there (as `lone_host_trim` moves one). Measure that, then drop the gate.
