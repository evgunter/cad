---
id: a-fillet-on-a-drawn-unions-operand-is-drawn-nowhere
kind: issue
title: A fillet on an operand of a drawn union (a tree pick) is inserted and drawn nowhere: the takeover re-points placements only
status: closed
opened: 2026-10-25
closed: 2026-11-02
priority: P1
cost: M
refs: [the-viewer-cannot-author-a-mirror]
---


Found by the mirror arrival fork's designer pair (2026-10-25; likely, by reading `add_blend`). The fillet gesture's takeover re-points only `placements_of_var(body)`. A body drawn only as an operand of a drawn union, picked in the tree, has no placements of its own, so the fillet is inserted and shows nowhere, and nothing says so. The same holds for any feature on one piece of a takeover's result, such as the mirror's `B ∪ m`.

**Shape of the fix.** The generalised takeover re-points the reader on the path the body was seen through: here the union's operand slot. After D′3 a pick's path (`VarDef::Pick`) spells that slot. Before then, the union's operand can be found by `world::seat_read`'s inverse; decide when this is dispatched.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.

## Re-parked on PR 6209 (2026-11-01)

D′3 (PR 5636) merged, so the old trigger fired. The row now waits on SPACES' `[ev]` PR 6209 (where a new body lands, and what the picture draws). Its recommended rule draws *bodies*, hiding a body that an operation reads as a body, and it hands the generalised takeover, re-pointing along the seen path, to SPACES (`a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`). Under that rule this row's defect may dissolve for an unplaced body and become SPACES' re-point for a placed copy. Re-read it against Ev's answer before building. (DOORS orchestrator)

## Closed

Re-read against Ev's ruling on PR 6209 and main at `822607621e` (2026-11-02). **Fixed on main for what the title says, and the remaining re-point is superseded by the ruling.**

- **Drawn nowhere: fixed by SPACES' `the-picture-draws-bodies` (PR 6265).** The picture now draws every body no operation reads as a body (`Picture::of`, `crates/viewer/src/world.rs:291`; the body-reading seats, `reads_as_body`, `:194`). A fillet reads the operand through its selection, so the operand leaves the picture, and nothing reads the fillet, so it is drawn in the operand's space, tinted as outside the product. Row: `picture_draws_bodies::a_fillet_of_a_construction_draws_the_fillet_alone` (`crates/viewer/tests/picture_draws_bodies.rs:133`); the rule is the same for an operand of a `∪`. PR 6265's body records this row as settled. The common case, a union of world copies, never stranded anything: a fillet picked on a copy is a fillet of the copied body and re-points that copy (`add_blend`, `crates/viewer/src/session.rs:3807`), so the union reading the copy follows.
- **Re-pointing the `∪`'s operand slot: superseded.** Under 6209 a `∪` is drawn as its result, not as its pieces; only a `⊔` is drawn as its pieces. So a `∪` is not a path a body is seen through, and an operand picked in the tree is seen, if at all, by its own toggle, in its own place. The ratified landing text lists the seen paths as "a placement's source, a `⊔` operand, the product slot" (`crates/viewer/README.md:2188`), and `Takeover::of_var` re-points exactly those (`session.rs:4795`, filtering on `Node::Assemble` at `:4803`; built by `a-new-body-lands-unplaced`, PR 6269, and its takeover row `lands_unplaced::a_takeover_re_points_a_union_operand_and_the_product_slot`, `crates/viewer/tests/lands_unplaced.rs:177`, whose "union" is a `⊔`).

One stale word remains for the orchestrator: `docs/INTENT-STAGE3-SPEC.md:493` (§5′.3 (a), written by D′3, PR 5636, and untouched by 6209) still says "a `⊔`/`∪` operand slot". The README's ratified list and the code agree on `⊔` alone. (DOORS re-read)
