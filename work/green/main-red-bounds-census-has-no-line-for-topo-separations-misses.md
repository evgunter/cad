---
id: main-red-bounds-census-has-no-line-for-topo-separations-misses
kind: issue
title: main red: geom-core's bounds_census has no roster line for topo::separation's misses (a sole T: Bounds door)
status: open
opened: 2026-11-24
priority: P1
---


Found by `from-f64-is-split-by-where-the-number-comes-from` (PR 7721), running the slow-set censuses that read what it changes. `cargo test -p geom-core --test all -- bounds_census::` on its head (main 9dc5eeaf merged) fails `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`:

```text
the sole-bracket-bound roster no longer matches the tree.
  arrived or moved (owe a roster line WITH its disposition): [
    ("crates/sweep/src/blend/reach.rs", "crossing"),
    ("crates/topo/src/separation.rs", "misses"),
]
```

`reach.rs` `crossing` is the red the scalar lane already knows. `separation.rs` `fn misses<T: Bounds>(hull: &Aabb, p: Point3<T>) -> bool` is new: it arrived on main with commit b18870a5 ("separation: the reversed-order placement row reads containment back; ...") and the roster in `crates/geom-core/tests/bounds_census.rs` has no line for it. PR 7721 does not touch `separation.rs`. The census is in the slow set, so only the nightly and a PR that edits its file run it.
