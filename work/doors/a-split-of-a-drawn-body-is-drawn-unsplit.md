---
id: a-split-of-a-drawn-body-is-drawn-unsplit
kind: issue
title: Splitting a drawn body leaves the picture showing the unsplit body: add_split takes nothing over, so its halves are reachable only in the tree
status: dispatched
opened: 2026-10-25
priority: P1
cost: M
refs: [the-viewer-cannot-author-a-mirror]
branch: doors/split-takes-over
---


Found by the mirror arrival fork's designer pair (2026-10-25; both confirmed it by reading the code, sure). `DocSession::add_split` (`crates/viewer/src/session.rs`) commits a bare `InsertNode`. A placed body's world copy keeps copying the unsplit body, so the picture shows no split, and the halves are reachable only in the tree. The mirror suite's split-half row places the lower half by hand for this reason.

**Shape of the fix (decided with the fork).** A split is a takeover with two results. The body's drawn copies are re-pointed onto `above ⊔ below` in its space, so they show both halves under the copy's own motion. `⊔` is D′3's operator (`Node::Assemble`, PR 5636). This is the generalised takeover: re-point whatever reads the picked body along the path it was seen through (a placement's source, a `⊔`/`∪` operand slot, or the product slot).

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.

## Re-parked on PR 6209 (2026-11-01)

D′3 (PR 5636) merged, so the old trigger fired. The row now waits on SPACES' `[ev]` PR 6209 (where a new body lands, and what the picture draws). Its recommended rule draws *bodies*, hiding a body that an operation reads as a body, and it hands the generalised takeover, re-pointing along the seen path, to SPACES (`a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`). Under that rule this row's defect may dissolve for an unplaced body and become SPACES' re-point for a placed copy. Re-read it against Ev's answer before building. (DOORS orchestrator)

## After 6209 (2026-11-02)

Re-read against Ev's ruling on PR 6209 and main at `822607621e` (SPACES' PRs 6265, 6269, 6279, 6270 and DOORS' 6264 merged). **Still live, narrowed to a body a person has placed.**

- **Settled for an unplaced body.** The split reads its target as a body (`reads_as_body`, `crates/viewer/src/world.rs:194`), so the target leaves the picture and the halves are drawn in its place (`Picture::of`, `world.rs:291`). Row: `lands_unplaced::a_split_and_a_pattern_of_a_fresh_box_leave_every_drawn_body_pickable` (`crates/viewer/tests/lands_unplaced.rs:69`). Since a new body lands unplaced, this is the default path.
- **Still wrong for a placed body.** `DocSession::add_split` (`crates/viewer/src/session.rs:3506`) still inserts a bare `Node::Split` with no takeover, and `SessionOp::AddSplit`'s doc says so (`crates/viewer/src/session/op.rs:742`: "the target's placements keep reading the target"). The split tool seats the body a picked copy places (`crates/viewer/src/combine.rs:332`). So for a copy that Add to the product (identity bundle), a transform or a duplicate made, the world copy keeps copying the unsplit body and stays drawn, and the split's halves are drawn as well, in the body's own space at the identity. For an identity placement the halves sit exactly on the unsplit copy, and a pick there answers `Ambiguous`. PR 6269's body says the same: "A person-placed world copy still draws unsplit, since the split writes no takeover."

**The fix.** `add_split` takes the target over as `add_blend` (`session.rs:3807`) and `add_pattern` (`:3640`) already do: `self.takeover(&target)?` and `taking_over`, with the result a `⊔` over the two halves (`Node::Assemble` reading `above` and `below`). A placement's source, a `⊔` operand and the product slot each read one body (§5′.3: `Place { source: BodyRead }`), so the `⊔` is what every reader is re-pointed to (`Takeover::repoint`, `session.rs:4907`); the guard `Takeover::stranded_pick` (`:4843`) and 6264's rebinding (which already walks a split chain) apply unchanged. Re-word `op.rs:742`'s doc. Rows owed: a split of a world copy draws both halves under the copy's motion and the unsplit copy is gone; a split of a body the product reads re-points the slot; one undo takes the split and the re-point back.

**Design.** None needed: a faithful elaboration. `docs/INTENT-STAGE3-SPEC.md` §5′.3 (a) has a gesture that takes a body over re-point what reads it along the seen path, the viewer README (*What the picture shows*) counts a split's input as intermediate beside a fillet's and a mirror's, and the `above ⊔ below` shape was decided with the mirror arrival fork (above). **Owner:** DOORS; no SPACES or PLACING row carries it (SPACES' `a-new-body-lands-placed-in-the-world`, which named the split's picture, closed with PR 6269 on the unplaced case). **Priority** lowered P0 → P1: the default path (split a fresh body) now works; what is broken is splitting a body after it was placed or added to the product. (DOORS re-read)
