---
id: a-split-or-pattern-gesture-leaves-its-target-placed
kind: issue
title: A split or pattern gesture leaves its target placed, so a projected half or copy overlaps it; residue 1's re-point has no one body to re-point to
status: closed
opened: 2026-10-08
closed: 2026-10-08
---


Found by INTENT stage 2 unit C's viewer lane (branch `intent/s2-c-world`);
the question is with the INTENT orchestrator.

Residue 1 of #4220: a feature gesture (combine, fillet, …) re-points the
target's world placement to the result in the same action
(`feature_over` and `combine_in_world` in `crates/viewer/src/session.rs`).
A split's result is two `Body` ports and a pattern's one `Bodies`, so
there is no one body to re-point to. As built, Split and Pattern leave
the world untouched (the target stays placed) and AddPart, a creation,
places the projection it makes, so a projected half or copy overlaps
its still-placed target until the user removes the target's placement.
Pinned by `combine_ops::a_pattern_places_nothing_and_a_projection_places_its_copy`.

The alternative: split re-points the target's placement to port 0 and
adds a port-1 placement at the same pose; pattern re-points to
`Part{0}` and places `Part{1..n}`. Its costs: an empty half refuses the
whole product, and a later count edit leaves the placed picks stale
(placements do not follow a count).

A related call the lane made: deleting a placed feature cascades its
placement, so the copy leaves the world with no reverse re-point onto
the feature's input (`story_authoring`'s rook row loses the rook when
the merlon block is deleted, restated to expect it).

## Closed

Ruled by the INTENT orchestrator (2026-10-08): the literal reading
stands. Split and Pattern leave the world untouched, and AddPart places
only what the author asks. The alternative picks members by position
(`Part{0}`, `Part{1..n}`, stale when a count changes), copies one
placement's pose into another (relating two placements by equal
values), and re-points silently, against Ev's 2026-10-03 principles. A
projected half overlapping its still-placed target is a loud at-rest
finding, and the author resolves it by removing the target's
placement. Patterns are being redesigned as index variables (`[ev]`
#4341), so no pattern-placement gesture is built beyond C's.
