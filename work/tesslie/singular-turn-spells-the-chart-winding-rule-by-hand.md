---
id: singular-turn-spells-the-chart-winding-rule-by-hand
kind: issue
title: pcurves::singular_turn's step spells the polygon-orientation rule by hand; its home is geom_brep::PolygonOrientation
status: dispatched
opened: 2026-11-14
priority: P1
cost: E
branch: tesslie/wrap-strip-period
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

## The step reads the home (EMBED, lane embed/winding-rule, 2026-11-17)

`singular_turn` now writes the step a face on the loop's left would take,
`if above { 1 } else { -1 }`, and takes it to the face's side through
`PolygonOrientation::of_face(sides.sense).to_interior(…)`. The census entry
`("crates/topo/src/pcurves/decision.rs", "1")` is struck. This landed in the
PR that routed `injective.rs`'s six spellings (PR 7071).

That does not discharge the second half. `singular_turn`'s `step = ±1` and
`walk::wrap_strip`'s `period = ∓turn` both say which way the azimuth runs round
a pole, by the face's side. They use opposite sign conventions on different
inputs: `above` from the singularity's kind here, `rising` from the poles'
heights there. Reading `PolygonOrientation` makes both look routed, but they
are still two derivations, and nothing proves they are one rule. Whether
`wrap_strip` reads its period off the loop's stored lift is still open.
`wrap_strip`'s hand fold under `interior_is_left` is ONECOPY's
`polygon-orientation-callers-fold-the-interior-side-by-hand`.

## Moved to TESSWALK (2026-11-20, CHARTTAIL triage)

The topo half is done (EMBED, above): `pcurves/decision.rs` `singular_turn` takes its step through `PolygonOrientation::of_face(sides.sense).to_interior(left)`, and no `pcurves` entry remains in the fold census. What is left is all in `crates/mesh/src/walk.rs`: whether `wrap_strip`'s `period` (from the poles' heights, `rising`) and `singular_turn`'s stored step are one rule, and whether `wrap_strip` reads its period off `Body::loop_lift` instead of deriving it again (census entry `("crates/mesh/src/walk.rs", "turn")`). That is TESSWALK's ground, beside its `mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers` (the same read for `unwrap_tie`) and ONECOPY's `polygon-orientation-callers-fold-the-interior-side-by-hand`. The band stays P1: two derivations of one rule. Moved by `git mv`, id unchanged.

Moved from TESSWALK to TESSLIE on 2026-11-20, by `git mv`, when TESSWALK merged into TESSLIE: TESSWALK held no P0 or P1 row before this one arrived, so no sitting would have dispatched it on its own (`work/README.md`, Track size), and its charter is one TESSLIE carries.
