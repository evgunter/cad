---
id: nurbs-surface-aabb-is-a-second-spelling-of-the-net-cover
kind: issue
title: geom::surfaces::boxes::nurbs_surface_aabb has no production reader and spells the spline carrier's net cover's box a second time
status: open
opened: 2026-11-08
priority: P4
cost: E
refs: [cover-app-readers, cover-kind-match-gate]
---


Found by `cover-app-readers` (2026-11-08).

`crates/geom/src/surfaces/boxes.rs` `nurbs_surface_aabb` boxes a
NURBS surface's control net, with a poison screen
(`net::any_poison`). Since `face_box` moved onto the cover, nothing in
any crate's `src/` calls it. The doc citations of its convex-hull
assumption in `editor-core`'s `mate::reach` and `eval::measure` were
retired in `cover-app-readers`, and `topo/src/census.rs` keeps one
comment naming it. The spline carrier's cover is
`Surface::cover` → `Cover::net`, whose finite screen refuses a net
with poison anywhere, and whose `aabb` is the same box.

Its readers are tests: `geom/tests/surfaces/boxes.rs`,
`geom/tests/net_placeholder_width.rs` (the poison box never prunes)
and `geom/tests/net_placeholder_width_interval.rs`, plus the
`geom-core/tests/bounds_census.rs` roster line.

**The fix:** delete the door and its module; re-point the
containment and poison rows at `Surface::cover` (a refusal for a
poisoned net, which a pruning reader turns into `Aabb::poison()`
itself); drop the roster line. `cover-kind-match-gate`'s tree gate
would catch a new reader of it.
