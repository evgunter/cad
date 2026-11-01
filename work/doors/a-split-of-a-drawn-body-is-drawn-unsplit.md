---
id: a-split-of-a-drawn-body-is-drawn-unsplit
kind: issue
title: Splitting a drawn body leaves the picture showing the unsplit body: add_split takes nothing over, so its halves are reachable only in the tree
status: parked
opened: 2026-10-25
priority: P0
cost: M
blocked_on: [6209]
refs: [the-viewer-cannot-author-a-mirror]
---


Found by the mirror arrival fork's designer pair (2026-10-25; both confirmed it by reading the code, sure). `DocSession::add_split` (`crates/viewer/src/session.rs`) commits a bare `InsertNode`. A placed body's world copy keeps copying the unsplit body, so the picture shows no split, and the halves are reachable only in the tree. The mirror suite's split-half row places the lower half by hand for this reason.

**Shape of the fix (decided with the fork).** A split is a takeover with two results. The body's drawn copies are re-pointed onto `above ⊔ below` in its space, so they show both halves under the copy's own motion. `⊔` is D′3's operator (`Node::Assemble`, PR 5636). This is the generalised takeover: re-point whatever reads the picked body along the path it was seen through (a placement's source, a `⊔`/`∪` operand slot, or the product slot).

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.

## Re-parked on PR 6209 (2026-11-01)

D′3 (PR 5636) merged, so the old trigger fired. The row now waits on SPACES' `[ev]` PR 6209 (where a new body lands, and what the picture draws). Its recommended rule draws *bodies*, hiding a body that an operation reads as a body, and it hands the generalised takeover, re-pointing along the seen path, to SPACES (`a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`). Under that rule this row's defect may dissolve for an unplaced body and become SPACES' re-point for a placed copy. Re-read it against Ev's answer before building. (DOORS orchestrator)
