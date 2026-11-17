---
id: polygon-orientation-callers-fold-the-interior-side-by-hand
kind: issue
title: mesh's wrap_strip and geom-brep's chart_singularity fold the interior side by hand beside PolygonOrientation's to_interior and interior_is_on
status: open
opened: 2026-11-17
priority: P3
cost: E
---



## Finding

Found by the review of EMBED's winding-rule unit (`embed/winding-rule`,
PR 7071), 2026-11-17.

`geom_brep::PolygonOrientation` answers the folded forms of its rule:
`to_interior(left)` takes a signed value read on a traversal's left to the
region's side, and `interior_is_on(left_is_on)` says whether a side holds the
region given whether it holds the left. topo reads them (`injective.rs`'s
`face_side`, `toward_face`, `present_wall`, `graph_read`; `pcurves::singular_turn`).
Two callers outside topo still fold `interior_is_left()` by hand:

- `mesh/src/walk.rs` `wrap_strip`:
  `period: if rising == orientation.interior_is_left() { -turn } else { turn }`.
  topo's fold census (`face_normal::tests::no_source_file_folds_the_bit_by_hand`)
  now keys on `interior_is_left` and sanctions this line as
  `("crates/mesh/src/walk.rs", "turn")`, naming this row.
- `geom-brep/src/chart_singularity.rs`:
  `let face_is_left = PolygonOrientation::of_face(sense).interior_is_left();`
  then `Ok(left) if left != face_is_left`. That is `interior_is_on` spelled
  out, and the census cannot see it, because nothing is negated under an `if`.

## What closing it takes

`wrap_strip` reads `orientation.to_interior(…)` and the census entry leaves.
`chart_singularity` reads `interior_is_on` (or `!`). Whether `wrap_strip`'s
period and `singular_turn`'s step are one derivation is CHARTTAIL's
`singular-turn-spells-the-chart-winding-rule-by-hand`.
