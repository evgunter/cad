---
id: injective-spells-the-chart-winding-rule-by-hand
kind: issue
title: injective.rs spells the chart-winding rule by hand four times (the quarter-turns and face_side); its home is geom_brep::ChartWinding
status: open
opened: 2026-11-14
---


## Finding

Found by TESSLANE's winding-rule unit (`tesslane/winding-rule`), 2026-11-14,
sweeping for siblings of the five spellings it routed in `crates/mesh`.

The rule "a face lies on the left of its loop in its chart exactly when
`Face::sense` is set" has a home, `geom_brep::ChartWinding`
(`ChartWinding::of_face(sense).interior_is_left()`).
`crates/topo/src/injective.rs` spells it by hand four times, each a `±1` or a
negation under the bit, and topo's fold census
(`face_normal::tests::no_source_file_folds_the_bit_by_hand`) sanctions all four:

- `face_side`: `Some(if sense { side } else { -side })`;
- `present_plane`, `present_plane_corner` and `present_plane_vertex`:
  `if b.sense { 1.0 } else { -1.0 }`, "its face side is the traversal turned a
  quarter, by sense".

## What closing it takes

Read the side through `ChartWinding::interior_is_left` where the scalar
discipline of those sites allows it (they are generic over the scalar, and the
home answers a bool, not a `T`), and strike the census entries that empties. If
a site stays, say on its census line why the home does not serve it.
