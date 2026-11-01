---
id: blend-reads-its-chain-convexity-off-the-link
kind: issue
title: Blend decides its chain convexity with its own triple-product predicate and lever, the same question topo's link answers
status: open
opened: 2026-10-26
priority: P1
cost: M
blocked_on: [material-and-convexity-are-read-three-ways]
---


`crates/sweep/src/blend/battery.rs` `convexity_at` (`fillet3_convexity_sign`) decides an edge's convexity as the sign of `(n₊ × n₋)·τ̂`, with its own lever. That is the predicate `Gap`'s `gap_edge_convexity` decided. ATREST's `material-and-convexity-are-read-three-ways` (decided 2026-10-26 on a converged designer pair) makes `topo::link` the one reader of a cell's local material, and it retires `Gap`'s copy. Blend should ask the link: its turn at the edge is one `metric::Distance` reading of the minus sector's real point against the plus sector's carrier, and a tangent edge drops to the second-order pairing and κ_rel layer. Then blend carries no private lever.

Filed by the ATREST orchestrator. It is parked on the link build. Read `work/atrest/material-and-convexity-are-read-three-ways.md` `## Decided` first.
