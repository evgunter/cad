---
id: quadric-datums-unchecked-at-rest
kind: issue
title: Check 1 names no analytic surface whose stored frame or datum fails to describe a locus - they escalate elsewhere, by accident
status: open
opened: 2026-09-05
refs: [S330]
track: P
---

## What

Disclosed while placing the named no-op arms that replaced check 1's
`_ => {}` (`crates/topo/src/validate.rs:3212-3237`), and then EXECUTED
by S330's R2 review, which falsified the first draft of those arms'
stated reasons. Two claims were made and both were wrong:

- *"A plane's stored frame carries no datum that can fail to describe a
  locus."* It does — a zero normal, a poisoned normal, a poisoned
  origin.
- *"The quadric datums are conventional-and-unchecked and no tier-3
  check reads them."* They are read, through **evaluation**, and the
  reading escalates.

The arms' comments now say what actually happens. What remains open is
the behaviour itself.

## The measurement

One body per case: the tier-3-clean `coplanar_pillow`
(`crates/topo/src/tier3_tests.rs:38`), one face's surface swapped
through `Body::set_face_surface`, `validate_geometric` at
`Tol::witness()`. Every case additionally draws the two
`DescriptionNotAdjacent` findings the swap itself causes; those are
elided below. **No case draws any check-1 verdict, and no refusal names
the datum.**

| swapped surface | what tier 3 answered |
| --- | --- |
| `Plane { origin: (NaN,0,0) }` | `PlanarFaceEscalated` ×2 (`planar_face_residual`, margin `Invalid`), `PlanarBoundaryEscalated` ×2 (`planar_boundary_residual`) |
| `Plane { normal: (0,0,0) }` | `SliverDihedral` ×2 (`dihedral_wedge`, margin `Invalid`) |
| `Plane { normal: (NaN,NaN,NaN) }` | `PlanarFaceEscalated` ×2, `PlanarBoundaryEscalated` ×2, `SliverDihedral` ×2 |
| `Cylinder { radius: NaN }` | `SliverDihedral` ×2 (`dihedral_arm`, margin `Invalid`) |
| `Sphere { radius: 0.0 }` | `SliverDihedral` ×2 (`dihedral_arm`, margin `Invalid`) |
| `Cone { half_angle: NaN }` | `SliverDihedral` ×2 (`dihedral_wedge`, margin `Invalid`) |

So the body IS refused in every case — loudly, and with a margin that
reports `Invalid` rather than a number. But the refusal comes from
check 3 (planar residuals), check 4 (the dihedral wedge) and check 5
(planar-boundary containment), each of which was reading the surface
for a different question and got poison; none of them says *this
surface's stored datum does not describe a locus*. **That is exactly
the "answered by accident" shape S330 closed for `Nurbs` and left open
for every analytic kind.**

## The question

Two halves, and they may have different answers.

**The poison half.** A `Plane` with a poisoned normal and a
`NetState::Poisoned` NURBS net are the same fact about a body: a stored
surface that claims a locus and cannot evaluate one. S330 ruled that
the `Nurbs` case owes a named refusal at check 1, on `geom`'s
totality-and-poison rule (*"must reach each consumer's described arm
and fail there"*, `crates/geom/src/net.rs:128-131`). Nothing in that
rule is about NURBS. If it binds check 1 for one surface kind it binds
it for all of them, and the analytic arms owe a poison read.

**The convention half.** `Cylinder`'s `radius > 0`, `Sphere`'s
`radius > 0` and `Cone`'s `half_angle ∈ (0, π/2)` are
conventional-and-unchecked (`crates/geom/src/surfaces.rs:121`, `:198`,
`:163`), and `geom`'s conventional-and-unchecked rule
(`crates/geom/src/lib.rs:56-64`) promises that violating one yields
*"well-defined garbage … not poison and not a panic"*. A radius of `0`
is not poison; it is a convention violation, and tier 3 may be entitled
to take the convention at its word. The torus is the one analytic kind
that IS checked (`validate.rs:3054-3095`), and its stated reason is
representability rather than datum hygiene — a horn or spindle torus
has a chart singularity no chart in the tree represents. Whether that
argument stops at the torus is the open question.

`Sphere { radius: 0.0 }` sits on the seam between the two halves: not
poison, but not a small sphere either.

## What must NOT be done

Not a suite row pinning the escalations above. They are an accident of
which check happened to evaluate the surface first, so a row asserting
them would freeze the accident and red the moment check 1 grows the
honest arm. The measurement belongs here; the pin belongs to whichever
arm answers the question.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1). Reading, not editing,
`crates/geom/src/surfaces.rs` — changing the conventions themselves is
a `geom` question and a DESIGN.md one. The poison half likely wants a
`geom`-side door of the shape `NurbsSurface::net_state` now has
(`crates/geom/src/surfaces/nurbs.rs`), which is S-CERT's territory and
a seam to announce.
