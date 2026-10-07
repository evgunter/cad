---
id: a-non-isosceles-turn-overruns-past-the-mitre
kind: issue
title: blend: two requested edges at a non-isosceles trivalent corner refuse TURN_NOT_ISOSCELES, where Ev's ruling builds the mitre plus one short overrun curve
status: open
opened: 2026-10-07
priority: P2
cost: H
design: true
---


Split from `a-plane-plane-blend-cannot-end-at-an-unrequested-corner` at its
step 4 (PR 4209), as Ev's PR 4085 ruling directs ("step 5: the non-isosceles
overrun (a numeric probe before its spec) … split to their own rows when
step 4 lands"). The whole-face planar path that step 5 was also to delete
was already folded into the local carve at step 2 (that row's findings),
so nothing is left to delete.

## What refuses today

Two requested edges at a trivalent plane–plane vertex whose third edge L is
unrequested build the mitre only where `fillet3_turn_isosceles` reads the
trihedron isosceles (`crates/sweep/src/blend/battery.rs` `turn_at`). The
margin is the larger of the levered face-angle cosine difference and the
gap between the two bands' feet on L. A definite verdict refuses
`UnsupportedRunOut { TURN_NOT_ISOSCELES }`.

Users meet this at:
- the bracket's section-face rims (tour wall 3, `demos/tour/src/bracket.rs`):
  90° at the cap chord against 45° or 135° at the section edge;
- the sheared box's supplementary corner (φ, π−φ), where a chamfer's two
  feet coincide on L but the face angles differ (pinned in
  `band_planar_mitre.rs`);
- the leaning trapezoid prism (`common::operands::leaning_turn`).

## The ruling

"two [edges], the MITRE along the bands' intersection (line; planar
ellipse), with one more short curve where the trihedron is not isosceles."
The two bands' trimlines meet L at different points. The overrun is the
short curve that closes the gap between the feet, on one of L's faces.

## What the taker owes

1. **A numeric probe before any spec** (the ruling's order). On the three
   witnesses above, and on a sweep of face-angle pairs for both verbs and
   both convexities, measure:
   - where each band's trimline meets L;
   - which band overruns the other and onto which face;
   - the shape of the short curve (for the chamfer, a line on L's face?
     for the fillet, a section of which surface?);
   - whether the supplementary chamfer, whose feet coincide, needs any
     overrun at all.
2. Then a spec, through the designer protocol if the probe shows a fork:
   the overrun's carrier, its topology (a valence and a new role name),
   its clearance (which meter covers the overrun region), and the
   verdict's three-way split (isosceles mitre / overrun / in band).
3. The build, retiring the bracket's wall 3.
