---
id: a-straight-section-of-two-crossing-lines-off-a-cone-pairs-germs-across-the-crossing
kind: issue
title: The join's on-line test is gated on a cone face, so a straight section of two crossing lines on any other carrier can pair germs across the crossing
status: dispatched
branch: splitdoor/section-line-components
opened: 2026-11-01
priority: P1
cost: M
---

Filed by SPLITDOOR on `splitdoor/cone-apex-boolean` (PR 6266), from its
review: the join fix was scoped by special case.

## What

`boolean::join::partners` reads a straight germ section's facing by the
chord test (`facing`, `bool_join_facing`). Where that section is two
lines meeting at a point, a site on the other line, ahead of the germ
across the crossing, faces it by the chord test alone, and the join
paired them (the 30° tipped cone against a tilted plane through its
apex). PR 6266 added an on-line test (`bool_join_on_line`) gated on "a
face of the germ's pair carries a cone" (`join::is_cone`); applied to
every straight section it broke `near_tangent_pierce_poses`, whose
plane × plane germs near a tangency are read only to the band.

## Resolution

The rule is the section's shape, not the surface kind. A germ on a
section of two lines through a point carries the line it rides,
decided where it is minted (`join::germ_branch` → `join::apex_line`,
`Branch::Line(k)`): off the crossing by the line its site lies on, at
it by the line its direction runs along. `facing` parts two germs whose
branches differ, as it parts a conic's loops. `is_cone`, the gate and
`bool_join_on_line` are gone. A plane × plane section is one line and
mints no line id, so `near_tangent_pierce_poses` is untouched.

The mechanism is general; its only reachable instance is plane × cone
through the apex. Two smooth carriers meet in two crossing lines only
where they are tangent, a C7 construction no lane builds, so a crossing
off a tangency needs a singular carrier point, and the cone's apex is
the one the kernel has; a cone against a cone through one apex is a
vertex pair, refused `VertexPairAtConeApex`. The saddle witness (a
bilinear spline wall cut by its tangent plane) refuses at the spline ×
plane pair before the join, pinned in
`crates/sweep/tests/a_saddle_cut_by_its_tangent_plane.rs` and filed as
`spliceface/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form`.

Unit row: `boolean::join::tests::facing_on_crossing_lines_pairs_only_along_the_germs_own_line`.
