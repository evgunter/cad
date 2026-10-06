---
id: two-section-loft-with-an-inverted-top-normal-builds
kind: issue
title: A two-section loft whose top section's plane normal points DOWN (against the stacking) builds and tier 3 says Ok — nothing checks the last section's normal against the stacking direction
status: dispatched
opened: 2026-09-16
refs: [2752]
priority: P0
cost: M
branch: carve/fold-reads-the-far-normal
---

Found by BOOL-6's R2 review (PR 2752), pre-existing at the merge base,
and filed by the S-BOOL orchestrator on sweep ground
(`crates/sweep/src/loft.rs`). The stacking fold reads each slab's
displacement against section k−1's normal only; the top cap is
oriented from the LAST section's own normal (`places.last().c2`),
which nothing checks against the stacking direction. A two-section
loft whose top plane normal points down while the section sits above
the base builds, and `validate_geometric` returns Ok. The Orientation
paragraph in `loft.rs` now documents the top cap as reading that
unchecked normal. The fix is one structural decide at the fold's last
slab (the last normal against the slab's displacement, under
`loft_stacking`'s band) refusing typed, or a stated reason the
inverted top normal is a legal authoring. Measured, not acted on;
difficulty S.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to BLEND (the sweep crate and the profile fillet door are BLEND's charter; crates/sweep/src/loft.rs passes to BLEND at this exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Weighed (2026-10-06)

Weighed with `self-overlapping-spines-build-and-validate` (its
"Weighed" section has the whole answer). The inverted top normal is a
fold that the canonical traversal causes: each ring winds about its own
plane's normal, so a plane facing back along the stack gives a ring
wound against its neighbour's. The fold checked only the near normal.
The interim fix, which the designers converged on, also decides the far
section's normal against each slab: branch
`carve/fold-reads-the-far-normal`. It closes this row. The certificate
that later retires the fold refuses the same body as a fold.
