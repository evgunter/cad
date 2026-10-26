---
id: a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in
kind: issue
title: blend: a blind bore or a pin inside a ruled cut-off's region refuses as a bound, its far faces having no exact core to be read in
status: open
opened: 2026-10-14
priority: P3
cost: M
---

From `a-concave-fill-and-a-whole-shell-inside-a-bands-region`, step 3.
A cup on a cut-off's cap (a blind bore under a convex band, a pin under
a concave one) goes with the plane–plane band, whose reach has an exact
core: the cup's faces that do not meet the cap (its floor or end) and
the edges they bound are read inside that core
(`reach::curve_in_core`, `reach::inside_bound`), and the faces that do
meet it are read through the mouth and those edges
(`reach::prism_inside`). The ruled band's reach (a cylinder support) has
no exact core (`straight_reach` keeps `core` only when `exact`; its
bounds enclose the material rather than being it), so the far edges are
never certified and the cup's wall reads as a bound:
`band_one_cycle_members::a_pin_inside_a_concave_ruled_creases_fill_refuses_as_a_bound`.

Building it wants an inside reading of the ruled band's material: the
cap meter's exact sliver (its arc, chord and rim pieces) swept along the
spine, i.e. a core whose bounds are the material exactly on the cylinder
side too (the cylinder support's own half-space by its signed distance,
which is 1-Lipschitz), or an inside-only core that a point certified in
is certainly in the material.
