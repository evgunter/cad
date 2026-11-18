---
id: the-chart-quarter-turn-has-no-home
kind: issue
title: the 2-D quarter turn (x, y) to (-y, x) has no home in geom-core; arc.rs, arc/pair.rs, mesh's cert.rs and injective.rs each spell it
status: open
opened: 2026-11-17
priority: P4
cost: E
---



## Finding

Found by the review of EMBED's winding-rule unit (`embed/winding-rule`,
PR 7071), 2026-11-17.

`geom_core::Vec2` has `perp_dot` (`linalg/vec.rs`) and no left
perpendicular, so the quarter turn `(x, y) → (−y, x)` is spelled at each
use:

- `geom-core/src/arc.rs`: `Vec2::new(-unit.y, unit.x)` in `from_chord_radials`
  and in `apex`, and `Vec2::new(-dir.y, dir.x)` in `register_tangent_at`;
- `geom-core/src/arc/pair.rs`: `Vec2::new(-u.y, u.x)` in `crossings`;
- `mesh/src/cert.rs`: the right turn `[e[1], -e[0]]` twice, in the test
  `the_two_distances_are_the_distance_on_triangles_built_to_be_degenerate`;
- `topo/src/injective.rs`: `left_turn`, on a bare `[T; 2]`. Its readers are
  `toward_face`, `face_side` and `left_is_above`.

## What closing it takes

`Vec2::perp` (the left turn), or a free function on `[T; 2]` if the
interval sites keep their bare arrays, with the sites above reading it.
