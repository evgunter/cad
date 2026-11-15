---
id: the-volume-readings-interval-re-derivation-re-asks-a-closed-forms-premises-on-the-value-channel
kind: issue
title: The volume reading's Interval re-derivation re-asks a closed form's premises on the value channel, so a cylinder wall the body's own scalar proves on its rim refuses props_rim_fit over a box
status: open
opened: 2026-11-15
priority: P0
cost: M
---


Found by the signed-radius lane's measure-first (2026-11-15,
`boxfinish/signed-radius`, the doors' arc half with the circle verb
storing `r` and the readers' re-posings applied; the tour's plate with
its holes subtracted, over the tour's study box, `Sym<Interval>`, one
leaf). It is what holds the cut plate once
`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`
and
`a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root`
are in.

**Measured.** The first `Subtract` takes its hole operand finished, and
tier 3's volume role walk refuses it:
`UnfinishedLeaf { NotFinished [VolumeUncomputable { Face 3v1, Escalated
{ predicate "props_rim_fit", [-6.0e-5, 6.0e-5] } }] }`, raised in
`geom-brep` `props/curved.rs` `cylinder_boundary`. By backtrace,
`topo::props`' closed form for the `Sym<Interval>` body calls
`closed_form_of::<Interval>`: the closed form is run a second time at
plain `Interval` over the geometry lifted, and there the premise
`r_c − radius` (the rim circle's radius less the cylinder's) is `r − r`
by dependency, as wide as the box on `r`. At `Sym` the same premise has
both operands on one node and the tier proves it, 8 of 8.

**Where it is in the code** (read): `closed_form_of`'s doc says "the
face walk runs it at the walk's scalar, and `QuadLane`'s `closed_form`
at the interval scalar over the same geometry lifted". So a curved
face's premises (`props_rim_fit`, `props_meridian_on_surface`,
`props_circle_axis_class`) are decided twice: once at the body's
scalar, where an identity is an identity, and once on the value
channel, where it is a width. The second asking can only lose: it
refuses what the first proved.

This is M6's finding ("the symbolic channel is not read") on a curved
face, at a premise rather than in the reading's width
(`check-7s-volume-reading-is-loose-over-a-box` holds the width).

**Not known:** whether the lifted run needs its premises at all once
the walk's scalar decided them (what the `Interval` re-derivation is
for, and what it certifies that the walk's run does not); whether a
premise decided at `Sym` may be carried to the lifted run as decided,
and what that asks of the lift (`Interval::from_certified`); which
other premises behave the same on a cone, sphere and torus wall.

**The tests that say it is closed.**
- The tour's plate with its holes subtracted, over the study box, no
  longer refuses at its first `Subtract` on `props_rim_fit` (measured
  on the tree that has the two rows above). What it does next is
  reported by payload and, if something else holds it, filed.
- A cylinder-walled body whose radius carries a box wider than the
  band finishes its volume role walk at `Sym<Interval>`, and the volume
  it reads contains every member's.
- At `f64` and plain `Interval` nothing moves, or each move is listed.
