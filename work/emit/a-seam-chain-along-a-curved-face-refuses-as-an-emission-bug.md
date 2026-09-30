---
id: a-seam-chain-along-a-curved-face-refuses-as-an-emission-bug
kind: issue
title: A pair boolean whose seam chain runs along a curved face refuses face_plane's emission bug, a missing rule
status: open
opened: 2026-09-30
---


## What

A cylinder of radius 0.3 lying along y across a plate's top (the plate
`[0,3]²×[0,1]`, the cylinder's axis at x = 1.5, z = 1.0, from y = 4.0
for 5.0) unions to a sound body, and the pair boolean's naming refuses
`Naming(Emission { what: "face_plane: non-planar carrier in planar
pipeline" })`. The union of the two refuses the same in both member
orders (`crates/editor-core/tests/emit_union_borders.rs`,
`a_curved_divider_answers_as_the_pair_boolean_does`).

The face pieces are not the cause: `Borders` reads no plane. The
refusal comes from the seam-chain ranker, which orients a chain along
`n_a × n_b` over the two faces' planes (`emit_topo::name_boolean_edges`,
the chain's first pair; and `emit_topo::seam_line_dir`), and one side
here is the cylinder. It is a legal recipe, so an `Emission` (a kernel
bug report) is the wrong category: it is a missing rule, like the
union's own `SplitReference` for the same ranking. Which direction a
curved seam's pieces are ranked along is the open question.
