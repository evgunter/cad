---
id: singular-turn-spells-the-chart-winding-rule-by-hand
kind: issue
title: pcurves::singular_turn's step spells the polygon-orientation rule by hand; its home is geom_brep::PolygonOrientation
status: open
opened: 2026-11-14
priority: P1
cost: E
---


## Finding

Found by TESSLANE's winding-rule unit (`tesslane/winding-rule`), 2026-11-14,
sweeping for siblings of the five spellings it routed in `crates/mesh`.

`pcurves::singular_turn` decides the whole turn a face takes round a singular
point of its chart as `let step = if above == sides.sense { 1 } else { -1 };`.
That is the polygon-orientation rule: the face is on the left of its loop in the chart
exactly when its sense is set, so along a latitude the face lies above the
azimuth advances. The rule has a home, `geom_brep::PolygonOrientation`
(`PolygonOrientation::of_face(sense).interior_is_left()`), and topo's fold census
(`face_normal::tests::no_source_file_folds_the_bit_by_hand`) sanctions this
spelling as `("crates/topo/src/pcurves.rs", "1")`.

`mesh::walk::wrap_strip` read the same quantity off the bit (its period is this
step at the strip's far tip, times a turn) and reads it through the home since
that unit. The lift stores the step as the joint's element, so the mesh could
read the strip's period off the stored lift rather than derive it a second time;
that is the larger repair and is not done.

## What closing it takes

Read `step` through `PolygonOrientation::interior_is_left` and strike the census
entry. Separately, decide whether `walk::wrap_strip` reads its period off the
loop's stored lift.
