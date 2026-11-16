---
id: a-fused-tools-minted-cells-are-spelled-by-arena-order-and-unnamed
kind: issue
title: A cut by a fused tool spells a seam beside a merged face by its least tool face key, and names no vertex the union minted where outlines cross
status: open
opened: 2026-11-12
priority: P3
cost: E
---


Filed by SEAL's orchestrator from the delta review of PR 6860, 2026-11-12. Neither point was measured; each is read from code.

## What

1. **A seam's two faces are picked by key order.** `Through::of` spells a seam the union minted as the meeting of the faces on its two sides (`seam_faces`, `crates/topo/src/boolean/material.rs:331-341`). Each side is `first_held`, the least `FaceKey` among the tool faces that the material face holds.
   - On a face merged from several flush tool faces, all of them lie on one plane, so the meeting line lies on the chosen face's carrier. But the chosen face may not reach the seam.
   - The pick follows arena order, which NS1 forbids a name to read.
   - It needs three or more solids in one fused group, and no fixture builds one yet. The sibling row `a-row-on-a-merged-face-of-a-cuts-material-names-one-tool-face` has the same key-order pick for a merged face's division.
2. **Crossing vertices have no naming lineage.** In `big − cross` (the delta review's crossing-outline fixture), the two vertices the union minted where the tool's outlines cross are grafted under the material's keys, with no naming row. `crossings` re-spells only coincidence rows. Whether the editor names such vertices from their edges is untested.

## Done when

- A three-solid fused group with a seam beside a merged face is built, and the seam is spelled by a face that reaches it, chosen by no arena order.
- An editor row names a crossing vertex of a fused cut, stably across a reorder of the tool's pieces.
