---
id: the-teapot-spout-lofts-four-arc-circles-for-a-crease-the-mesher-now-meshes
kind: issue
title: The teapot's spout authors each round section as circle_split(4), for a crease refusal the mesher no longer makes; whether a plain circle section lofts, meshes and exports there is unmeasured
status: open
opened: 2026-11-20
priority: P3
cost: M
---



Found by `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment.

## What

`demos/tour/src/teapot.rs` `SPOUT_ARCS` authors each of the spout's seven round sections as `LoopProgram::CircleSplit { n: 4 }`. Its doc gave the reason as the C⁰ crease a lofted `Circle` wall carries at its sub-arc joints, which the tessellator refused (`lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`, TESS). That row closed on 2026-10-20: the crease meshes. So the four-arc split no longer answers a refusal the tour would otherwise meet, and the scene is not written the way a user would write it (`docs/prompts/implementer-discipline.md` §3: "the natural spelling of the task through the public doors").

Moving the spout to `circle` (one segment per section, one wall) is not a comment change. It moves the scene's frames and volumes, and it puts a seven-section one-segment loft through paths with live rows: LATHE's `the-skin-cuts-an-arc-wall-at-each-sub-arc-joint` (P0) and EXPORT's `one-segment-loft-at-eight-sections-fails-step-reimport` (the tour exports STEP for every scene).

## What closing it takes

Author the spout's sections with `circle`, measure what the scene does (build, mesh, enclosure, STEP round trip), and either keep it, re-rendered, or restore the split with the refusal it then meets named in `SPOUT_ARCS`'s doc. `crates/pncad-py/tests/test_north_star.py` `TestTeapot` mirrors the count and its prose.
