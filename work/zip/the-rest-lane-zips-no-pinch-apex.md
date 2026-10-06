---
id: the-rest-lane-zips-no-pinch-apex
kind: issue
title: The REST lane reads its vertex correspondence one-to-one, so a pinch apex meeting one vertex refuses
status: parked
opened: 2026-10-05
priority: P3
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---

## What

`boolean/rest.rs` `try_rest_union` keys its vertex correspondence
`vcorr` by the A vertex and holds one B vertex per key; `a_of` and
`b_of` invert it, and `glue_pair`'s `SeamCorrespondence` holds one
correspondent each. A pinch apex — two vertices of one operand at one
point, as a union carried with its v-v rows leaves where two pieces
touch along an edge — met by ONE vertex of the other operand has no
one-to-one reading.

The lane refuses such a correspondence typed
(`RestZipFrontier::PinchApex`) as it builds it, in either order, before
any chord is minted. `crates/sweep/tests/rest_nested_strut.rs`
`a_pinch_apex_meeting_one_vertex_refuses_as_the_frontier_in_either_order`
pins it (a notched block holding a wedge, touching along the apex line,
and a prism whose corner rests on the apex; the join refuses
`Euler(NotSameFace)` in both orders, its halves on two faces at the
fillets' tangency
(`work/join/a-boolean-match-takes-a-half-from-a-sector-on-a-face-its-ends-do-not-share.md`),
and hands the union to the lane): the two apex vertices each correspond
to the prism's corner.

Without that refusal the two orders answered different classes for the
one geometry:

- **Pinch first.** The two apex vertices each mapped to the prism's
  corner and the seam realized on both operands. The glue then refused
  `SeamOrientation` (the "kernel bug" variant): after the first patch
  pair's glue fused the corner into one apex vertex, the second pair's
  glue met that corner again (see also
  `work/zip/the-rest-lanes-glue-reads-its-correspondence-unfused.md`).
  Read through the glue's fusions instead, the second glue fused the two
  apex vertices into one and refused `Euler(SelfLoopEdge)`: the result
  wants the apex kept as the pinch it is, which neither zip models.
- **Prism first.** `vcorr` mapped the corner to the first apex vertex
  and declined the second (`Ok(None)`), so the join's own refusal
  (`NotSameFace`) stood.

A fillet-free pinch-and-prism pose (notch 30°–150°, wedge 60°–120°,
prism corner 10°–170°) builds through the normal join in both orders,
nested strut and all; only the join's refusal brings a pinch here.

## Direction

A correspondence that is a relation (as `zip::SeamCorrespondence`
already is, "one each, except a welded pinch"), carried into the patch
pairing and the glue, and a glue that keeps a pinch apex's vertices
apart; then the `PinchApex` refusal goes. The witness's assertion is
pinned to today's refusal and moves with the fix.

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.
