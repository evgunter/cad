---
id: an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root
kind: issue
title: validation's pair pass recomputes an adjacent pair's shared vertex as a root of the carriers' intersection, then asks which root it got: six numeric decisions on a parameter bulge stand on that choice
status: open
opened: 2026-09-26
priority: P2
cost: D
refs: [the-apothems-sign-is-a-value-read, arc-arc-shallow-corner-legs-escalate-arc-span]
---


## What

`Profile::validate`'s pair pass (`crates/profile/src/validate.rs`,
`judge_pair`) runs `seg::pair_contacts` on every segment pair, adjacent
pairs included.
- It builds `shared`, the pair's legal shared vertices, and does not
  pass it to `pair_contacts`.
- It only discounts afterwards a Touch that `contact_at_shared_vertex`
  finds within the band of a shared vertex.

So for an adjacent pair, `pair_contacts` solves the carriers'
intersection from scratch:
- For a line and an arc, `seg::line_arc` spells the two candidates
  `tc ∓ sqrt(r² − h²)`, with `tc` the foot of the centre's perpendicular
  on the line. It asks `line_span` and `arc_span` at each.
- `arc_arc`'s secant arm is the same shape, across the radical line.

One of the two candidates IS the shared vertex, and which one it is
depends on a sign. On R2's D-tab, where the lines adjacent to the arc are
perpendicular to its chord, that sign is the apothem's,
`sign(1 − b²)·σ`. To prove "this candidate is the shared vertex", the
symbolic tier must show `|apothem| = ±apothem`. So on a parameter bulge
these decisions land `numeric`:
- on the `0.5` parameter control, `line_span`, `arc_span` and
  `contact_at_shared_vertex`, two each;
- on the `0.4` parameter D-tab, `line_span`, two, which are frozen on
  `fl(0.4)`'s coefficient ring.

The early forms are in `crates/editor-core/tests/m10_bulge_renders.txt`
(on `props/sign-hull`). The attribution is
`work/decide/the-apothems-sign-is-a-value-read.md`.

## The fix

`pair_contacts` takes the pair's shared vertices.
- **The shared vertex is a contact by construction:** an endpoint of both
  segments, so a Touch at a shared vertex. Nothing is asked about it.
- **The other intersection is spelled from it, with no square root:**
  - line/circle: `2·foot − v`, from the sum of the quadratic's roots;
  - two circles: `v` reflected across the line through the centres.
- **Unchanged:**
  - the carrier predicates still decide tangent against secant, and a
    tangent adjacent joint has no second candidate;
  - `line_span` and `arc_span` are asked at the second candidate only;
  - non-adjacent pairs.

## What to measure first

- **What stops being asked:**
  - the six on the `0.5` parameter control;
  - the same joints' decisions on both `0.4` D-tabs, including the
    literal D-tab's six frozen on `fl(0.4)`'s ring;
  - the boss's two `sign_gated` `line_span`;
  - the bracket's fillet-joint `line_span` and
    `contact_at_shared_vertex`.
- **Whether `contact_at_shared_vertex` keeps a caller.** A Touch at a
  definite-secant pair's second candidate may always be far from the
  shared vertex. `SHARED_CLAUSE_ONLY`'s roster row holds it either way.
- **The second candidate's value bits,** which move. Re-baseline what
  moves.
- **Whether it moves the adjacent `line × arc` instances of
  `arc-arc-shallow-corner-legs-escalate-arc-span`** (P0): the same pair
  pass at a near-tangent joint. The arc × arc witness there is
  NON-adjacent (segments 0 and 2), which this fix does not touch.
