---
id: a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product
kind: issue
title: The viewer's takeover re-points only placement sources, and no creation gesture adds its result to the product ⊔ (spec §5′.3 (a), (b))
status: parked
opened: 2026-10-26
priority: P1
cost: M
blocked_on: [6209]
---

Filed by D′3 (PR 5636) on the INTENT orchestrator's 2026-10-26 ruling. `docs/INTENT-STAGE3-SPEC.md` §5′.3 ("The viewer's contract after D′3") states two sentences that this row builds, and AUTHTAIL's mirror arrival fork rests on them (`work/intent/log.md`, 2026-10-25):

- **(a) The takeover re-points along the seen path.** It re-points whatever reads the picked body along the path it was seen through: a placement's source, a `⊔`/`∪` operand slot, or the product slot. Today `repoint` (`crates/viewer/src/session.rs:3802`) re-points placement sources only.
- **(b) Creation gestures add their result to the product `⊔`** (a `SetProduct` from the gesture). Today no gesture sets the product: `create_placed` (`:3289`) writes the world placement alone (`place_in_world`, `:4404`), and the properties pane's Add/Take-out row (`viewer::world::product_with`/`product_without`, `crates/viewer/src/world.rs:307`; applied by `set_product`, `session.rs:3522`) is its only writer. `the-viewer-sets-no-product` closed on "nothing sets it unasked".

(a) generalises the residue-1 gesture rule (INTENT-STAGE2, fork row 85) beyond placements. D′3 adds one guard on the takeover: a takeover that would strand a pick refuses, `Refusal::StrandedPick` (`stranded_pick`, `session.rs:3767`). A re-pointed `⊔` operand slot keeps the guard's question, whether the new source still holds the piece a pick names.

**Tests owed:**
- a fillet of a body read by the product `⊔` re-points the `⊔`'s operand;
- a body read directly by the product slot is re-pointed in the slot;
- each creation gesture leaves its copy in the product, and one undo takes the gesture and the product's change back together;
- the viewer rows that set the product by hand (`crates/viewer/tests/common/mod.rs:807`, `product_is`) are re-read.

## Parked on PR 6209 (2026-11-01)

SPACES' `[ev]` PR 6209 retires (b): its designers recommend that nothing joins the product unasked, and they put "Add to the product" behind one door. It makes (a) load-bearing for the pattern takeover and that door, and says SPACES claims it once Ev answers. Parked until then. On Ev's answer, (b) closes as superseded or comes back, and (a) moves to `work/spaces/` by `git mv`. The P0 `a-mate-on-a-taken-over-copys-face-goes-stale` does not wait for this. It builds the rebind that follows any re-point, and so does not depend on which readers a re-point reaches. (DOORS orchestrator)
