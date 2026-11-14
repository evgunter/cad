---
id: injective-spells-the-chart-winding-rule-by-hand
kind: issue
title: injective.rs spells the polygon-orientation rule by hand six times (the quarter-turns, face_side and two left_is_above comparisons); its home is geom_brep::PolygonOrientation
status: open
opened: 2026-11-14
priority: P1
cost: E
---


## Finding

Found by TESSLANE's winding-rule unit (`tesslane/winding-rule`), 2026-11-14,
sweeping for siblings of the five spellings it routed in `crates/mesh`.

The rule "a face lies on the left of its loop in its chart exactly when
`Face::sense` is set" has a home, `geom_brep::PolygonOrientation`
(`PolygonOrientation::of_face(sense).interior_is_left()`).
`crates/topo/src/injective.rs` spells it by hand six times. Four are a `±1` or a
negation under the bit, which topo's fold census
(`face_normal::tests::no_source_file_folds_the_bit_by_hand`) sanctions:

- `face_side`: `Some(if sense { side } else { -side })`;
- `present_plane`, `present_plane_corner` and `present_plane_vertex`:
  `if b.sense { 1.0 } else { -1.0 }`, "its face side is the traversal turned a
  quarter, by sense".

Two are comparisons with the bit, which that census cannot see (found by the
unit's review):

- `left_is_above(rising == u.plus, Axis::V) == piece.boundary.sense`;
- `left_is_above(rising == plus, column) == piece.boundary.sense`.

## What closing it takes

Read the side through `PolygonOrientation::interior_is_left` where the scalar
discipline of those sites allows it (they are generic over the scalar, and the
home answers a bool, not a `T`), and strike the census entries that empties. If
a site stays, say on its census line why the home does not serve it.
