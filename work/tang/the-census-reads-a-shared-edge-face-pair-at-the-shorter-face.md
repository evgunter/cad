---
id: the-census-reads-a-shared-edge-face-pair-at-the-shorter-face
kind: issue
title: D1's census may read the wedge between two faces sharing an edge at the shorter face, where check 4 reads the farther
status: open
opened: 2026-10-16
priority: P3
cost: E
---

D1's census bullet (`docs/DESIGN.md`, "A census finding decides one number per pair of cells") reads two straight cells that share an edge at "their largest distance over the shorter cell … its length times the sine of the angle between them … with the cell's own extent as the arm". Since TANG's U-lever (PR 5204), tier 3's check 4 levers the same pair's wedge at the FARTHER face's reach (`topo::wedge_reach`).

If `topo::census` measures two faces that share an edge, a large face meeting a small one at a shallow angle could refuse in the census's sliver band (the small face's extent times the sine) and pass check 4 (the large face's). Those may be two different questions, not two readings of one: the census asks whether the cells meet anywhere else, which the small face settles; check 4 asks whether the edge is a corner. Not established either way. `crates/topo/src/census.rs` excludes each edge's two faces from some passes (`EdgeGeo::{f_plus, f_minus}`, "structural-adjacency exclusions"), but whether a face–face pass reads an edge-sharing pair was not traced.

To close: find whether any census pass reads an edge-sharing face pair. If one does, pin the large-face-beside-small-face pose and decide whether the two readings must agree. Found by the U-lever review (PR 5204, finding m6).

