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

`injective.rs` reads the rule from its home at all six sites, and the home
answers the folded forms itself:

- **The home's new methods:** `PolygonOrientation::to_interior(left)` takes a
  signed value read on a traversal's left to the region's side.
  `interior_is_on(left_is_on)` says whether a side holds the region.
- **The bit's one conversion:** `Boundary::orientation()` is
  `of_face(self.sense)`.
- **`face_side` and `toward_face`** take the orientation, not the bit.
- **The turn:** the left-of-traversal turn is one function, `left_turn`. It is read
  by `toward_face` (on interval arrays: a line edge's in-face normal, at
  `present_plane`'s two arms, `present_plane_corner` and `present_plane_vertex`),
  by `face_side` (on an iso edge's axis) and by `left_is_above` (as a boolean).
- **`present_wall` and `graph_read`** read `interior_is_on(left_is_above(…))`.
- **`pcurves::singular_turn`** reads `to_interior` (CHARTTAIL's row, first
  half).

The census struck its four `injective.rs` entries and the `pcurves/decision.rs`
entry, and it now keys on `interior_is_left` too. The one fold that change
newly sees, `mesh::walk::wrap_strip`, is sanctioned against ONECOPY's
`polygon-orientation-callers-fold-the-interior-side-by-hand`. The 2-D quarter
turn's own home is ONECOPY's `the-chart-quarter-turn-has-no-home`.

A new row, `a_planes_reflex_corner_of_two_lines_is_not_its_wedge`, holds
`present_plane`'s two-line arm in the direction that would certify wrongly.
Before it, deleting that arm's lean gate turned no topo test red.

Mutants (the home's answer flipped at one site, `topo` under the `ci` profile)
are listed in the PR. Every site turns rows red.
