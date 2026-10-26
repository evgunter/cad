---
id: the-mirror-keeps-two-pieces-and-mirrors-across-another-spaces-face-after-dprime3
kind: issue
title: After D′3 the mirror tool gains its two-pieces output (B ⊔ image) and a world-level mirror across another space's face
status: parked
opened: 2026-10-25
priority: P1
cost: M
blocked_on: [5636]
refs: [the-viewer-cannot-author-a-mirror]
---


The residue of the mirror arrival fork (2026-10-25, designer pair, converged; recorded in `the-viewer-cannot-author-a-mirror`'s `## Closed` once PR 5683 lands). Before D′3 the mirror tool makes only the fused symmetric part: `B ∪ Mirror{B, plane}`, taken over by B's drawn copies. Two outputs wait on D′3's operators and product slot:

- **Two pieces.** `B ⊔ Mirror{B, plane}`, taken over the same way. The image stays its own piece (a pick through the `⊔`'s path), and the seam is recorded as a touch rather than glued. It becomes the tool's second output beside the fused one, as the pattern tool offers Fused beside Instances.
- **A mirror across another space's face**, such as a bracket mirrored across a chassis's centre plane. This is a world-level creation, `Mirror { body: C, plane: Plane { face: wall.f } }`, where C is the clicked copy. The gesture adds it to the product's `⊔` (D′3's creation rule: a gesture writes `SetProduct`). Today it refuses `SpaceMismatch`, which is right until then. Both designers lean to building it, and neither is sure.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
