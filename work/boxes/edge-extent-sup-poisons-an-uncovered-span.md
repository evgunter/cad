---
id: edge-extent-sup-poisons-an-uncovered-span
kind: issue
title: edge_extent_sup's ellipse arm poisons an uncovered span where Reach::of_span refuses it typed
status: open
opened: 2026-11-15
priority: P3
cost: M
---


`geom_brep::edge_extent_sup` (`crates/geom-brep/src/extent.rs`, `edge_extent_sup` near :612) is total and propagates poison by contract. Its ellipse arm reads `cover_far` (near :413), which maps a carrier with no cover over the span to `poison()`. The parallel door `Reach::of_span` now returns a typed `Uncovered` for the same refusal (PR 6976). `ExtentBall::of_edge_from`, the other poison door over `cover_far`, had only a test caller and was deleted in that PR.

Two production callers read the extent: `topo::boolean::contact_verify` (near :337) and `topo::offset_derive::section_closed` (near :676). Making the extent typed means deciding what each caller does with an uncovered edge (pruning or granting). The spline's speed bound and control net poison the same way, so the choice covers the whole door, not only the ellipse arm. That is why it was left out of PR 6976.
