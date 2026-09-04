---
id: fillet-ruled-spine-arms-no-surgery
kind: unit
title: fillet - the ruled-spine arms classify but no surgery carves their band
status: open
opened: 2026-08-25
github: 987
refs: [962]
needs_ev: true
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

## For Ev — what terminates a ruled band (the OQ6 question this unit waits on)

The arms exist (`BlendArm::CylinderCylinderCylinder`,
`BlendArm::CylinderPlaneCylinder`: a `Surface::Cylinder` about a
straight spine, both trimlines lines along the ruling). The surgery does
not: a ruled pair meets along an OPEN edge, and the open-chain door
(`AdmittedOpen::admit`, `crates/sweep/src/blend/admit.rs:97`) takes
plane–plane supports ending in fully-requested trivalent corners only.
A ruling on a finite cylinder ends at the cylinder's caps — transverse
faces whose rim edges the caller has NOT requested — so the termination
is a run-out against an unrequested transverse face, the "general
run-outs are not implemented" clause of `FILLET3_CORNER_RECOURSE`.
ARMS3 A3-3 (`crates/sweep/README.md`) names the MID-CURVE run-out
(ball-cap stop; feather-out) and reserves the taxonomy for you. This
is a different termination; the question is whether it needs the same
conversation before it is built.

1. **The transverse cut-off, for caps perpendicular to the ruling
   (recommended).** Where the terminating face is a plane perpendicular
   to the spine, the band's end is that plane's section of the cylinder
   band: an exact circular arc, stored, no new surface kind. The cap
   face's loop gains the arc (the old corner vertex splits into the two
   feet); the two supports' trimlines end at the feet. Anything else at
   an end — an oblique cap, a curved face, a chart seam — refuses typed
   with its own sentence. This completes the flat-milled-rod and
   parallel-cylinders shapes the arms were built for. The unit's spec
   would propose the `CornerConfig` / `RunOutPolicy` name for it for
   your ratification, as `SeamVertex` was.
2. **Hold H7 until the whole run-out taxonomy is designed** — ball-cap,
   feather-out and the transverse cut-off as one family. Consistent,
   larger, and consumer-gated: no corpus shape asks for a ruled band.
3. **Cut H7 from the program**: the arms stay classified-but-unbuilt
   with their honest refusal, recorded as residue at the exit walk.

A 👍 on 1 unblocks the spec; 2 or 3 parks the item with its trigger
named.

**Ruled (Ev, PR 1736, 2026-09-04): option 1.** The ruled band's
termination is the transverse cut-off at a cap perpendicular to the
ruling — the cap plane's section of the cylinder band, an exact stored
arc; every other end refuses typed. Its `CornerConfig` / `RunOutPolicy`
name is proposed in the unit's spec for ratification, as `SeamVertex`
was. The mid-curve run-out taxonomy (A3-3) stays reserved and is not
touched by this. This item is now the unit that builds it (H7, last in
order); the spec follows.

**Spec for ratification (2026-09-04):** `docs/FILLET-H7-SPEC.md` proposes
`CornerConfig::TransverseCap` and `RunOutPolicy::CutOffAtTransverseCap`
for the cut-off Ev chose on PR 1736; a 👍 on this PR ratifies the names
and the unit dispatches (block FILLET-B2). `needs_ev` set for that
ratification only.
