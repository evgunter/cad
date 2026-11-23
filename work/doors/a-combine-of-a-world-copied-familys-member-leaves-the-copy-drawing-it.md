---
id: a-combine-of-a-world-copied-familys-member-leaves-the-copy-drawing-it
kind: issue
title: A boolean cannot read one member of a family something copies, because no takeover says what a member read does to the copy
status: open
opened: 2026-11-09
priority: P1
cost: M
design: true
---


Found by the REFS unit `viewer-boolean-tool-cannot-pick-a-read` (PR 6593) and its fix pass.

**What the GUI cannot offer.** The boolean tool's seats read a viewport pick on a drawn member of a family as that member (`pattern[k]`, `seats::Port::at_seat`, `world::seat_member`), and a tree pick as the family whole with a row choosing one member. **Member reads stay off for a family something reads along a seen path** (`world::is_seen`: a copy of it, a `⊔` operand, the product slot). There the seat reads the family whole, as it did before member reads, and the combine takes the copy over. The row that pinned this was re-spelled by PR 6581, whose pattern of a placed body writes no placement. It is now `combine_ops::a_member_picked_on_a_family_drawn_as_itself_reads_that_member`, pinning the member read of a family drawn as itself. The case here is now reached by hand alone: a world copy of a family. So for a family copied into the world, the person cannot union, intersect or subtract one copy of the pattern. That is GUI breadth the tool cannot offer until this is ruled.

**Why it waits.** The takeover vocabulary re-points readers of a variable (`Takeover::of_var`, `crates/viewer/src/session.rs`), and the copy reads the family whole. Probed on the PR before the fix pass, with member reads through the copy:
- **Re-pointing nothing** (what `combine_in_world` does for a member read): the world copy still covers every member (`Covers::AllBut({})`), so member 1 stays drawn in the world, and the union is drawn in the family's own space, not the world. The world's product volume stayed the two members.
- **The sibling** (probed): `Union([pattern[0], C])` through the API, the family copied and `C` world-copied. `combine_in_world` takes `C`'s copy over onto the union. The world then draws the family copy (both members) and `C`'s copy (now `member 0 ∪ C`), so member 0 is drawn twice: the product's volume was 3·A for two members and a `C` inside member 0. The tool cannot author this, since it reads the copied family whole, but the session door takes it.

**Decide when dispatched:** what a combine of one member does to a copy of its family. Options:
- Re-point the copy to a `⊔` of the other members and the result.
- Read the copy's own member (`copy[i]`, in the world) instead of the source's, which runs against the seat rule as it stands. Note that README's *A new body lands unplaced* now says a pick on a copy seats that copy (PR 6580), so `work/placing/` may decide this half.
- Something else.

**Done when** the ruling lands, `Port::at_seat`'s `is_seen` condition goes, and the pinned row becomes a member read whose picture draws the result once and every other member where it stood.

**A copy of one piece (a pick) is the same hazard, narrower** (OFFER, PR 7356). A placement `Y` whose source picks piece `k` of a placement `X` copies no body of `X` (`Doc::placements_of`, `Doc::placements_of_var`), so `world::is_seen(X)` stays false and a seat may read one member of `X`. A member read of `k` itself then leaves `Y` drawing `k` beside the result, as a whole copy would; members `j ≠ k` are not drawn by `Y`. Row: `lands_unplaced::a_copy_of_one_piece_reads_the_placement_and_copies_none_of_it`. The ruling this row waits for should say what a member read does to a pick of that member too.
