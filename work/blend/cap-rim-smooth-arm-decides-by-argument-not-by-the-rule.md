---
id: cap-rim-smooth-arm-decides-by-argument-not-by-the-rule
kind: issue
title: sweep: the cap-rim smooth arm decides a description by an in-code argument, not by the must-carry rule
status: open
opened: 2026-09-13
---


## Finding

`geom_brep::must_carry_over_edge` is now the one home of the rule that
decides what description a definitely-smooth join carries, and the two
arms the item
`smooth-arm-siblings-disagree-on-the-in-band-case` named — extrude's
strut arm (`crates/sweep/src/extrude.rs`, the `DihedralClass::Smooth`
arm of the join pass) and revolve's latitude join
(`crates/sweep/src/revolve/upgrade.rs::upgrade_intersection`) — both
call it.

A THIRD smooth-description arm does not: `upgrade_rim`'s
(`crates/sweep/src/extrude.rs`, the `Ok(DihedralClass::Smooth)` arm,
~`:1225`) stores the conventional chart image unconditionally, and its
comment argues that the second-order rule has nothing to add there
because *"only plane pairs reach this arm (a cylinder wall's normal is
radial about the sketch normal, so it is perpendicular to the cap's ±n
at every rim point), and a plane pair's `κ_rel` is identically zero"*.

That is the shape the item objects to — an argument rather than a
shared spelling — and the argument's premise is imprecise on one door:
under `Extrusion::Vector` the wall cylinder is radial about the
extrusion vector `w`, not about the sketch normal `n`. The conclusion
still holds, but by a different reason: a smooth cap–wall pair needs
the cylinder's normal (⊥ `w`) parallel to the cap's `±n`, i.e.
`n · w = 0`, which the obliquity gate (`Extrusion::Vector`'s definite
normal component) already refuses. The comment states the reason that
holds only for the `Distance` door.

## Disposition

Not this unit: BLEND-9's spec scopes the change to the strut arm and
the latitude join, and constrains `dihedral.rs` to the wrapper. Two
separable pieces of work:

1. Route `upgrade_rim`'s smooth arm through `must_carry_over_edge` so
   the third arm reads the rule rather than restating a conclusion —
   which would also make the K-stream cost and the lane gate uniform
   across all three arms.
2. Failing that, restate the argument so it covers both extrusion
   doors (the obliquity gate, not the sketch normal).

Either way the claim wants a row: today nothing fails if the premise
stops holding.
