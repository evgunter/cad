---
id: check-1-reads-a-turning-frames-length-off-its-bracket
kind: issue
title: Check 1 reads a stored frame's unit length off its bracket, so a frame that truly turns over the box reads non-unit at first order in the box
status: open
opened: 2026-11-14
priority: P1
cost: H
design: true
refs: [6936]
---


Raised by the design weighing of
`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`
(fork 268, `[ev]` PR 6936, where both designers set it apart as its own
question). No BOXFINISH row measured it; it is on record as fork 259's
case (a): a cylinder moved by a boxed motion refuses "axis not unit
length" at a later finish and finishes only at a box half-width of
1e-11.

**The case, by arithmetic.** A hole of radius 5 mm on a sketch plane
turned by θ = 30° ± 1°. The stored reference direction is
(cos θ, sin θ, 0) as two independent brackets, and since
cos²(θ − δ) + sin²(θ + δ) = 1 + sin 2θ · sin 2δ, its length reads
1 ± δ·sin 2θ: `[0.985, 1.015]`, first order in the box. Check 1's
margin (`topo::validate::surface_datum_errors` over
`Surface::representability_margins`, a bracket read that never goes
through the decision funnel) is then ±7.6e-5 m against ε = 1e-9, and
decides only below about 1e-5 of a degree. Every frame in the family is
exactly unit. However the direction is minted, the reading is the same.

**Why it bears on the switch.** With the switch on, a document of
sweeps alone on a turned sketch plane has tier 2 only; with the switch
deleted it meets this check (inferred, unmeasured).

**The design question** (both designers lean to the first):
- a carrier stores its frame as the witness (`UnitVec3`, `RigidMotion`),
  as `SketchPlane` does, and check 1's frame margins go; this changes
  `geom`'s at-rest paragraph and the "What is NOT this type" paragraphs
  from PR 5068, and a carrier's structural lift between scalars would
  carry the witness;
- check 1 asks through the decision funnel, so `Sym` discharges the
  identity where its rules reach;
- it stays, and such documents certify at tier 3 only near ε.

**Before it goes to Ev: one measurement.** On the probe branch, extrude
a circle on a sketch plane turned by 30° ± 1° and read check 1's root
margin and the width at which it decides. If it is first order, the
deletion of the switch regresses every variable-angle document with a
round feature, and this row joins the deletion row's blockers.
