---
id: fillet-ruled-spine-arms-no-surgery
kind: issue
title: fillet - the ruled-spine arms classify but no surgery carves their band
status: open
opened: 2026-08-25
github: 987
refs: [962]
---

## From GitHub issue 987

Opened 2026-08-25; 0 comments.

**Raised by VERBS-ARMS-2** (PR #962), which landed the two shared-ruling
arms as part of the ratified cut and could not carve them.

## The gap

`BlendArm::CylinderCylinderCylinder` and `BlendArm::CylinderPlaneCylinder`
are implemented and exact: two supports that share a ruling direction
reduce to a cross-section normal to that ruling, each cuts a line or a
circle there, and the ball centre is the crossing of the two offset
traces — the same `blend::sheet_center` the coaxial arms use. The blend
is a `Surface::Cylinder` about a straight spine, and both trimlines are
lines along the ruling. `verbs_arms2_arms.rs` pins both arms at their
closed forms in both material configurations.

What does not exist is the SURGERY. A ruled pair meets along an OPEN
edge (a common ruling of two parallel cylinders; a flat milled on a
rod), so the request lands on `ConvexOpen::admit`, which requires
plane–plane supports:

> an open chain's supports are not plane–plane (the trivalent corner
> patch is the only termination built)

So the arms classify, pass every predicate, and then refuse at the
open-chain door. The refusal is honest and names what is missing.

## What is actually needed

The open-chain carve is written for straight trimlines on PLANAR
supports: the strip `mef`s, the strut `mev`s and the corner patch all
assume the support is a plane. A ruled band's trimlines are still LINES
— that part transfers — but its supports are curved, so:

- the trimline edges must carry curved-support descriptions
  (`TangentIntersection` on a cylinder, which is already in the
  line arm of `geom_brep::tangent_certificate_lane`);
- the chain TERMINATIONS are the real work: a ruling on a finite
  cylinder ends where the cylinder's own caps or seams are, and that is
  the run-out taxonomy OQ6 reserves.

## Sequencing

Behind ARMS-3, which owns the corner/run-out door (OQ6's reserved design
question) — the terminations are the same question. Consumer-gated: no
shape in the corpus asks for it yet.

## Home

Fillet band and surgery were S-BLEND's, which is closed and may hold only closed items; VERBS ceded that ground explicitly, so this open residue lands under `work/issues/`.
