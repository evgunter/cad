---
id: a-projected-split-is-unreachable-from-the-viewport
kind: issue
title: After one projection of a pattern, its other copies are reachable only from the tree: a viewport click seats the projection, not the source
status: closed
opened: 2026-09-24
priority: P1
cost: D
closed: 2026-11-22
refs: [the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family, the-projection-tool-writes-a-member-read-a-pick-already-seats]
---



## Finding

**Narrowed to the pattern case** (PR 4618, `part-split-half-retires`):
a split's halves are its outputs, read by port, and no `Part` projects
one, so a viewport click on a placed half seats that half's port. What
follows is the finding as filed; the pattern case stands as written.

Found by AUTH-4's re-review (AUTHOR, PR 3052), filed by AUTH-4's lane
under implementer-discipline §6. A capability AUTH-4 added and then
narrowed within the same unit, not a regression against `main`.

**The shape.** Split a box and project the lower half
(`SessionOp::AddPart`, `PartSelectSpec::SplitHalf(Below)`). The split
leaves `Doc::roots` — a projection consumes its input — so the upper
half stops being drawn and the projection is the only body left of it
on screen. Clicking that body's cut face gives the projection tool
`Selection::seat_node()` = the PROJECTION (a `Node::Part`), which the
seat routing drops into the split seat, and the commit refuses "node N
is not a split". The same holds for a pattern after one instance is
projected.

**Why the pick route is not the fix.** `Selection::seat_node`
(`crates/viewer/src/session/select.rs`) is the one answer every seated
tool's pick takes — the node whose DRAWN body the ray met — and it is
right for every other seat: a body seat fed the feature that minted a
face authors against geometry the user did not click. Special-casing
the projection tool's pick to walk back to the source would be a second
answer to that question.

**What the chrome says today.** The projection panel's standing
sentence (`pane::create::PROJECTION_HIDES_THE_REST`) says the other
bodies left the picture and names the FEATURE TREE as where to pick the
split or pattern again. That is true, and it is the only route.

## Shapes worth weighing (not decided here)

- **Seat the source through the projection.** A `Node::Part` names its
  source (`of`), so a seat wanting a split or a pattern could be filled
  from a pick on a projection of one. That is a statement about what a
  PICK of a projection means to a multi-body seat — a routing rule in
  `crate::seats`, beside the existing kind routing — rather than a
  change to what `seat_node` answers, and it would serve any future
  multi-body seat too.
- **Keep the source drawn.** Draw the unselected bodies of a projected
  split or pattern as ghosts, pickable, so the source stays in the
  picture. A display rule, and a bigger one.

**Measured** (AUTH-4 lane): box split at half height, lower half projected, ray straight down at the centre → the pick seats node 5 (the projection) in the split seat, and `half_op` commits to the refusal "node 5 is not a split in this document".

## Closed (2026-11-22): the premise is gone

Two designers weighed the row independently, and both found that the
document model it describes has been retired. There is no `Node::Part`
and no pattern node. A pattern is a family over an index
(`[repetition]`), and the projection tool writes a placement of one
member read, `xs[k]`, that consumes nothing. `world::Picture::of` draws
the family less the members read alone (`Covers::AllBut`), so after
one copy is projected, every other copy is still drawn, and a viewport
pick on one of them seats the pattern in the projection tool
(`lands_unplaced::a_projected_member_is_drawn_once`). The panel
sentence the row cites is gone; `PROJECTION_IS_PLACED` now states the
opposite. A click on the projected copy itself seats the copy, which is
Ev's ruling on PR 6353. So neither of the row's two shapes has anything
left to fix.

What the row was evidence of is tracked elsewhere. The split, mirror
and pattern seats read a picked member as the whole family
(`work/doors/the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family.md`),
and once they read the member, the projection tool is a detour that
can go (`work/chrome/the-projection-tool-writes-a-member-read-a-pick-already-seats.md`).
The DOORS row should land first. The part tool's prompt still says
"pick a pattern in the tree" (`pane/create.rs`, `ToolKind::Part.says`),
which undersells the viewport pick; it goes with the tool's deletion
on CHROME's row.
