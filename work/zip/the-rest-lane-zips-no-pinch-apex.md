---
id: the-rest-lane-zips-no-pinch-apex
kind: issue
title: The REST lane reads its vertex correspondence one-to-one, so a pinch apex meeting one vertex refuses or declines
status: open
opened: 2026-10-05
priority: P3
cost: M
---

## What

`boolean/rest.rs` `try_rest_union` keys its vertex correspondence
`vcorr` by the A vertex and holds one B vertex per key; `a_of` and
`b_of` invert it, and `glue_pair`'s `SeamCorrespondence` holds one
correspondent each. A pinch apex — two vertices of one operand at one
point, as a union carried with its v-v rows leaves where two pieces
touch along an edge — met by ONE vertex of the other operand has no
one-to-one reading.

`crates/sweep/tests/rest_nested_strut.rs`
`a_nested_struts_segment_end_reads_as_the_vertex_it_fuses_into`
measures both orders (a notched block holding a wedge, touching along
the apex line, and a prism whose corner rests on the apex; the join
refuses on the fillets' tangency and hands the union to the lane):

- **Pinch first.** The two apex vertices each map to the prism's corner
  and the seam realizes on both operands. The glue then refuses
  `SeamOrientation` (the "kernel bug" variant): after the first patch
  pair's glue fuses the corner into one apex vertex, the second pair's
  glue meets that corner again (see also
  `work/zip/the-rest-lanes-glue-reads-its-correspondence-unfused.md`).
  Read through the glue's fusions instead, the second glue fuses the two
  apex vertices into one and refuses `Euler(SelfLoopEdge)`: the result
  wants the apex kept as the pinch it is, which neither zip models.
- **Prism first.** `vcorr` maps the corner to the first apex vertex and
  refuses the second (`correspond` → `Ok(None)`), so the join's own
  refusal (`NotSameFace`) stands.

A fillet-free pinch-and-prism pose (notch 30°–150°, wedge 60°–120°,
prism corner 10°–170°) builds through the normal join in both orders,
nested strut and all; only the join's refusal brings a pinch here.

## Direction

A correspondence that is a relation (as `zip::SeamCorrespondence`
already is, "one each, except a welded pinch"), carried into the patch
pairing and the glue, and a glue that keeps a pinch apex's vertices
apart. The witness's assertion is pinned to today's refusals and moves
with the fix.
