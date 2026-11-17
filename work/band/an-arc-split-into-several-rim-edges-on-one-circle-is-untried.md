---
id: an-arc-split-into-several-rim-edges-on-one-circle-is-untried
kind: issue
title: A rounded outline's arc split into several rim edges on one circle is untried under either verb
status: open
opened: 2026-11-17
priority: P3
cost: E
---

Found by the band-stripfru lane (PR 7050) while pinning the chamfer over rounded outlines.

A rounded outline whose quarter arc is drawn as 2, 3 or 4 arcs of one circle extrudes to one wall face and one rim edge per circle. So `band_closed_mixed_chain_shapes::rounded_outlines_drawn_in_arc_pieces_chamfer_whole` carves each quarter as one link and builds at Pappus. No door available to a test today leaves a circle's arc as several rim edges on several faces of one surface.

Such a chain would meet two links on one support surface at a valence-2 vertex inside a mixed chain. `junction::shape` refuses two consecutive links on one support pair (`JUNCTION_ON_ONE_PAIR`) under both verbs. The open door also refuses a joint inside a mixed chain. So the case would refuse today, typed, whichever verb asks. It is untried rather than wrong.

Building it needs a fixture that leaves the split, such as a boolean or a lattice-drawn profile that keeps its pieces, and then a carve that joins the pieces of one band across the split.
