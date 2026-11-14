---
id: a-faces-chart-winding-rule-has-no-home-in-mesh
kind: issue
title: The rule 'a face's loop winds counter-clockwise in its chart exactly when sense is set' has five spellings in crates/mesh and no home
status: closed
opened: 2026-11-14
priority: P1
cost: M
branch: tesslane/winding-rule
closed: 2026-11-14
pr: 6944
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

## Closed (2026-11-14, PR 6944)

The home is `geom_brep::PolygonOrientation`: the orientation of a polygon in a chart's `(u, v)` plane, `of_face(sense)` being the rule. It holds on every chart, a spline's included, because none of the three facts it is made of is a kind of surface. It is in geom-brep by `[topo.homes]` rule 1 (a reading that takes no `Body` and no arena key). The name is not "winding": that word is the turning number's in `topo::pcurves`.

Seven sites read it: the trimmed lane's check and emit pass and its test, `walk::wrap_strip`, `curved`'s flip (a sixth spelling this row had not listed) and `geom_brep::chart_singularity::holds` (a seventh, found by the review's `== sense` sweep). Site 5, the band azimuth, turned out to be the normal fold and goes through `OutwardNormal::from_chart`. The three census entries are struck.

Left for others, each in its own row: the spline arm does not compare its polygon with the bit (`the-trimmed-spline-arm-does-not-check-its-polygons-winding`); `shoelace2` sums about the chart origin (`shoelace2-sums-about-the-chart-origin-so-a-thin-face-far-up-a-chart-can-read-the-wrong-orientation`); the rule is still spelled by hand in `topo::injective` (EMBED), `topo::pcurves`' `singular_turn` and a sweep test (CHARTTAIL), and step-import's `normalize` (IMPORTGATE); the fold census does not see a comparison against the bit (ONEDOOR).
