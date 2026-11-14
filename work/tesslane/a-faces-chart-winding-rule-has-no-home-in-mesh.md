---
id: a-faces-chart-winding-rule-has-no-home-in-mesh
kind: issue
title: The rule 'a face's loop winds counter-clockwise in its chart exactly when sense is set' has five spellings in crates/mesh and no home
status: dispatched
opened: 2026-11-14
priority: P1
cost: M
branch: tesslane/winding-rule
---


## Finding

Found by the review of TESSLANE's sense-door unit (`tesslane/sense-door`), 2026-11-14.

`crates/mesh` relies on one rule: a face's loop winds counter-clockwise in its
chart exactly when `Face::sense` is set. That holds because the loop runs
interior-left about the outward normal and an analytic chart's `∂u × ∂v` is
the surface's outward normal. The rule has no home in the crate. Five sites
spell it, each by hand (checked by name at `d6e1eb1f07`):

1. `trimmed::tessellate_trimmed`'s orientation check:
   `(shoelace2(&poly2) > 0.0) != face.sense` refuses with `TrimLoopOrientation`.
2. `TrimmedFace::emit`'s `flip` (`shoelace2(&poly2) < 0.0`). It reads the
   polygon's winding, not the bit. On the analytic arms the check in 1 makes it
   equal to `!sense`, so the two are one rule read twice.
3. The test `trimmed::tests::the_tilted_pairs_faces_keep_their_poles_as_sides`:
   `let area = if face.sense { area } else { -area };` turns the equal-area
   shoelace into the face's interior area.
4. `walk::wrap_strip`'s period, `if rising == sense { -TAU } else { TAU }`:
   the period's sign is chosen so that the polygon winds counter-clockwise
   exactly when `sense` is set (its doc says so).
5. `walk`'s band-azimuth read in the no-rim arm:
   `let chart_area = if sense { area } else { -area };`. Here `area` is
   `loop_area`'s 3-D VECTOR area, which points to the outward side, and the
   bit turns it into the chart frame before an `atan2`. That is a vector
   folded by hand, not a scalar. topo's fold census
   (`face_normal::tests::no_source_file_folds_the_bit_by_hand`) holds it in
   its `sanctioned` list with that said on its line.

The census carries 3, 4 and 5 as `sanctioned` entries. Each sanction makes
one more by-hand spelling of the rule legitimate, and 1 and 2 are not
visible to it at all (a comparison and a winding read, not a negation).

## What closing it takes

Give the rule one home: a function the mesh crate reads, taking the bit and
answering the chart's winding or its frame turn. Route the five sites
through it, and strike the census's `walk.rs` `area` and `TAU` entries and
`trimmed.rs` `area` as they leave. Item 5 is a vector: if the home answers
an `OutwardNormal`-shaped reading for it, that is the door. If no single
home fits all five, say which spelling stays and why.

## Unit (2026-11-14, TESSLANE)

**Letter M. Tier: single review, FULL.** Five sites that each decide a sign; a home that gets one of them backwards ships a mirrored or inside-out patch that `check_mesh` may pass, so the review carries claims to falsify.

Branch `tesslane/winding-rule`.

- **Say what the rule is before giving it a home.** It is stated above for an analytic chart, whose `∂u × ∂v` is the outward normal. The trimmed lane's spline arm does not check it ("a spline chart's winding is not checked"), and site 2 reads the polygon instead of the bit. The unit says whether the rule holds on a spline chart and what the spline arm relies on instead; if it holds there too, the home serves all arms.
- **Where the home lives is part of the unit.** A loop's place in its face's chart is stored once, as its lift (`topo::pcurves`); whether "which way a face's loop winds in its chart" is a fact about the face that `topo` should answer, or a mesh-local reading, is decided with a reason. A home in `topo` is a seam with that file's owner, announced in the PR.
- Site 5 is a vector's fold. It leaves the census's `sanctioned` list by going through a door, not by being renamed.
- The census entries that the unit empties are struck; none is added.
- **Shared ground, live now:** unit 1 of the chart lane (`tesslane/chord-rule`) is in `chords.rs` and `sizing.rs`, and `tesslane/turned-ball` is in `curved.rs`'s CDT entries. Neither reads these five sites; merge main as they land.
