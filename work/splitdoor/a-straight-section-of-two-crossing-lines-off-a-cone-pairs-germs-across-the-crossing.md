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
review: the join fix is scoped by special case.

## What

`boolean::join::partners` reads a straight germ section's facing by the
chord test (`facing`, `bool_join_facing`). Where that section is two lines
meeting at a point, a site on the other line, ahead of the germ across
the crossing, faces it by the chord test alone, and the join pairs them
(the 30° tipped cone against a tilted plane through its apex, which
desynced the join before the fix). The fix adds an on-line test
(`bool_join_on_line`: the partner's site on the germ's own line) and
gates it on "a face of the germ's pair carries a cone"
(`join::is_cone`). The test is not applied to every straight section:
`near_tangent_pierce_poses` breaks under it, its germs near a tangency
being read only to the band, so their partners are not on their line to
the band.

The trigger is the section's shape, not the cone. Any carrier pair
whose straight section is two lines through a point meets the same
mismatch outside the gate. The witness to build: a saddle, the
hyperbolic paraboloid `z = xy` as a bilinear spline patch, cut by its
tangent plane `z = 0` at the saddle point, whose section is the lines
`x = 0` and `y = 0`. Unmeasured: whether the spline lane reaches
`facing`'s straight arm at all, or refuses first.

Unit row: `boolean::join::tests::facing_on_rulings_pairs_only_along_the_germs_own_line`.

## Where to look

Component identity decided where the germ is minted
(`join::germ_branch`, the one door every germ's `Branch` goes through):
a straight section of two lines would carry the line's index as a
conic's loop does (`Branch::Component`), and `facing` would pair germs
only on one component. A germ whose site is the crossing point itself
lies on both lines, so the index is read off its direction there, not
its site, which `germ_branch` does not take today. With that in place
the kind gate retires.
