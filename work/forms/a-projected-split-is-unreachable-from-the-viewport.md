---
id: a-projected-split-is-unreachable-from-the-viewport
kind: issue
title: After one projection of a split or pattern, its other bodies are reachable only from the tree: a viewport click seats the projection, not the source
status: open
opened: 2026-09-24
priority: P1
cost: D
---



## Finding

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
