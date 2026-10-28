---
id: a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt
kind: issue
title: A PlacedUnion of a ball whose master is tilted by an ulp against exact copy frames fails the CDT at 1e-4
status: open
opened: 2026-10-20
priority: P3
cost: M
---

Found by INTENT stage 3 D's exact-turn lane (`work/intent/transform-retires-into-a-placement.md`), which measured it and then moved its own corpus off it; nothing in the suite reaches it now.

**What was measured.** At `ccf433d0e4` the corpus document `die_tool` (`crates/editor-core/tests/corpus/die_tool.rs`) placed its master ball with `place_against`, whose spins come from `atan2` and so leave the master's pole an ulp off the cube's `z` (a cosine of `6.1e-17` where `0` is meant). Its six `PlacedUnion` frames had just become exact (a quarter turn now has cosine `0` exactly), so the master and its copies disagree by that ulp. The viewer's display budget then refused it at a display tolerance of `1e-4`: `tessellate: the CDT rejected a point insertion on a face — a non-finite or out-of-range chart coordinate, i.e. corrupt geometry`, raised as `TessellateError::Triangulation { face: FaceKey(14v3) }`. The face is a sphere face of the subtracted cavity. The rows that showed it were `viewer::all display_budget::*` ("die_tool fits at 0.0001") and `index_memo::every_parametric_corpus_document_indexes_the_same_through_the_seam_across_edits`.

The same document with both the master and the frames inexact, or both exact (`5cf76d715e`, master on `corner_frame` plus an axis mate), tessellates. So the mesher refuses a sphere face whose chart is tilted by an ulp against the plane cutting it, though the geometry is valid.

**Where.** `crates/mesh/src/curved.rs:295-297` (the `insert` closure) and `:329-331` (`can_add_constraint` refused); the message cannot tell these apart, so the first step is to find which one fires. To reproduce, check out `die_tool.rs` at `ccf433d0e4` on top of the current tree.

**Done when** a sphere face whose pole is tilted by an ulp off its cutting plane's normal tessellates, or refuses with a typed reason that names that cause and not "corrupt geometry"; add that row.
