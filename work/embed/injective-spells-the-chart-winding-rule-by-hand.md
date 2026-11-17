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

**Evidence (GREEN, PR 6921, 2026-11-14; folded in from a duplicate filed before this row reached main).** The census side of the duplication: `face_normal`'s `sanctioned` list holds one identical `("crates/topo/src/injective.rs", "1.0")` tuple per quarter-turn copy, three now. A fourth plane presentation adds a fourth spelling and a fourth tuple, and the census cannot tell a fourth sanctioned quarter-turn from a new hand fold. Routing the copies through `PolygonOrientation` takes the tuples out with them.

## Landed (EMBED, lane embed/winding-rule, 2026-11-17)

`injective.rs` reads the rule from its home at all six sites. `Boundary::orientation`
is `PolygonOrientation::of_face(self.sense)`. `face_side` and the new
`toward_face(tau, orientation)` take a `PolygonOrientation` rather than the bit;
`toward_face` is the one quarter-turn, toward the chart side `interior_is_left()`
names, and `present_plane` (both arms), `present_plane_corner` and
`present_plane_vertex` call it. The two `left_is_above` sites compare with
`orientation().interior_is_left()`. `pcurves::singular_turn` reads the same home
(CHARTTAIL's `singular-turn-spells-the-chart-winding-rule-by-hand`, first half).
The census's four `injective.rs` entries and the `pcurves/decision.rs` entry are
struck, and its blind-spot list names a fold under a `PolygonOrientation`.

Mutants (the home's answer flipped at one site, `topo` under the `ci` profile):
every one of the nine reddens `injective::presentation_rows` or, for
`singular_turn`, `pcurves::pole_slit_tests` and `lift_rows`; the two-line corner
arm of `present_plane` reddens only
`clearance::wiring_rows::the_verdict_door_answers_at_every_certifying_scalar`.
