---
id: a-moved-leaf-is-finished-again-at-the-box-scalar-and-the-tours-pin-refuses-at-export
kind: issue
title: A sweep leaf's move finishes its image again at the box scalar: with the gather carrying and the witness's premises repaired, the tour's certified command stops at a pin's export, refused SliverDihedral
status: open
opened: 2026-11-28
priority: P1
cost: H
---


Found by PR 7723's second review (2026-11-28), on a local probe tree:
the switch's box arm off (`boxfinish/probe-switch-off`), PR 7723's
head (the gather carries its members' certificates) and PR 7716 (the
witness's premises) merged in. Not reproduced by the orchestrator.

**The fact.** On that tree the chain's drive certifies its whole box
at every link count, which is what PR 7723's acceptance asked. The
command `demo-tour certified` then stops at `demos/tour/src/chaintol.rs`
(`certified_pin_boxes`): `pncad::export::body_in_space` of a pin at
`Sym<Interval>` refuses `SliverDihedral` at `dihedral_sign` (low end
−5.25e-5). The pin is a sweep leaf, and a leaf's move finishes the
image again from values (`LeafBody::moved_by` calls `T::leaf`).

**What it is.** The gather's defect one door along: a body that was
finished, moved by an isometry, derived again where DESIGN
`[one-way.places]` says a finished body moved by an isometry stays
finished and asks only the session box. PR 7723 closed it for a gather
of finished members; a moved LEAF is not finished until its taker
finishes it, so its image is re-derived, and over a box that turns it
the re-derivation does not decide.

It stands before the switch's deletion
(`delete-the-box-scalar-sweep-leaf-switch`): that row speaks of the
`f64` re-finish only. Whether a leaf moved before it is taken carries
anything (it has no certificate yet), or the export should take before
it moves, is to be read from the code before a unit is cut; the
`dihedral_sign` margin itself may also be one of
`a-margin-definite-at-every-member-of-a-box-reads-no-sign-where-its-value-reads-the-box-twice`'s.

**First step:** reproduce on the probe tree once PRs 7716 and 7723 are
on main; trace the refusing margin (its form, what carries the box),
and which doors move a leaf before a take.
(BOXFINISH orchestrator)
