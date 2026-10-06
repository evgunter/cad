---
id: a-boss-flush-with-a-block-edge-refuses-its-declared-union
kind: issue
title: A boss drawn on a block's top flush with its side wall refuses the declared union: seam chord between two isolated pierce points
status: closed
opened: 2026-09-30
closed: 2026-10-06
branch: zip/chord-rows
priority: P1
cost: H
refs: [m9-3-semantic-residues, a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence]
---


Found by AUTH-9 (`author/declared-union`) while looking for a pair of
bodies that touch at two faces.

## The finding

Draw a rectangular boss on a block's top face so that one of its side
walls is flush with a side wall of the block, and union the two. The
union refuses `UndeclaredContact` on the resting pair (block
`Cap(End)`, boss `Cap(Start)`, `Rest`). Declaring exactly that pair,
the way the boolean tool's offer does, moves the refusal into the
declared-REST zip:

```
the Boolean op refused: declared-REST union zip: seam chord between two
isolated pierce points — a named sub-frontier of the boundary-on-boundary
REST lane (planar declared contacts whose seam splits cleanly are covered);
declare the coincidence, move the geometry, or lower the tolerance
```

raised at `crates/topo/src/boolean/rest.rs`, the `([], [])` arm of the
seam-chord match (`unsupported("seam chord between two isolated pierce
points")`).

The scene, through the viewer's op vocabulary: a 40 × 20 × 10 mm block
(a rectangle centred on the world xy frame, extruded 10 mm); a frame
read off its top cap (`DatumSpec::FaceFrame`, spin 0, origin at the
cap's centre); on it a closed path through (10, −5), (20, −5), (20, 5),
(10, 5) mm, extruded 4 mm. The boss's +x wall lies in the block's
+x wall plane, co-oriented. All planar.

## Why it matters

A boss or rib drawn to the edge of a face is an ordinary gesture, and
since AUTH-9 the declared union is one the GUI authors in one click.
This scene lands the `Declare` and the union and then the union's row
fails with the sentence above.

**The same refusal text, found before.** `work/tang/m9-3-semantic-residues.md`
§2 (R1 NOTE-4) meets "seam chord between two isolated pierce points" as
the refusal that shadows `glue_pair`'s ring-count gate on the natural
mismatch fixture. That row's configuration and this one's may share a
cause, and whoever takes either should look at both.

**Its recourse is a separate row.** The sentence tells the author to
"declare the coincidence" to a boolean whose coincidence is already
declared. That is true of every refusal in this lane:
`a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence`.

**Folded here: the flush side walls were never reported as a second
contact.** The boss's +x wall and the block's are coplanar and
co-oriented, yet declaring the resting pair moves the refusal into the
zip rather than to a second `UndeclaredContact` naming the walls. So
it is not known whether declaring them as well would reach a different
rung. This was not tried: their names come from no refusal, and
constructing them by hand would be exactly the guessed declaration the
boolean tool's offer exists to avoid. Whether coplanar co-oriented
walls in a union are a contact the census should report is a detection
question, and it belongs with this scene.

## A second declared-flush union past its declaration (AUTH-9's review, 2026-09-30)

Block ∪ a cylinder through it whose two caps sit flush with the
block's top and bottom. The boolean tool offers both cap pairs, one
refusal at a time. After both are accepted, the union commits and its
row fails with "the solids do not cross … every test ray grazed …
Recourse: declare the coincidence". So the declared pairs reach a
classification rung that cannot place the operands, rather than the
zip. The same operands evaluate correctly as subtract
(V = 7.2146e-6 m³) and intersect (7.854e-7 m³). This is a different
rung from the seam chord above, reached by the same gesture, and it
carries the same recourse.

## Measured on main (2026-10-06)

Measured on `origin/main` 3f1e3b0d by the ZIP lane `zip-chord`, at
the 1e-9, 1e-6 and 1e-12 rows, both operand orders. The scene was
built four ways: the sweep extrusion and the brick door, each at 1 and
at 1e-3 (millimetres in metres), and through the op vocabulary
(`Datum::FaceFrame`, spin 0, off the block's `Cap(End)`).

- Undeclared, and with only the resting cap pair declared, the union
  refuses `UndeclaredCoincidence { relation: SameOriented }` naming
  the two `+x` walls (`bool_plane_offset`, margin 0). The flush walls
  are the first contact reported, so the "Folded" paragraph above no
  longer describes main.
- Declaring what the refusals name, one at a time (the walls as a
  continuation, then the caps as `Rest`), builds the union in the
  chord join at the closed form `8000 + 400` (one shell, tier 3, the
  census). The declared-REST zip is not entered.
- AUTH-9's through-cylinder scene (a cylinder of radius 5 through the
  40 × 20 × 10 block, its caps flush with the block's top and bottom)
  was not measured when it was written up. Measured now: each cap pair
  refuses in turn as `SameOriented`, and with both declared as
  continuations the union builds at the block's volume, 8000, with the
  rim authored as two arcs and as three.

No declaration, door, scale, order or eps row reproduces the seam-chord
refusal. The viewer binary itself, and the AUTH-9 branch the scene was
found on, were not run.

## Closed (2026-10-06)

Pinned by `crates/sweep/tests/boss_flush_with_a_block_edge.rs`:
- `the_flush_walls_are_reported_before_and_after_the_cap_pair_is_declared`;
- `declaring_what_the_refusals_name_builds_the_union_at_its_closed_form`;
- `a_cylinder_through_the_block_with_flush_caps_unions_to_the_block`.

The op-vocabulary build is pinned by
`crates/editor-core/tests/boss_flush_offer.rs`
(`accepting_each_offer_in_turn_builds_the_flush_boss_union`).

The recourse sentence stays its own row
(`a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence`).
