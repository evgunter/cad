---
id: a-prism-bridging-two-wedges-about-a-contact-line-refuses-shared-rim
kind: issue
title: A prism bridging two wedges about a contact line refuses Naming(SharedRim NotAdjacent) on a body topo builds sound
status: open
opened: 2026-10-08
priority: P2
cost: M
---

## What

Found by the sweep of `three-solids-touching-along-one-line-refuse-their-union`
(program tang), measured on 8fd03fce and again on its branch: the same
orders refuse on both, so the topo change does not reach them.

The plate `[0,3] × [0,2] × [0,1]` and three upright prisms over
triangles from (1.5, 1) to the points 0.4 from it at the bearings
0°–50° (z 0.5–2.0), 120°–170° (z 0.47–1.7) and 30°–140° (z 0.44–1.81).
The first two touch along the vertical line through (1.5, 1); the
third bridges the gap between their wedges, crossing both.

Through editor-core's union (`crates/editor-core/tests/union_pinch_member_order.rs`'s
`sector_prism`, `crate::fixture::union_over`), the member orders
[2, 1, 3, 0] and [1, 2, 3, 0] (0 = the plate), and [1, 0, 2] and
[0, 1, 2] for the prisms alone, refuse `Naming(SharedRim { found:
NotAdjacent, .. })` (`names/emit.rs`, the `[] => Rim::NotOne(RimShare::NotAdjacent)`
arm). Topo builds those orders sound
(`crates/topo/tests/three_solids_on_one_line.rs`,
`a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`,
"both wedges crossed": 3/3′, closed-form volume, the analytic
oracle). The orders folding the bridging prism last refuse in topo
first (`work/tang/an-edge-crossing-two-wedges-about-a-contact-line-refuses.md`);
the other 16 (4) build through editor-core.

Unmeasured: which face pair the refusal names and why it shares no
edge. The refusing orders are those that fold the bridging prism
onto the two touching ones.

