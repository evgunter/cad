---
id: a-germ-planes-normal-is-levered-from-its-stored-origin
kind: issue
title: boolean::join levers a germ plane's normal length from the plane's stored origin, where its comment claims a bound from below
status: open
opened: 2026-11-20
priority: P2
cost: M
---

`boolean::join`'s `bool_connect` (`crates/topo/src/boolean/join.rs`, the `germ_normal` closure) decides a germ plane's normal has a length with `UnitVec3::levered(n, BOOL_GERM_PLANE_NORMAL, band, reach.lever_from(origin))`. The comment above it says the arm is "a LOWER bound on the reach it is consumed over: the ball through the two germ sites this join connects, read from the plane's origin".

The pivot is the point the caller hands. In `plane_of` (the planar kind arm) that is the plane carrier's stored `origin`, which can be anywhere on the plane. A plane stored far from the germ sites gets an arm as long as that distance. That is neither a bound from below on the section nor a read at a pivot derived from it (`crates/geom-core/README.md`, "Float rounding", the consumed-extent bullet). `UnitVec3::levered` decides `|n|·arm` positive, so a longer arm admits a shorter normal. That is the direction the comment's bound from below exists to prevent: a degenerate normal is read as a direction. The `EdgePlane` arms hand the conic's centre, which lies off the sites by the conic's size.

Found by `oneround/lever-sites-topo`, which left the ball itself (`germ_reach`) as it stands: its antipode needs `SectionLoop` to hold its conic. The fix is the pivot: the sites' ball read from its own centre, or the plane's origin re-anchored at the sites (`geom_brep::anchored_at`). Measure the change with a differential.

