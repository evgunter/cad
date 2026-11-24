---
id: two-plane-faces-on-one-line-through-a-vertex-they-share-alone-refuse-at-interval
kind: issue
title: Two plane faces whose edges lie on one line on either side of a vertex they share alone hold at f64 and refuse Uncertified Departure at Interval, as built and translated, and hold turned
status: open
opened: 2026-11-22
priority: P1
cost: C
---


## Finding

Found by EMBED's lane `embed/clip-vertex` (2026-11-22), building the
body rows of `crates/sweep/tests/self_intersection_proves_injectivity.rs`
for a vertex in the middle of a wall's ruling.

The body is `half_rod_on_a_block` of that file: the half disc
`x² + y² ≤ 1`, `y ≥ 0` over `z ∈ [0, 2]`, united with the block
`[0, 1] × [-1, 0] × [0.5, 1.5]`, one valid solid. Its plane faces alone,
asked of `topo::clearance`'s sweep at the strict bound:

| pose | `f64` | `Interval` |
|---|---|---|
| as built | `Holds` | `Refused(Uncertified { a: 6v3, b: 9v1, shared: Vertex(12v1), arm: Departure })` |
| moved by `(100, -100, 100)` | `Holds` | the same refusal |
| turned `0.7` about `(2, 3, 6)/7` through `(0.3, -0.2, 0.1)` | `Holds` | `Holds` |

The refusing pair is the half rod's flat face `y = 0` (less the block's
footprint) and the block's side face `x = 1`. Vertex `12v1` is
`(1, 0, 0.5)`. Both faces hold the line `x = 1`, `y = 0`: the flat face
has it as an edge below the vertex, the side face above it, and the two
share the vertex alone. Each face's corner there is convex.

With the block reaching to `x = 2` (the same builder, its block
`[0, 2] × [-1, 0] × [0.5, 1.5]`) the same table reads
`Uncertified { a: 3v3, b: 6v3, shared: Edge(15v1), arm: Departure }`
at `Interval` as built and moved, between two plane faces again, and
`Holds` turned and at `f64`.

**Measured on main's `injective.rs`** (`b99f71c3d`, the file swapped into
the lane's checkout, the plane faces asked alone): the same three
verdicts per scalar. So it is not the lane's reading.

**Not traced:** which exit refuses the `Interval` pair (the presentation
of one face, or the vertex-alone plane's sign), and why a rigid turn
reads it. A turn makes every chart coordinate a bracket with width;
as built they are points at `f64` and brackets a rounding wide at
`Interval`, so a reading that asks for an exact value on one scalar and
a decided one on the other is the first thing to test.

## Valid or in band

Valid: an ordinary union, its faces a unit across. The file's rule is
that the `Interval` body reads the `f64` body's verdict class.

## Evidence

- Pinned as it reads in
  `self_intersection_proves_injectivity::a_wall_holds_against_a_face_at_a_vertex_mid_ruling`
  (the whole body at `Interval`: refusing between two plane faces as
  built and moved, holding turned).
- Cheapest reproducer: that test's body, `injective(&body, Some(planes), BUDGET)`
  at `Interval`.

## On the landing branch (2026-11-23)

On `origin/embed/wave3-land` at `8d34076a9e` the body of this row holds
at `Interval` as built and moved as well as turned: the whole-body
`Interval` assertion of
`self_intersection_proves_injectivity::a_wall_holds_against_a_face_at_a_vertex_mid_ruling`
is `Holds` in every pose there. **Not traced** which of the units in
that branch reads the plane pair; this row closes when that branch
lands, if the row's own two verdicts are re-read then.
