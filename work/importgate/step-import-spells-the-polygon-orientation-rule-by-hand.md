---
id: step-import-spells-the-polygon-orientation-rule-by-hand
kind: issue
title: step-import's normalize spells the polygon-orientation rule by hand on cylinder and cone walls; its home is geom_brep::PolygonOrientation
status: open
opened: 2026-11-14
priority: P4
cost: E
---


## Finding

Found by the review of TESSLANE's winding-rule unit (`tesslane/winding-rule`),
2026-11-14, sweeping for comparisons with a face's sense bit.

The rule "the polygon a face's boundary draws in its chart is counter-clockwise
exactly when `Face::sense` is set" has a home,
`geom_brep::PolygonOrientation::of_face`. `crates/step-import/src/normalize.rs`
spells it by hand twice, on cylinder and cone walls:

- the wall-rectangle gate: `let ccw = if va < vb { *ru_a } else { *ru_b };`
  then `if face.sense != ccw` refuses "an ORIENTATION-INVERTED cylinder/cone
  wall face";
- the region's start rim: `let ls = usize::from(rim[0].0 != sense);`, "the loop
  the face's boundary traverses in +u when `same_sense`".

Neither is a negation, so topo's fold census cannot see either.

## What closing it takes

Compare the rectangle's orientation with `PolygonOrientation::of_face(face.sense)`
at the gate, and read the start rim's direction through
`interior_is_left`, or say at each site why the home does not serve it.
