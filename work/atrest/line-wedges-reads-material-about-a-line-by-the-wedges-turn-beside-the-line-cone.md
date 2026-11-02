---
id: line-wedges-reads-material-about-a-line-by-the-wedges-turn-beside-the-line-cone
kind: issue
title: census::line_wedges reads two coincident edges' material about their line by each wedge's turn (validate::wedge_holds_named), beside the line cone's parity
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Filed by ONEREAD's `recl-edge-edge-composes-the-other-solids-material-beside-the-cone`
fix pass (branch `oneread/recl-line-cone`, PR 6282), whose first sweep's
pattern (`reflex`, `convex`, half-space and/or accumulation) could not
match "half turn" or `wedge_holds`.

## What

`crates/topo/src/census.rs` `line_wedges` (called by the result gate,
`boolean::ops`, and tier 3′, `census` `ee_overlap_sides`) reads each of
two coincident edges' material wedges about their shared line and asks
whether one's sheet or interior direction lies strictly inside the
other's: through `crate::validate::wedge_holds_named` (`LINE_WEDGE`),
tier 3's pinch-corner reading, which decides a wedge's turn under or
over half a turn (`turn`, `straight`) and the direction's side of each
bound. That is material about a line read by a wedge's convexity and
two bound sides, where the boolean's edge-edge site now reads a solid's
material about a line by the line cone's crossing parity
(`boolean::sectors::line_fans`, `link::Cone::reads_tied`): the same
fact (which directions about the line one solid's material holds), a
second reader. `splitting::rules::edge_wedge` (convex or reflex by
`enters_material`) is the split's instance, filed as
`work/oneread/the-split-rule-b-reads-an-in-plane-edges-material-by-convexity.md`.

Not a defect: `wedge_holds_named` is one decided reader with its own K
rows, and the overlap it asks (one wedge's sheet strictly inside the
other) is a two-cone question the line cone does not yet answer.

## Owed

Weigh reading each edge's wedge as a two-sheet line cone and asking
the overlap of the cone (a sheet's direction `In`, or `On` the other's
sheet), so material about a line has one reader; or say why the pinch
corner's turn reading is the right one here (it is tier 3's own
question at a pinch vertex).
