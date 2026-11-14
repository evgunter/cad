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

## Measured (2026-11-14), and moved to ONEHOME: it is live on main, and not the switch's

BOXFINISH's M3 lane; probe `crates/editor-core/tests/boxfinish_m3_probe.rs`
on `boxfinish/probe-switch-off` (commit `bba45062`, never merged; rows
`m3_turned_disc`, `m3_turned_plane_door`), on main `4c7404c2` (switch
on) and on the probe (switch off). Moved here by `git mv`, id unchanged:
this is fork 259's case (a), a later finish of a body moved by a boxed
motion, and the measurement says the box-scalar switch plays no part.

**What the text above gets wrong.** No document extrudes on a turned
sketch plane (`eval::wire` builds every extrude on `SketchPlane::xy()`
and carries a turned frame beside the body), so a sweeps-only document
never meets this check: with no taker it certifies in one leaf on both
trees. The check is met where a taker finishes a body a boxed turn has
moved, and main refuses there today.

**The document.** A disc (radius r, depth r), one copy spun about world
y by θ ± δ, united with a second copy standing clear.

- **Main, 30° ± 1°, r 5e-3:** the union's take refuses
  `UnfinishedLeaf { input: the turned copy, findings: [2× SurfaceDatumUndecided
  { kind: Cylinder, datum: Axis, measure: Length, end: Upper, margin:
  Enclosure { -7.4996e-5, 7.6140e-5 }, band: (1e-9, 1e-8) }, 4×
  CurveDatumUndecided { kind: Circle, datum: Axis, same margin }] }`,
  raised by `surface_datum_errors` / `curve_datum_errors`. A drive over
  the box at the shipped depth certifies 0 of 64 leaves, on both trees.
- **Stored:** axis `([-0.51504, -0.48481], 0, [0.85717, 0.87462])`,
  `u_ref` as wide, radius exact; each reads length `[0.9848, 1.0150]`.
  The finding names the axis because `analytic_datum_verdicts` keeps the
  first undecided margin per carrier.
- **Order in the box.** Length at 30°: r·sin 2θ·δ, first order (five
  widths). Length at 0°: r·δ²/2, second order. Tilt (`axis·u_ref`):
  2·r·δ, first order at both nominals, and it binds once the length
  decides. Check 1 first passes between 1e-5° and 1e-6°. It scales with
  r.
- **What stands behind it.** The document certified at no width tried.
  Past check 1: `VolumeUncomputable` on `props_cylinder_circle_off_axis`
  (`[0, 2.2e-8]` at 1e-6°); then, at 1e-8° and 1e-9°, every finish
  passes and the leaf refuses `FlipCrossing`,
  `PredicateDivergence { "pcurve_cylinder_chart_axial", old_count: 0,
  new_count: 32 }` at the union: the witness logs that predicate 0
  times and the box lane 32. On the copy whose centre also swings, the
  same finish adds 4× `SliverDihedral { "dihedral_sign" [-5.0e-3, 3.2e-2] }`
  and 2× `Pcurve { ResidualExceeded { EnvelopeTerm(FidelityU) } }`, a
  definite-sounding finding over a 1° box.
- **Not refused by check 1,** though as wide: the caps' normals and the
  seam lines' directors (no plane or line finding exists:
  `work/restdatum/unlevered-frame-conventions-are-uncertified-at-rest.md`).
- **Through the doors on a turned plane** (`sweep::extrude` on
  `SketchPlane::from_frame(RigidMotion::rotation(y, θ ± δ))`), the build
  refuses before check 1 at every width where check 1 would:
  `newell_plane_residual` at `Interval`, `carrier_endpoint_end` at
  `Sym<Interval>`, both first order in δ. The first is BOXFINISH's
  `boxfinish/sweep-frames`.

**The design question is unchanged,** with two things now known: the
reading is first order and blocks, and fixing check 1 alone does not
make the document certify. Fork 268's two designers (BOXFINISH, PR
6936) both leaned to a carrier storing its frame as the witness; fork
259 weighed carrying facts into a later finish
(`a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs`),
which would also keep check 1 from being asked again of a moved body.
Those are two answers to one question and should be weighed together.
