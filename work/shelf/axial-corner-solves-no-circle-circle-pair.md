---
id: axial-corner-solves-no-circle-circle-pair
kind: issue
title: the axial corner solve has no circle-circle pair, so a sphere or torus tangent to another circle profile has no tangency arm
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-of-a-tangent-dome-refuses-at-the-axial-corner]
---


`topo::offset_axial`'s profile solve has no circle–circle pair:
`transversality` answers `None` for one ("a form the corpus has no
fixture for, so it is not written"), `roots` gives none, and the pair
is skipped. A corner where only two circle profiles meet — a sphere and
a torus, or two tori — has no pair
to solve, and refuses with the no-resolvable-pair text.

Its TANGENT case has no arm either. The line–circle tangency fallback
(`tangent_foot`, and `branch`'s side of the foot on a tie) was written
for line–circle pairs only, so a sphere tangent to a torus at a rim, or
a torus tangent to a sphere, has neither the transversal solve nor the
tangent one. That is the blind spot of the sweep that closed
`shell-of-a-tangent-dome-refuses-at-the-axial-corner`.

The item: a door-built body with a circle–circle corner (the C5 table
or the teapot corpus may hold one), measured; then the pair's
transversal solve (the radical line against either circle) and its
tangency arm (the contact point on the line of centres — compare
`work/issues/decided-tangent-point-is-the-radical-foot.md` for where
that point should sit when the tangency is decided inside the band).
