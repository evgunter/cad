---
id: a-boss-flush-with-a-block-edge-refuses-its-declared-union
kind: issue
title: A boss drawn on a block's top flush with its side wall refuses the declared union: seam chord between two isolated pierce points
status: open
opened: 2026-09-30
priority: P1
cost: H
refs: [addboolean-doc-names-a-vocabulary-that-does-not-exist]
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

Two things about that sentence are worth a look beside the frontier
itself: its recourse says "declare the coincidence" to a boolean whose
coincidence is declared, and the flush side walls were never reported
as a second undeclared contact, so it is not known whether declaring
them as well would reach a different rung. Not tried here: their names
come from no refusal, and constructing them by hand would be exactly
the guessed declaration the offer exists to avoid.
