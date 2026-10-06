---
id: a-pinch-crossed-before-the-zips-fuse-it-crosses-the-ring-at-a-twice-visited-vertex
kind: issue
title: A pinch crossed before the zips fuse it splits a ring at a twice-visited vertex into holes whose corners overlap
status: open
opened: 2026-10-06
priority: P0
cost: H
---


## What

Found by TANG's PR 4129 review. The PR now refuses these cases typed
where they used to build crossed; this row is the real fix.

**The witness.** The plate `[0,3] × [0,2] × [0,1]` less U, in one
boolean. U is the union of right prisms whose footprints on the plate's
top are k holes meeting at (1.5, 1, 1), each prism leaning out of its
footprint (`topo::test_support::meeting`). On main, k = 2 builds
[18, 41, 25]. Tiers 3 and 3′ and the volume pass, but the top face holds
two triangular rings through one vertex, whose corners there overlap
(`topo::test_support::meeting::corners_disjoint` fails). The next
boolean on the body can refuse `ClassificationInvariant { "two crossing
germs of a vertex pair lie along one direction in one sector entry" }`.
With PR 4129's k ≥ 3 pierces, k = 3 gives [27, 66, 40] and k = 4 gives
[32, 80, 49], crossed the same way.

These are clean:

- U − P, P ∩ U and U ∩ P;
- the plate less each prism in turn: two vertices at the point and one
  ring through both;
- the union, where one vertex has one ring through it k times.

**The trace.**

1. `vtxfac::classify_vertex_on_face` reads the same runs at U's vertex
   in union and subtract: n = 8, runs [(2, 1), (0, 1)] at k = 2. Flipping
   the subtract strut facing desyncs the join, so the facing is
   consistent.
2. In P − U both operands keep the meeting point as one vertex, so the
   zips would fuse it twice. `zip::cross_pinches` crosses it first, and
   `split_across` (instrumented: one hit, at the point) takes the
   `Crossing::OneLoop` arm. That is `mev`, then `kemr`, across two
   corners of the top face's single ring.
3. **The kemr swaps the corner pairing.** The ring passes the vertex
   twice, with corners (in₁ → out₁) and (in₂ → out₂), disjoint, so the
   four directions run out₁, in₁, out₂, in₂ round the vertex. The `kemr`
   leaves rings with corners (in₁ → out₂) and (in₂ → out₁). Each of
   those sweeps both originals, so the corners overlap whatever the
   geometry. The pair's own zip then fuses the new vertex back, giving
   one vertex and two rings: the crossed face.
4. **`finish::pinch_site`'s `Joint::Hole` crosses the same way.** Its
   weld of two vertices, which one ring passes once each, leaves two
   rings through one vertex with the swapped pairing.

Both came in with 65a09d4a ("a pinch the zips would fuse twice is
crossed first"; PRs 4026 and 4074), documented as "two holes meeting at
the point, one shape at rest".

**What PR 4129 did.**

- `split_across` no longer takes a one-ring crossing. Where it is the
  only one on offer, the split refuses
  `BooleanError::PinchCrossesRingCorners`.
- `pinch_site` refuses the same joint across one ring.
- `Joint::Hole` and the `OneLoop` crossing are gone.

## The candidate fix

Keep the ring whole and the point as two vertices on it, as the
sequential subtract builds: the split's `mev` without the `kemr`, with
the pair's zip leaving the two copies apart. Or find a crossing that
keeps each corner's pairing. Then retire `PinchCrossesRingCorners`.

## Rows that flip to builds when it lands

`crates/topo/tests/holes_meeting_at_a_vertex.rs`,
`the_plate_against_the_holes_union_builds_sound_or_refuses_typed_in_every_op`:
P − U for two, three and four wedges, three wedges on one side, and an
L with two wedges. Each then owes the same checks the other ops pass:
`corners_disjoint`, tier 3, the volume, and a block across the point
that unions with it. The PR's list of existing rows that moved is in
its body.
