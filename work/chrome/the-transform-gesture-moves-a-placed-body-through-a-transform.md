---
id: the-transform-gesture-moves-a-placed-body-through-a-transform
kind: issue
title: The viewer's Transform gesture moves a placed body through a Transform; under D10 moving a placed body edits its placement
status: open
opened: 2026-10-09
---


Found by the second review of INTENT stage 2 C (PR #4359, MINOR-D);
kept as built on the orchestrator's ruling, a known interim gap.

`add_transform` (`crates/viewer/src/session.rs`) inserts a `Transform`
of the target and re-points the target's world placements to it
(`feature_over`, residue 1's feature-gesture rule), so a placed body is
moved by a `Transform` under its placement, and its world pose has two
homes: the placement's pose and the `Transform`. Before C, A10's tip
transfer had the same visible effect.

Under D10, moving a placed body edits its placement (Ev, 2026-10-03:
"you can't Transform an already placed part"); that edit arrives with
stage 3's placements (`a-placement-is-the-bundle-of-mates`), and
`Transform` as an operand-maker retires with `[ev]` #4326 ("nothing
moves a body").

Why not changed in C: there is no door that edits a placement's pose in
place, and the gesture's result is read as an operand (the combine
suite's moved second block), which a placement's copy may not be
(`ReadsWorldCopy`). Refusing the gesture on a placed body turned 24
viewer rows red, among them `story_parametric::the_parametric_living_walk`,
and would leave the GUI no way to make a moved body.
